use std::env::args;
use std::fs::{create_dir_all, read_dir};
use std::path::{Path, PathBuf};

fn main() {
    // Confirm that the necessary parameters are present
    let args: Vec<String> = args().collect();
    if args.len() < 3 {
        eprintln!("Not enough parameters provided.\nExpected: <data_directory_path> <deposit_html_path>");
        std::process::exit(2);
    }

    // Define paths from parameters
    let data_directory_path = Path::new(&args[1]);
    let deposit_html_path = Path::new(&args[2]);

    // Confirm paths are valid
    if !data_directory_path.is_dir() {
        eprintln!("Path provided for data directory is not valid.");
        std::process::exit(2);
    } else if create_dir_all(deposit_html_path).is_err() {
        eprintln!("Directory to deposit HTMl not found and creation failed.");
        std::process::exit(1);
    }

    println!("Initialization worked!\nData directory path: \t{}\nDeposit HTML path: \t{}",
             data_directory_path.canonicalize().unwrap().display(), deposit_html_path.canonicalize().unwrap().display());
    
    _ = get_markdown_files_in_data(data_directory_path);
}

fn get_markdown_files_in_data(directory: &Path) -> Vec<PathBuf> {
    let mut markdown_files = Vec::new();
    for file in read_dir(directory).unwrap() {
        let file = file.unwrap();
        if file.path().to_str().unwrap().ends_with(".md") | file.path().to_str().unwrap().ends_with(".markdown") {
            markdown_files.push(file.path());
        }
    }

    markdown_files
}