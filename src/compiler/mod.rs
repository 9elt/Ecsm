mod observer;
mod compiler;

use crate::config::ECSMConfig;
use compiler::ECSMCompiler;
use std::io::Result;

pub fn start(config: &ECSMConfig) -> Result<()> {
    println!("starting [{}] autocompiler", config.name());

    let mut compiler = ECSMCompiler::new(config);

    match observer::watch(&mut compiler) {
        Ok(_) => println!("observer stopped..."),
        Err(err) => println!("error starting autocompiler: {:?}", err),
    };

    Ok(())
}
