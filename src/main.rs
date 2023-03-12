use clap::Parser;
use std::{fs, io};

/// Local Search Engine for txt and pdf files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path to index
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    read_dir_recursive(&args.path).expect("Path is valid");
}

fn read_dir_recursive(dir_path: &str) -> io::Result<()> {
    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            read_dir_recursive(path.to_str().unwrap())?;
        } else {
            match path.extension().and_then(|e| e.to_str()) {
                Some("txt") => println!("TXT: {}", path.display()),
                Some("pdf") => println!("PDF: {}", path.display()),
                _ => println!("ELSE: {}", path.display()),
            }
        }
    }

    Ok(())
}
