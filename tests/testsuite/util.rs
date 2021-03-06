use elba::{
    index::Index,
    package::resolution::DirectRes,
    retrieve::cache::{Cache, Layout},
    util::{copy_dir, lock::DirLock, shell::Shell},
};
use slog::{self, Logger};
use std::{path::PathBuf, str::FromStr};
use tempdir::TempDir;

lazy_static! {
    pub static ref INDEX_DIR: TempDir = index_dir();
    pub static ref CACHE_DIR: TempDir = cache_dir();
    pub static ref LOGGER: Logger = new_logger();
    pub static ref CACHE: Cache = cache();
}

fn new_logger() -> Logger {
    /*
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    */

    // Suppress logging output during tests - we don't need to see it
    Logger::root(slog::Discard, o!())
}

pub fn index() -> Index {
    let url = DirectRes::from_str("dir+data/index").unwrap();
    let path = DirLock::acquire(&INDEX_DIR.path()).unwrap();
    Index::from_disk(url, path).unwrap()
}

pub fn shell() -> Shell {
    Shell::default()
}

pub fn cache() -> Cache {
    let layout = Layout {
        bin: CACHE_DIR.path().join("bin"),
        build: CACHE_DIR.path().join("build"),
        indices: CACHE_DIR.path().join("indices"),
        src: CACHE_DIR.path().join("src"),
        tmp: CACHE_DIR.path().join("tmp"),
    };

    Cache::from_disk(&LOGGER, layout, shell()).unwrap()
}

fn index_dir() -> TempDir {
    let start = env!("CARGO_MANIFEST_DIR");
    let mut path = PathBuf::new();
    path.push(start);
    path.push("tests/data/index");

    let tmp = TempDir::new("elba").unwrap();

    copy_dir(&path, tmp.path()).unwrap();

    tmp
}

fn cache_dir() -> TempDir {
    let start = env!("CARGO_MANIFEST_DIR");
    let mut path = PathBuf::new();
    path.push(start);
    path.push("tests/data/cache");

    let tmp = TempDir::new("elba").unwrap();

    copy_dir(&path, tmp.path()).unwrap();

    tmp
}
