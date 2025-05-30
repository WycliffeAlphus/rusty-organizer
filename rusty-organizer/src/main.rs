use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::{
    fs,
    error::Error,
    path::{Path, PathBuf},
}