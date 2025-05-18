use std::env::args;
use std::fs::{create_dir_all, read_dir, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use build_html::{HtmlElement, HtmlTag, Html, HtmlPage, HtmlContainer};
use markdown;

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

    let markdown_files = get_markdown_files_in_data(data_directory_path);
    generate_html_templates(deposit_html_path, markdown_files);
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

fn generate_html_templates(deposit_html_path: &Path, markdown_files: Vec<PathBuf>) {
    create_dir_all(deposit_html_path).expect("Error creating HTML deposit directory");

    // Create index.html
    let index_file_path = deposit_html_path.join("index.html");
    let mut index = File::create(index_file_path).unwrap();

    let mut navbar = HtmlElement::new(HtmlTag::Navigation);

    for markdown_file_path in markdown_files {
        // Create file name in the HTML deposit directory with .html extension
        let mut file_name = markdown_file_path.clone();
        file_name.set_extension("html");
        let html_file_path = deposit_html_path.join(file_name.file_name().unwrap());
        let mut page = File::create(html_file_path.clone()).unwrap();

        // Read Markdown file
        let mut markdown= String::new();
        File::open(markdown_file_path).unwrap().read_to_string(&mut markdown).expect("Failed to read markdown file");

        // Write Markdown converted to HTML in the new HTML file created
        let html = markdown::to_html(markdown.as_str());
        page.write_all(html.as_bytes()).expect("Failed to write HTML file");

        navbar.add_child(HtmlElement::new(HtmlTag::Div)
            .with_link(html_file_path.parent().unwrap().parent().unwrap().join(html_file_path.file_name().unwrap()).to_str().unwrap(), html_file_path.file_name().unwrap().to_str().unwrap()).into());
    }

    // Create HTML for index
    let index_content = HtmlPage::new()
        .with_title("index")
        .with_html(
            navbar
        );

    index.write_all(index_content.to_html_string().as_bytes()).expect("Failed to write index.html");
}
