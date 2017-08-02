extern crate fallible_iterator;
extern crate glob;
extern crate walkdir;

use fallible_iterator::{convert, FallibleIterator};
use walkdir::WalkDir;

use std::env;
use std::path::{Path, PathBuf};

extern crate du;
use du::errors::*;

fn local_du<P: AsRef<Path>>(path: P) -> Result<u64> {
    Ok(convert(WalkDir::new(path).into_iter())
       .and_then(|entry| Ok(entry.metadata()?.len()))
       .fold(0, |a, b| a + b)?)
}

fn main() {
    let dir = PathBuf::from(env::args().skip(1).next().unwrap());
    match local_du(&dir) {
        Ok(bytes) => println!("{} {}", bytes, dir.display()),
        Err(error) => println!("ERROR: {:?}", error)
    }
}

