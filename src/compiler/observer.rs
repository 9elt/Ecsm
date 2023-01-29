use notify::{RecursiveMode, Result};
use notify_debouncer_mini::{new_debouncer, DebouncedEvent};
use std::time::Duration;
use super::compiler::ECSMCompiler;

pub fn watch(compiler: &ECSMCompiler) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(500), None, tx).unwrap();

    debouncer.watcher().watch(&compiler.config().source_path()?, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => observer_router(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn observer_router(events: Vec<DebouncedEvent>) {
    for event in events {
        let path_str = event.path.to_string_lossy().to_string();

        let file_or_dir_name = match path_str.split("/").collect::<Vec<&str>>().last() {
            Some(file_name) => file_name,
            None => "",
        };

        let split_name = file_or_dir_name.split(".").collect::<Vec<&str>>();

        let file_ext = match split_name.len() > 1 {
            true => match split_name.last() {
                Some(ext) => ext,
                None => "",
            },
            false => "dir",
        };

        if !event.path.exists() {
            // file or dir was deleted
            // when a html file is deleted, recompile all css
            continue;
        }

        match file_ext {
            "html" => println!("compiling [html] -> {}", file_or_dir_name),
            "css" => println!("compiling [css] -> {}", file_or_dir_name),
            "dir" => println!("compiling [dir] -> {}", file_or_dir_name),
            _ => (),
        }
    }
}
