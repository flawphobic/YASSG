use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn run() -> String {
    String::from("")
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
    fn get_posts_dir() {
        let mut posts_dir = env::current_dir().unwrap();
        posts_dir.push("posts");
        assert_eq!(super::get_posts_dir(), format!("{}", posts_dir.display()));
    }
}
