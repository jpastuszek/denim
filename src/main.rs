use cotton::prelude::*;
use cotton::problem;
use std::os::unix::fs::PermissionsExt;

mod project;
use project::{Project, CargoMode};

const MODE_USER_EXEC: u32 = 0o100;

#[derive(Debug, StructOpt)]
enum ScriptAction {
    /// Create new scipt from template
    New {
        /// Create bare minimum template
        #[structopt(short, long)]
        bare: bool,

        /// Don't pre-build the script
        #[structopt(short, long)]
        no_prebuild: bool,

        /// Path to script file
        script: PathBuf,
    },
    /// Run `cargo check`
    Check {
        /// Path to script file
        script: PathBuf,
    },
    /// Build and stage for fast execution
    Build {
        /// Path to script file
        script: PathBuf,
    },
    /// Build, stage for fast execution and execute
    Exec {
        /// Path to script file
        script: PathBuf,

        /// Arguments for the script
        arguments: Vec<String>, //TODO: OsString not supported
    },
    /// Build and run tests
    Test {
        /// Path to script file
        script: PathBuf,
    },
    /// Remove all cached build files related to scipt file
    Clean {
        /// Path to script file
        script: PathBuf,
    },
    /// Remove all cached build files
    CleanAll,
}

/// Single file rust scritps.
#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(flatten)]
    logging: LoggingOpt,

    #[structopt(subcommand)]
    script_action: ScriptAction,
}

fn write_template<'i>(script: &Path, template: String) -> Result<()> {
    fs::write(script, &template).problem_while("writing template to new scipt file")?;

    let file = File::open(script).unwrap();
    let meta = file.metadata().unwrap();
    let mut perm = meta.permissions();
    perm.set_mode(perm.mode() | MODE_USER_EXEC);
    drop(file);

    fs::set_permissions(script, perm).problem_while("setting executable permission")?;

    Ok(())
}

fn main() -> FinalResult {
    if let Some(script) = std::env::args().skip(1).next().filter(|arg1| PathBuf::from(arg1).is_file()) {
        problem::format_panic_to_stderr();
        let project = Project::new(PathBuf::from(script))?;

        if !project.has_binary() {
            project.cargo()?.ensure_built(CargoMode::Silent)?;
        }

        project.execute(&std::env::args().skip(2).collect::<Vec<_>>()).unwrap();
        unreachable!()
    }

    let args = Cli::from_args();
    init_logger(&args.logging, vec![module_path!()]);

    match args.script_action {
        ScriptAction::New { bare, no_prebuild, script } => {
            let project_name = script.file_stem().ok_or_problem("Path has no file name")?.to_str().ok_or_problem("Script stem is not UTF-8 compatible")?;
            info!("Generating new sciprt {:?} in {}", project_name, script.display());

            if bare {
                write_template(&script, format!(include_str!("../templates/bare.rs"), name = project_name))?;
            } else {
                write_template(&script, format!(include_str!("../templates/cotton.rs"), name = project_name))?;
            }

            if !no_prebuild {
                let project = Project::new(script)?;
                let cargo = project.cargo()?;
                cargo.ensure_built(CargoMode::Verbose)?;
            }
        }
        ScriptAction::Exec { script, arguments } => {
            let project = Project::new(script)?;
            let cargo = project.cargo()?;
            cargo.ensure_built(CargoMode::Verbose)?;
            project.execute(&arguments)?;
        }
        ScriptAction::Build { script } => {
            let project = Project::new(script)?;
            let cargo = project.cargo()?;
            cargo.ensure_built(CargoMode::Verbose)?;
        }
        ScriptAction::Check { script } => {
            let project = Project::new(script)?;
            let cargo = project.cargo()?;
            cargo.ensure_updated()?;
            cargo.check()?;
        }
        ScriptAction::Test { script } => {
            let project = Project::new(script)?;
            let cargo = project.cargo()?;
            cargo.ensure_updated()?;
            cargo.test()?;
        }
        ScriptAction::Clean { script } => {
            let project = Project::new(script)?;
            project.clean()?;
        }
        ScriptAction::CleanAll => {
            Project::clean_all()?;
        }
    }

    Ok(())
}
