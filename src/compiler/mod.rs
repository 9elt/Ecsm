mod observer;

use crate::utils::config::ECSMConfig;
use std::io::Result;

pub fn start(config: &ECSMConfig) -> Result<()> {
    println!("starting [{}] autocompiler", config.name());

    let source_path = config.source_path()?;

    let observer = observer::watch(&source_path);

    match observer {
        Ok(_) => println!("observer stopped..."),
        Err(err) => println!("error starting autocompiler: {:?}", err),
    };

    Ok(())
}
