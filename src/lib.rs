use std::env;
use std::fs::read_dir;
use std::io;
//use std::path::PathBuf;

pub fn run() -> String {
    match read_posts_dir() {
        Ok(posts) => println!("{:?}", posts),
        Err(e) => println!(
            "Failed to read your posts directory.\nHere's the error: {}",
            e
        ),
    }
    format!("{:?}", read_posts_dir())
}

fn read_posts_dir() -> Result<Vec<String>, io::Error> {
    let posts = read_dir(get_posts_dir()?)?
        .map(|res| String::from(res.unwrap().path().to_str().unwrap()))
        .collect::<Vec<String>>();
    Ok(posts)
}

fn get_posts_dir() -> Result<String, io::Error> {
    let mut path = env::current_dir()?;
    path.push("posts");
    Ok(format!("{}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_fine() {
        run();
    }

    #[test]
    fn get_posts_dir_success() {
        get_posts_dir();
    }

    #[test]
    fn read_posts_dir_success() {
        read_posts_dir();
    }
}
