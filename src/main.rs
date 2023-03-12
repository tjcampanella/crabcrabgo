use std::{fs, io};

fn main() {
    read_dir_recursive("/Users/tjcampanella/Desktop").expect("works");
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
