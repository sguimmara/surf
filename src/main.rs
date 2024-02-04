use std::path::PathBuf;

use clap::Parser;

mod surf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to WAV file
    #[arg(index = 1)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    match std::fs::read(&args.file) {
        Ok(data) => match surf::get_info(&data) {
            Ok(info) => {
                println!("{}\n", (&args.file.file_name().unwrap()).to_str().unwrap_or("file"));
                println!("{}", info)
            },
            Err(e) => eprintln!("{}: {}", (&args.file).to_str().unwrap_or("file"), e),
        },
        Err(e) => eprintln!("{}: {}", (&args.file).to_str().unwrap_or("file"), e),
    };
}
