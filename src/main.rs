use cotton::prelude::*;
use cotton::problem;
use std::os::unix::fs::PermissionsExt;

mod cargo;
use cargo::{CargoMode, Cargo};

const MODE_USER_EXEC: u32 = 0o100;

#[derive(Debug, StructOpt)]
enum ScriptAction {
    /// Create new scipt from template
    New {
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

fn write_template<'i>(script: &Path, project_name: &str) -> Result<()> {
    let template = format!(include_str!("../template.rs"), name = project_name);

    fs::write(script, &template).problem_while("writing template to new scipt file")?;

    let file = File::open(script).unwrap();
    let meta = file.metadata().unwrap();
    let mut perm = meta.permissions();
    perm.set_mode(perm.mode() | MODE_USER_EXEC);
    drop(file);

    fs::set_permissions(script, perm).problem_while("setting executable permission")?;

    Ok(())
}

fn main() -> Result<()> {
    if let Some(script) = std::env::args().skip(1).next().and_then(|arg1| arg1.ends_with(".rs").as_some(arg1)) {
        problem::format_panic_to_stderr();

        let cargo = Cargo::new(PathBuf::from(script)).or_failed_to("initialize cargo project");

        if !cargo.binary_built() {
            cargo.ensure_built(CargoMode::Silent).or_failed_to("build script");
        }

        cargo.execute(std::env::args().skip(2)).unwrap();

        unreachable!()
    }

    let args = Cli::from_args();
    init_logger(&args.logging, vec![module_path!()]);

    match args.script_action {
        ScriptAction::New { script } => {
            let project_name = script.file_stem().unwrap().to_str().ok_or_problem("Script stem is not UTF-8 compatible")?;
            info!("Generating new sciprt {:?} in {}", project_name, script.display());

            write_template(&script, project_name).or_failed_to("write script template");
        }
        ScriptAction::Exec { script, arguments } => {
            let cargo = Cargo::new(script).or_failed_to("initialize cargo project");
            cargo.ensure_built(CargoMode::Verbose).or_failed_to("update_and_build script binary");
            cargo.execute(arguments).unwrap();
        }
        ScriptAction::Build { script } => {
            let cargo = Cargo::new(script).or_failed_to("initialize cargo project");
            cargo.ensure_built(CargoMode::Verbose).or_failed_to("build script binary");
        }
        ScriptAction::Check { script } => {
            let cargo = Cargo::new(script).or_failed_to("initialize cargo project");
            cargo.ensure_updated().or_failed_to("update cargo project");
            cargo.check().or_failed_to("check script");
        }
        ScriptAction::Test { script } => {
            let cargo = Cargo::new(script).or_failed_to("initialize cargo project");
            cargo.ensure_updated().or_failed_to("update cargo project");
            cargo.test().or_failed_to("test script");
        }
        ScriptAction::Clean { script } => {
            let cargo = Cargo::new(script).or_failed_to("initialize cargo project");
            cargo.clean().or_failed_to("clean script repository");
        }
        ScriptAction::CleanAll => {
            Cargo::clean_all().or_failed_to("clean script repository");
        }
    }
    Ok(())
}
