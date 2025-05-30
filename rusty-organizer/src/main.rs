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