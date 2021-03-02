use pulldown_cmark::{html, Parser};
use std::env;
use std::fs::{create_dir, read_dir, read_to_string, DirEntry};
use std::io;

// temp test
pub fn run() {
    println!(
        "{}",
        File::parse_markdown_file(&File::list_md_in_posts().unwrap()[0]).unwrap()
    );
}

struct File;

impl File {
    // readability is horrendous here
    // requires review
    fn parse_markdown_file(md_file: &DirEntry) -> Result<String, io::Error> {
        let file_path = match File::verify_extension(&md_file, "md") {
            true => md_file.path(),
            false => panic!("Failed to parse file"),
        };

        let file_content = read_to_string(file_path).expect("Failed to read file");
        let mut html_output = String::new();
        html::push_html(&mut html_output, Parser::new(&file_content));

        println!("{}", html_output);
        Ok(html_output)
    }

    fn create_posts_dir() -> Result<(), io::Error> {
        // not the best error-handling...horrendous to be frank
        // requires review
        match File::list_md_in_posts() {
            Ok(_) => Ok(()),
            Err(error) => match error.kind() {
                io::ErrorKind::NotFound => Ok(create_dir(File::get_posts_dir()?)?),
                err => Err(io::Error::new(err, "Failed to create posts directory")),
            },
        }
    }

    fn list_md_in_posts() -> Result<Vec<DirEntry>, io::Error> {
        // modified(?) from https://users.rust-lang.org/t/filtering-file-names-with-ends-with/16939/4
        let posts = read_dir(File::get_posts_dir()?)?
            .filter_map(Result::ok)
            .filter(|f| File::verify_extension(f, "md"))
            .collect::<Vec<DirEntry>>();
        Ok(posts)
    }

    fn get_posts_dir() -> Result<String, io::Error> {
        let mut path = env::current_dir()?;
        path.push("posts");
        Ok(format!("{}", path.display()))
    }
    fn verify_extension(f: &DirEntry, ext: &str) -> bool {
        if let Some(ext) = f.path().extension() {
            ext == "md"
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
