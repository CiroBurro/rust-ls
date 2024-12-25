use ansi_term::Colour;
use std::env::current_dir;
use std::fs::read_dir;
use std::io::Error;
use std::path::PathBuf;

fn main() {
    let path = current_dir();
    let files = trova_file(path);
    println!("\nFile in questa cartella:");
    for (i, file) in files.iter().enumerate() {
        println!("{}) {}", i + 1, file);
    }
    println!("\n")
}

fn trova_file(path: Result<PathBuf, Error>) -> Vec<String> {
    let entries = read_dir(path.unwrap()).unwrap();

    let mut files = Vec::new();
    let mut dirs = Vec::new();
    for entry in entries {
        let file = entry.unwrap();
        let file_type = file.file_type().unwrap();
        if file_type.is_dir() {
            dirs.push(file.file_name());
        } else if file_type.is_file() {
            files.push(file.file_name());
        }
    }

    let mut nomi: Vec<String> = Vec::new();
    for dir in dirs.iter() {
        let nome = dir.to_str().unwrap();
        nomi.push(format!("📁{}/", Colour::Yellow.paint(nome)));
    }
    for file in files.iter() {
        let nome = file.to_str().unwrap();
        nomi.push(nome.to_string());
    }
    nomi
}
