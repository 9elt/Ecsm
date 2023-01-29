mod observer;
mod compiler;

use crate::config::ECSMConfig;
use compiler::ECSMCompiler;
use std::io::Result;

pub fn start(config: &ECSMConfig) -> Result<()> {
    println!("\nstarting \x1b[33m\x1b[1m[{}]\x1b[0m autocompiler\n", config.name());

    let mut compiler = ECSMCompiler::new(config);

    match observer::watch(&mut compiler) {
        Ok(_) => println!("observer stopped..."),
        Err(err) => println!("error starting autocompiler: {:?}", err),
    };

    Ok(())
}
