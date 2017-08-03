extern crate glob;

use std::env;
use std::path::{Path, PathBuf};

extern crate du;
use du::errors::*;

fn local_du(path: &Path) -> Result<u64> {
    Ok(path.read_dir()
           .chain_err(|| Io("list", path.into()))?
           .map(|res| match res {
               Ok(entry) => {
                   match entry.metadata() {
                       Ok(meta) => {
                           if meta.is_dir() {
                               local_du(&entry.path()).unwrap_or_else(|e| {
                                   println!("warning: could not list directory {} (skipping): {}",
                                            entry.path().display(), e);
                                   0
                               })
                           } else {
                               meta.len()
                           }
                       }
                       Err(e) => {
                           println!("warning: could not stat {} (skipping): {}",
                                    entry.path().display(), e);
                           0
                       }
                   }
               }
               Err(e) => {
                   println!("warning: could not stat entry in {} (skipping): {}",
                            path.display(), e);
                   0
               }
           })
           .fold(path.metadata().chain_err(|| Io("stat", path.into()))?.len(),
                 |a, b| a + b))
}

fn main() {
    let dir = PathBuf::from(env::args().skip(1).next().unwrap());
    match local_du(&dir) {
        Ok(bytes) => println!("{} {}", bytes, dir.display()),
        Err(error) => println!("ERROR: {:?}", error),
    }
}
