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
