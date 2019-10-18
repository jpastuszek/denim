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
    pub fn new(script: PathBuf) -> Result<Project> {
        let script = script.canonicalize().problem_while_with(|| format!("accessing script file path {:?}", script.display()))?;
        info!("Script path: {}", script.display());

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

        let name = script.file_stem().unwrap().to_str().ok_or_problem("Script stem is not UTF-8 compatible")?.to_owned();
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
    pub fn cargo(&self) -> Result<Cargo> {
        Cargo::new(self)
    }

    pub(self) fn binary_path(&self) -> PathBuf {
        self.home.join(&self.name)
    }

    /// Returns true if execute has binary to run.
    pub fn has_binary(&self) -> bool {
        self.binary_path().is_file()
    }

    /// Replace this image with imange of the binary.
    pub fn execute<I>(&self, args: I) -> Result<()> where I: IntoIterator, I::Item: AsRef<OsStr> {
        // TODO: replace return with ! when stable
        Err(Problem::from_error(exec(self.binary_path(), args)).problem_while("executing compiled binary"))
    }

    pub fn clean(&self) -> Result<()> {
        info!("Removing content of {}", self.home.display());
        fs::remove_dir_all(&self.home)?;
        Ok(())
    }

    pub fn clean_all() -> Result<()> {
        let project_root = app_cache(None)?;

        info!("Removing content of {}", project_root.display());
        fs::remove_dir_all(&project_root)?;
        Ok(())
    }
}
