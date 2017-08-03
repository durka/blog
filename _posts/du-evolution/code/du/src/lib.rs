#[macro_use]
extern crate error_chain;
extern crate glob;
extern crate walkdir;

pub mod errors {
    use std::path::PathBuf;

    error_chain! {
        errors {
            Io(op: &'static str, path: PathBuf) {
                description("I/O operation failed")
                display("Could not {} {}", op, path.display())
            }
        }

        foreign_links {
            Glob(::glob::GlobError);
            GlobPattern(::glob::PatternError);
            WalkDir(::walkdir::Error);
        }
    }
    pub use self::ErrorKind::*;
}
