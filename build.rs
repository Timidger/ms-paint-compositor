use std::{env, fs, io};

extern crate fs_extra;

fn main() {
    match fs::create_dir(env::home_dir().unwrap().join(".icons")) {
        Ok(_) => {},
        Err(err) => if err.kind() != io::ErrorKind::AlreadyExists {
            Err(io::Error::new(err.kind(), err)).unwrap()
        }
    }
    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    fs_extra::copy_items(&vec!["./paint-brush"],
                         env::home_dir().unwrap().join(".icons"),
                         &options)
        .expect("Could not move brush icon to ~/.icons");
}
