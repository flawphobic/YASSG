use std::env;
use std::fs::{create_dir, read_dir};
use std::io;

fn create_posts_dir() -> Result<(), io::Error> {
    // not the best error-handling
    // requires review
    match read_posts_dir() {
        Ok(_) => Ok(()),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match create_dir(get_posts_dir()?) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            err => Err(io::Error::new(err, "Failed")),
        },
    }
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
}
