use pulldown_cmark::{html, Parser};
use std::fs::{create_dir, read_dir, read_to_string, DirEntry};
use std::io;
use std::{env, path::PathBuf};
#[derive(Debug)]
struct HTMLFiles {
    content: String,
    path: PathBuf,
}

enum CustomError {
    FileError(io::Error),
    InvalidFileError(String),
}

impl HTMLFiles {
    fn new(entry: &DirEntry) -> Result<HTMLFiles, CustomError> {
        let content = match parse_markdown_file(entry) {
            Ok(res) => res,
            Err(e) => return Err(e),
        };
        let path = entry.path();
        Ok(HTMLFiles { content, path })
    }
}
fn convert_posts_entries_to_html() -> Result<Vec<HTMLFiles>, CustomError> {
    let dir_entries = match list_md_in_posts() {
        Ok(f) => f,
        Err(e) => return Err(CustomError::FileError(e)),
    };

    let collection = dir_entries
        .iter()
        .map(|entry| HTMLFiles::new(entry))
        .collect();
    collection
}
// readability is horrendous here
// requires review
fn parse_markdown_file(md_file: &DirEntry) -> Result<String, CustomError> {
    let file_path = match verify_extension(&md_file, "md") {
        true => md_file.path(),
        false => {
            return Err(CustomError::InvalidFileError(String::from(
                "File extension is not .md",
            )))
        }
    };

    let file_content = read_to_string(file_path).expect("Failed to read file");
    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new(&file_content));

    Ok(html_output)
}

// relevant for cli
fn create_posts_dir() -> Result<(), io::Error> {
    // not the best error-handling...horrendous to be frank
    // requires review
    match list_md_in_posts() {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => Ok(create_dir(get_posts_dir()?)?),
            err => Err(io::Error::new(err, "Failed to create posts directory")),
        },
    }
}

fn list_md_in_posts() -> Result<Vec<DirEntry>, io::Error> {
    // modified(?) from https://users.rust-lang.org/t/filtering-file-names-with-ends-with/16939/4
    let posts = read_dir(get_posts_dir()?)?
        .filter_map(Result::ok)
        .filter(|f| verify_extension(f, "md"))
        .collect::<Vec<DirEntry>>();
    Ok(posts)
}

fn get_posts_dir() -> Result<String, io::Error> {
    let mut path = env::current_dir()?;
    path.push("posts");
    Ok(format!("{}", path.display()))
}
fn verify_extension(f: &DirEntry, ext: &str) -> bool {
    if let Some(res) = f.path().extension() {
        res == ext
    } else {
        false
    }
}
