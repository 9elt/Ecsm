mod observer;

use std::env::current_dir;

use crate::utils::config::ECSMConfig;
use std::io::Result;

pub fn start(config: &ECSMConfig) -> Result<()> {
    println!("starting [{}] autocompiler", config.name());

    let source_dir = current_dir()?.join(config.source_dir());

    let observer = observer::watch(&source_dir);

    match observer {
        Ok(_) => println!("observer stopped..."),
        Err(err) => println!("error starting autocompiler: {:?}", err),
    };

    Ok(())
}
