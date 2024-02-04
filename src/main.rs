use std::path::{Path, PathBuf};

use clap::{builder::OsStr, Parser, Subcommand};

mod surf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print metadata on WAV file
    Info {
        /// Path to WAV file
        #[arg(index = 1)]
        file: PathBuf,
    },
}

fn print_info(file: &Path) {
    let filename = file
        .file_name()
        .unwrap_or(&OsStr::from("file"))
        .to_str()
        .unwrap_or("file")
        .to_string();

    // TODO exit with error codes
    match std::fs::read(file) {
        Ok(data) => match surf::get_info(&data) {
            Ok(info) => println!("{}: {}", filename, info),
            Err(e) => eprintln!("{}: {}", filename, e),
        },
        Err(e) => eprintln!("{}: {}", filename, e),
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Info { file } => print_info(&file),
    }
}
