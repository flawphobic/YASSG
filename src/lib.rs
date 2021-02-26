use std::env;
use std::fs::read_dir;
//use std::io;
//use std::path::PathBuf;

pub fn run() -> String {
    format!("{:?}", read_posts_dir())
}

fn read_posts_dir() -> Vec<String> {
    let posts = read_dir(get_posts_dir())
        .unwrap()
        .map(|res| String::from(res.unwrap().path().to_str().unwrap()))
        .collect::<Vec<String>>();
    posts
}

fn get_posts_dir() -> String {
    let mut path = env::current_dir().expect("Failed to get posts directory");
    path.push("posts");
    format!("{}", path.display())
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
