#[macro_use] extern crate error_chain;
extern crate walkdir;
use walkdir::WalkDir;

use std::env;
use std::path::{Path, PathBuf};

error_chain! {
    foreign_links {
        WalkDir(::walkdir::Error);
    }
}

fn local_du<P: AsRef<Path>>(path: P) -> Result<u64> {
    WalkDir::new(path)
        .into_iter()
        .map(|entry| entry.and_then(|entry| entry.metadata())
                          .map(|meta| meta.len())
                          .map_err(Into::into))
        .fold(Ok(0),
              |a, b| match (a, b) {
                  (Ok(a), Ok(b)) => Ok(a + b),
                  (e @ Err(_), _) | (_, e @ Err(_)) => e,
              })
}

fn main() {
    let dir = PathBuf::from(env::args().skip(1).next().unwrap());
    match local_du(&dir) {
        Ok(bytes) => println!("{} {}", bytes, dir.display()),
        Err(error) => println!("ERROR: {:?}", error),
    }
}
