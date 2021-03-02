use std::env;
use std::fs::{create_dir, read_dir, DirEntry};
use std::io;

// temp test
pub fn run() {
    println!("{:#?}", File::read_md_in_posts().unwrap());
}

struct File;

impl File {
    fn create_posts_dir() -> Result<(), io::Error> {
        // not the best error-handling...horrendous to be frank
        // requires review
        match File::read_md_in_posts() {
            Ok(_) => Ok(()),
            Err(error) => match error.kind() {
                io::ErrorKind::NotFound => Ok(create_dir(File::get_posts_dir()?)?),
                err => Err(io::Error::new(err, "Failed to create posts directory")),
            },
        }
    }
    fn read_md_in_posts() -> Result<Vec<DirEntry>, io::Error> {
        // modified(?) from https://users.rust-lang.org/t/filtering-file-names-with-ends-with/16939/4
        let posts = read_dir(File::get_posts_dir()?)?
            .filter_map(Result::ok)
            .filter(|f| {
                if let Some(ext) = f.path().extension() {
                    ext == "md"
                } else {
                    false
                }
            })
            .collect::<Vec<DirEntry>>();
        Ok(posts)
    }
    fn get_posts_dir() -> Result<String, io::Error> {
        let mut path = env::current_dir()?;
        path.push("posts");
        Ok(format!("{}", path.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
