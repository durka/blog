#[macro_use] extern crate error_chain;
#[macro_use] extern crate closet;
extern crate ignore;
use ignore::{WalkBuilder, WalkState};

use std::env;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::sync::{Arc, Mutex};

error_chain! {
    foreign_links {
        Ignore(ignore::Error);
    }
}

fn local_du<P: AsRef<Path>>(path: P) -> Result<u64> {
    static TOTAL: AtomicUsize = ATOMIC_USIZE_INIT;
    TOTAL.store(0, Ordering::SeqCst);
    let error = Arc::new(Mutex::new(None));
    WalkBuilder::new(path).hidden(false).parents(false).ignore(false).git_ignore(false).git_exclude(false)
        .build_parallel()
        .run(clone_army!([error]
                         move || Box::new(clone_army!([error]
                                                 move |entry| match entry.and_then(|ent| ent.metadata()) {
                                                     Ok(meta) => {
                                                         TOTAL.fetch_add(meta.len() as usize, Ordering::Relaxed);
                                                         return WalkState::Continue;
                                                     }
                                                     Err(err) => {
                                                         *error.lock().unwrap() = Some(err);
                                                         return WalkState::Quit;
                                                     }
                                                 }))));
    if let Some(err) = Arc::try_unwrap(error).unwrap().into_inner().unwrap() {
        Err(err)?;
    }
    Ok(TOTAL.load(Ordering::SeqCst) as u64)
}

fn main() {
    let dir = PathBuf::from(env::args().skip(1).next().unwrap());
    match local_du(&dir) {
        Ok(bytes) => println!("{} {}", bytes, dir.display()),
        Err(error) => println!("ERROR: {:?}", error),
    }
}
