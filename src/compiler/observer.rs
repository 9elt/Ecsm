use super::compiler::ECSMCompiler;
use notify::{RecursiveMode, Result};
use notify_debouncer_mini::{new_debouncer, DebouncedEvent};
use std::time::Duration;

pub fn watch(compiler: &ECSMCompiler) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(500), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(&compiler.config().source_path()?, RecursiveMode::Recursive)?;

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
        let name = match event.path.file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name,
                None => "", // directry check
            },
            None => "",
        };

        if !event.path.exists() {
            // file or dir was deleted
            // when a html file is deleted, recompile all css
            continue;
        }

        let ext = match event.path.extension() {
            Some(ext) => ext.to_str(),
            None => match event.path.is_dir() {
                true => Some("dir"),
                false => None,
            },
        };

        match ext {
            Some("html") => println!("compiling [html] -> {}", name),
            Some("css") => println!("compiling [css] -> {}", name),
            Some("dir") => println!("compiling [dir] -> {}", name),
            _ => (),
        }
    }
}
