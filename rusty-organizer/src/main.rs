use clap::{Parser, ValueEnum};
use rayon::prelude::*;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OrganizeMode {
    /// Organize by file extension
    Extension,
    /// Organize by file type (documents, images, etc.)
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
    /// Source directory to organize
    #[arg(short, long, default_value = ".")]
    source: String,

    /// Organization mode
    #[arg(value_enum, short, long, default_value_t = OrganizeMode::Type)]
    mode: OrganizeMode,

    /// Dry run - don't actually move files
    #[arg(short, long, default_value_t = false)]
    dry_run: bool,

    /// Verbose output
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let args = Args::parse();
    let source_dir = Path::new(&args.source);

    if !source_dir.exists() {
        return Err(format!("Source directory '{}' doesn't exist", args.source).into());
    }

    if args.verbose {
        println!("Organizing files in: {}", source_dir.display());
        println!("Mode: {:?}", args.mode);
        println!("Dry run: {}", args.dry_run);
    }

    let files = collect_files(source_dir)?;

    if args.verbose {
        println!("Found {} files to organize", files.len());
    }

    files.par_iter().try_for_each(|file| {
        let dest_dir = match args.mode {
            OrganizeMode::Extension => get_destination_by_extension(file),
            OrganizeMode::Type => get_destination_by_type(file),
        };

        let new_path = source_dir.join(&dest_dir).join(file.file_name().unwrap());

        if args.verbose {
            println!(
                "Moving {} to {}",
                file.file_name().unwrap().to_string_lossy(),
                new_path.display()
            );
        }

        if !args.dry_run {
            fs::create_dir_all(new_path.parent().unwrap())?;
            fs::rename(file, &new_path)?;
        }

        Ok::<(), Box<dyn Error + Send + Sync>>(())
    })?;

    if args.dry_run {
        println!("Dry run completed - no files were actually moved");
    } else if args.verbose {
        println!("Organization completed successfully");
    }

    Ok(())
}

fn collect_files(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error + Send + Sync>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        }
    }

    Ok(files)
}

fn get_destination_by_extension(file: &Path) -> String {
    file.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("Unknown")
        .to_lowercase()
}

fn get_destination_by_type(file: &Path) -> String {
    let ext = file
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "webp" => "Images",
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" | "rtf" | "md" => {
            "Documents"
        }
        "zip" | "rar" | "tar" | "gz" | "7z" | "bz2" => "Archives",
        "mp3" | "wav" | "flac" | "aac" | "ogg" => "Audio",
        "mp4" | "mov" | "avi" | "mkv" | "flv" | "wmv" => "Videos",
        "exe" | "msi" | "dmg" | "deb" | "rpm" => "Executables",
        "json" | "xml" | "yaml" | "yml" | "csv" => "Data",
        "html" | "htm" | "css" | "js" | "ts" | "jsx" | "tsx" => "Web",
        "rs" | "c" | "cpp" | "h" | "hpp" | "java" | "py" | "go" | "sh" => "Code",
        _ => "Other",
    }
    .to_string()
}