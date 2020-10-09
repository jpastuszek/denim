use cotton::prelude::*;

mod cargo;
pub use cargo::{CargoMode, Cargo};

#[derive(Debug)]
pub struct Project {
    pub(self) name: String,
    pub(self) home: PathBuf,
    pub(self) script: PathBuf,
}

impl Project {
    pub fn new(script: PathBuf) -> PResult<Project> {
        let script = script.canonicalize().problem_while_with(|| format!("accessing script file path {:?}", script.display()))?;
        debug!("Script path: {}", script.display());

        if !script.is_file() {
            return Err(Problem::from_error(format!("Script {:?} is not a file", script.display())))
        }

        let parent_path = script
            .parent()
            .map(|p| p.to_str().ok_or_problem("Script parent path is not UTF-8 compatible"))
            .transpose()?
            .unwrap_or("./");

        let parent_path_digest = hex_digest(Some(parent_path))[0..16].to_string();
        debug!("Parent path: {} (digest: {})", parent_path, parent_path_digest);

        let name = script.file_stem().ok_or_problem("Scipt path has no file stem")?.to_str().ok_or_problem("Script stem is not UTF-8 compatible")?.to_owned();
        debug!("Project name: {}", name);

        let home = app_cache(format!("project-{}-{}", parent_path_digest, name).as_str())?;
        debug!("Project home: {}", home.display());

        Ok(Project {
            name,
            script,
            home,
        })
    }

    /// Convert to Cargo repository
    pub fn cargo(&self) -> PResult<Cargo> {
        Cargo::new(self)
    }

    pub fn binary_path(&self) -> PathBuf {
        self.home.join(&self.name)
    }

    /// Returns true if execute has binary to run.
    pub fn has_binary(&self) -> bool {
        self.binary_path().is_file()
    }

    /// Replace this image with imange of the binary.
    pub fn execute<I>(&self, arguments: &[I]) -> PResult<Infallible> where I: AsRef<OsStr> {
        exec_with_name(&self.binary_path(), &self.name, arguments).problem_while("executing compiled binary")
    }

    pub fn clean(&self) -> PResult<()> {
        info!("Removing content of {}", self.home.display());
        remove_dir_all(&self.home)?;
        Ok(())
    }

    pub fn clean_all() -> PResult<()> {
        let project_root = app_cache(None)?;

        info!("Removing content of {}", project_root.display());
        remove_dir_all(&project_root)?;
        Ok(())
    }
}
