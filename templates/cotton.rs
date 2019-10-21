#!/usr/bin/env denim

//
// Example script description
//

/* Cargo.toml
[package]
name = "{name}"
version = "0.1.0"
authors = ["Anonymous"]
edition = "2018"

[dependencies]
cotton = "0.0.5"
structopt = "0.3.2"
*/

use cotton::prelude::*;

// See: https://docs.rs/structopt/0.3.2/structopt/index.html#how-to-derivestructopt

/// Does stuff
#[derive(Debug, StructOpt)]
struct Cli {{
    #[structopt(flatten)]
    logging: LoggingOpt,

    #[structopt(flatten)]
    dry_run: DryRunOpt,
}}

fn main() -> FinalResult {{
    let args = Cli::from_args();
    init_logger(&args.logging, vec![module_path!()]);

    warn!("Hello world!");

    Ok(())
}}
