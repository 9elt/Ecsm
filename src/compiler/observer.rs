use super::compiler::ECSMCompiler;
use notify::{RecursiveMode, Result};
use notify_debouncer_mini::{new_debouncer, DebouncedEvent};
use std::time::Duration;

pub fn watch(compiler: &mut ECSMCompiler) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(500), None, tx).unwrap();

    debouncer
        .watcher()
        .watch(&compiler.config().source_path()?, RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => observer_router(event, compiler),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn observer_router(events: Vec<DebouncedEvent>, compiler: &mut ECSMCompiler) {
    for event in events {

        if !event.path.exists() {
            match event.path.is_dir() {
                true => compiler.remove_dir(event.path),
                false => compiler.remove_file(event.path),
            }.ok();
            continue;
        }

        match event.path.is_dir() {
            true => compiler.compile_files_in(&event.path),
            false => compiler.compile_file(event.path),
        }.ok();
    }
}
