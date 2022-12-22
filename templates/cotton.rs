#!/usr/bin/env -S denim
/* Cargo.toml
[package]
name = "{name}"
version = "0.1.0"
authors = ["Anonymous"]
edition = "2021"

[dependencies]
cotton = "0.1.0"
*/
use cotton::prelude::*;

/// Example script description
#[derive(Parser)]
struct Cli {{
    #[command(flatten)]
    logging: ArgsLogger,

    #[command(flatten)]
    dry_run: ArgsDryRun,
}}

fn main() -> FinalResult {{
    let Cli {{
        logging,
        dry_run,
    }} = Cli::parse();
    setup_logger(logging, vec![module_path!()]);

    if !dry_run.enabled {{
        warn!("Hello world!");
    }}

    Ok(())
}}

// vim: ft=rust
