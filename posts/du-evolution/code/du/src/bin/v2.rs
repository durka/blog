extern crate glob;
extern crate walkdir;

use walkdir::WalkDir;

use std::env;
use std::path::{Path, PathBuf};

extern crate du;
use du::errors::*;

fn local_du<P: AsRef<Path>>(path: P) -> Result<u64> {
    WalkDir::new(path).into_iter()
        .map(|entry| entry.and_then(|entry| entry.metadata())
             .map(|meta| meta.len()))
        .fold(Ok(0), |a, b| match (a, b) {
            (Ok(a), Ok(b)) => Ok(a + b),
            (err, _) => err
        })
}

fn main() {
    let dir = PathBuf::from(env::args().skip(1).next().unwrap());
    match local_du(&dir) {
        Ok(bytes) => println!("{} {}", bytes, dir.display()),
        Err(error) => println!("ERROR: {:?}", error)
    }
}

