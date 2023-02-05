use super::compiler::ECSMCompiler;
use super::config::ECSMConfig;
use super::server;
use notify::{RecursiveMode, Result};
use notify_debouncer_mini::{new_debouncer, DebouncedEvent};
use std::time::Duration;

pub fn start(config: &ECSMConfig) -> Result<()> {
    println!(
        "\nstarting \x1b[33m\x1b[1m[{}]\x1b[0m live compiler\n",
        config.name()
    );

    let mut compiler = ECSMCompiler::new(config);

    println!(
        "\nstarting \x1b[33m\x1b[1m[{}]\x1b[0m development server on \x1b[33m\x1b[1mhttp://{}\x1b[0m\n",
        config.name(), config.server()
    );

    server::create(config).expect("error creating dev server");

    observe(&mut compiler).expect("error starting live compiler");

    Ok(())
}

pub fn observe(compiler: &mut ECSMCompiler) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_millis(300), None, tx).unwrap();

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
            }
            .ok();
            continue;
        }

        match event.path.is_dir() {
            true => compiler.compile_files_in(&event.path, "*"),
            false => compiler.compile_file(event.path, "*"),
        }
        .ok();
    }
}
