use cotton::prelude::*;
use super::Project;
use serde_json::Value;

#[derive(Debug, Clone, Copy)]
pub enum CargoMode {
    Silent,
    Verbose,
}

#[derive(Debug, Clone, Copy)]
pub enum CargoState {
    ScriptDiffers,
    NoBinary,
    BinaryOutdated,
    UpToDate,
}

impl CargoState {
    pub fn needs_update(&self) -> bool {
        match self {
            CargoState::ScriptDiffers => true,
            _ => false
        }
    }

    pub fn needs_build(&self) -> bool {
        match self {
            CargoState::ScriptDiffers => true,
            CargoState::NoBinary | CargoState::BinaryOutdated => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct Cargo<'p> {
    project: &'p Project,
}

impl<'p> Cargo<'p> {
    pub fn new(project: &Project) -> PResult<Cargo> {
        if !project.home.join("src").exists() {
            info!("Initializing cargo project in {}", project.home.display());
            collect_errors(run_result!(
                "cargo", "init",
                "--quiet",
                "--vcs", "none",
                "--name", &project.name,
                "--bin", &project.home
                ).problem_while("running cargo init")?)
            .problem_while("initializing cargo project")?;
        }

        Ok(Cargo {
            project,
        })
    }

    /// Runs cargo build with JSON output to get executable path.
    ///
    /// Assuming we already have built the crate and this will actually just get output of build in
    /// JSON.
    fn built_executable_path(&self) -> PResult<PathBuf> {
        // ask cargo for information about target binary file name as it depends on Cargo.toml
        // project name
        let out = collect_output_and_errors(run_result!(
                CurrentDir(&self.project.home),
                "cargo", "build", "--message-format=json", "--release"
            ).problem_while("running cargo build")?
        ).problem_while("getting build metadata")?;

        let mut lines = out.lines().collect::<Vec<_>>();

        // NOTE: executable will be probably logged last
        lines.reverse();

        let executable = lines.into_iter()
            .map(|line| serde_json::from_str(line))
            .or_failed_to("parse cargo JSON message")
            .find_map(|line: Value| line.get("executable").and_then(Value::as_str).map(ToOwned::to_owned))
            .ok_or_problem("Failed to find executable path in cargo JSON output")?;

        Ok(executable.into())
    }

    fn main_path(&self) -> PathBuf {
        self.project.home.join("src").join("main.rs")
    }

    fn manifest_path(&self) -> PathBuf {
        self.project.home.join("Cargo.toml")
    }

    fn script_content(&self) -> PResult<String> {
        // TODO: read up to _DATA_ marker and provide File object seeked at first byte after it
        Ok(read_to_string(&self.project.script).problem_while("reading script contents")?)
    }

    fn manifest_content(&self) -> PResult<String> {
        let manifest = self.script_content()?
            .lines()
            .map(|l| l.trim())
            .skip_while(|l| *l != "/* Cargo.toml")
            .skip(1)
            .take_while(|l| *l != "*/")
            .join("\n");

        if manifest.is_empty() {
            Err(Problem::from_error("Cargo.toml manifest not found in the script"))
        } else {
            Ok(manifest)
        }
    }

    /// Checks state of the repository and script.
    pub fn state(&self) -> PResult<CargoState> {
        if hex_digest(Some(self.script_content()?.as_str())) != hex_digest_file(&self.main_path())? {
            return Ok(CargoState::ScriptDiffers)
        }

        let binary_path = self.project.binary_path();

        if !binary_path.is_file() {
            return Ok(CargoState::NoBinary)
        }

        // binary should be newer than the script file or we have a failed build of the script
        if metadata(&binary_path)?.modified()? < metadata(&self.project.script)?.modified()? {
            return Ok(CargoState::BinaryOutdated)
        }

        Ok(CargoState::UpToDate)
    }

    /// Updates repository from the script file.
    pub fn update(&self) -> PResult<()> {
        info!("Updating project");

        write(&self.main_path(), self.script_content()?).problem_while("writing new main.rs file")?;
        write(&self.manifest_path(), self.manifest_content()?).problem_while("writing new Cargo.toml file")?;

        Ok(())
    }

    /// Builds cargo project.
    pub fn build(&self, mode: CargoMode) -> PResult<()> {
        info!("Building release target");
        match mode {
            CargoMode::Silent => collect_output_and_errors(
                run_result!(CurrentDir(&self.project.home), "cargo", "build", "--release")
                    .problem_while("running cargo build")?)
                    .map(drop),
            CargoMode::Verbose => check_status(
                run_result!(CurrentDir(&self.project.home), "cargo", "build", "--color", "always", "--release")
                    .problem_while("running cargo build")?),
        }
        .problem_while("building cargo project")?;

        rename(self.built_executable_path()?, self.project.binary_path()).problem_while("moving compiled target final location")?;

        Ok(())
    }

    /// Prepares executable
    pub fn ensure_updated(&self) -> PResult<()> {
        let state = self.state()?;
        if state.needs_update() {
            self.update()?;
        }
        Ok(())
    }

    /// Prepares executable
    pub fn ensure_built(&self, mode: CargoMode) -> PResult<()> {
        let state = self.state()?;
        debug!("State: {:?}", state);
        if state.needs_update() {
            self.update()?;
        }
        if state.needs_build() {
            self.build(mode)?;
        }
        Ok(())
    }

    /// Runs 'cargo check' on updated repository
    pub fn check(&self) -> PResult<()> {
        self.update()?;
        check_status(run_result!(
            CurrentDir(&self.project.home),
            "cargo", "check", "--color", "always")
                .problem_while("running cargo check")?)
            .problem_while("running cargo check")?;
        Ok(())
    }

    /// Runs 'cargo test' on updated repository
    pub fn test(&self) -> PResult<()> {
        self.update()?;
        check_status(run_result!(
            CurrentDir(&self.project.home),
            "cargo", "test", "--color", "always")
                .problem_while("running cargo test")?)
            .problem_while("running cargo test")?;
        Ok(())
    }
}
