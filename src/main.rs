use std::path::{Path, PathBuf};
use std::fs;
use std::ffi::OsStr;
use std::vec;


fn find_csv_files(dir: &Path) -> Vec<PathBuf>{
    let mut files = vec::Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let maybe_file = entry.unwrap();
        if maybe_file.path().is_file() && is_csv(&maybe_file.path()) {
            files.push(maybe_file.path());

        }        
    }
    files
}

fn is_csv(maybe_file: &Path) -> bool {
    let ext = match maybe_file.extension() {
        Some(e) => e,
        _ => OsStr::new("no_extension"),
    };
    match ext.to_str().unwrap() {
        "csv" => true,
        _ => false,
    }
}

fn main() {
    let dir = Path::new(".");
    let files = find_csv_files(&dir);
    for f in &files {
        println!("{:?}", f)
    }
    
    
}
