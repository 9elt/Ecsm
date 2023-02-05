use crate::config::ECSMConfig;
use std::path::PathBuf;

pub fn path_from_src(path: &PathBuf, config: &ECSMConfig) {
    let source_dir = match config.source_path() {
        Ok(path) => path,
        Err(_) => return ()
    };

    let to_remove = match source_dir.to_str() {
        Some(path_str) => path_str,
        None => return ()
    };

    let path_from_src = match path.to_str() {
        Some(path_str) => path_str.replace(to_remove, ""),
        None => return ()
    };

    print!("\x1b[1m{}\x1b[0m", path_from_src);
}

pub fn arrow() {
    print!(" > ");
}

pub fn success<S: AsRef<str>>(text: S) {
    print!("\x1b[32m\x1b[1m{}\x1b[0m", text.as_ref());
}

pub fn _warning<S: AsRef<str>>(text: S) {
    print!("\x1b[33m\x1b[1m{}\x1b[0m", text.as_ref());
}

pub fn error<S: AsRef<str>>(text: S) {
    print!("\x1b[31m\x1b[1m{}\x1b[0m", text.as_ref());
}

pub fn flush() {
    print!("\n");
}
