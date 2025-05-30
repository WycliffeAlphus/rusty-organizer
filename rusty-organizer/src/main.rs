use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::{
    fs,
    error::Error,
    path::{Path, PathBuf},
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]

enum OrganizeMode {
    Extension,
    Type,
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Rusty Organizer - A simple CLI tool to sort files by type/extension",
    long_about = None
)]

struct Args {

    #[arg(short, long, default_value_t = OrganizeMode::Type)]

    mode: OrganizeMode,

    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::pars();
    let source_dir = Path::new(&args.source);

    if !source_dir.exists() {
        resturn Err(format!("Source directory '{}' doesn't exist", args.source).into())
    }
}
