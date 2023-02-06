use crate::config::ECSMConfig;
use std::path::PathBuf;

pub fn running_compiler(name: &String) {
    println!(
        "\nstarting \x1b[33m\x1b[1m[{}]\x1b[0m live compiler\n",
        name
    );
}

pub fn running_server(name: &String, host: &String) {
    println!(
        "\nstarting \x1b[33m\x1b[1m[{}]\x1b[0m development server on \x1b[33m\x1b[1mhttp://{}\x1b[0m\n",
        name, host
    );
}

pub fn create_project() {
    print!("\ncreate a new project | \x1b[33m\x1b[1mname\x1b[0m: ");
}

pub fn path_from_src(path: &PathBuf, config: &ECSMConfig) {
    let ext = path.extension();

    let source_dir = match config.source_path() {
        Ok(path) => path,
        Err(_) => return (),
    };

    let to_remove = source_dir.to_string_lossy();
    let from_src = path.to_string_lossy().replace(to_remove.as_ref(), "");

    print!(
        "{} \x1b[1m{}\x1b[0m",
        match ext {
            Some(ext) => match ext.to_str() {
                Some("html") => "\x1b[1m\x1b[38;5;215mhtml\x1b[0m",
                Some("css") => "\x1b[1m\x1b[38;5;39mcss\x1b[0m",
                _ => "",
            },
            _ => "",
        },
        from_src
    );
}

pub fn arrow() {
    print!(" > ");
}

pub fn success<S: AsRef<str>>(text: S) {
    print!("\x1b[32m\x1b[1m{}\x1b[0m", text.as_ref());
}

pub fn warning<S: AsRef<str>>(text: S) {
    print!("\x1b[33m\x1b[1m{}\x1b[0m", text.as_ref());
}

pub fn error<S: AsRef<str>>(text: S) {
    print!("\x1b[31m\x1b[1m{}\x1b[0m", text.as_ref());
}

pub fn flush() {
    print!("\n");
}
