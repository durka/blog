#[macro_use] extern crate error_chain;
extern crate fallible_iterator;
use fallible_iterator::{FallibleIterator, convert};
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
    use std::io::Write; let mut f = std::fs::File::create("v3.log").unwrap();
    Ok(convert(WalkDir::new(path).into_iter())
           .and_then(|entry| { writeln!(&mut f, "{} {}", entry.metadata().unwrap().len(), entry.path().display()).unwrap(); Ok(entry.metadata()?.len()) })
           .fold(0, |a, b| a + b)?)
}

fn main() {
    let dir = PathBuf::from(env::args().skip(1).next().unwrap());
    match local_du(&dir) {
        Ok(bytes) => println!("{} {}", bytes, dir.display()),
        Err(error) => println!("ERROR: {:?}", error),
    }
}
