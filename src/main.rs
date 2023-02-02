mod compiler;
mod html_editor;
mod setup;
mod utils;
mod config;

use config::ECSMConfig;

fn main() {
    // load config
    let config = match ECSMConfig::parse() {
        Ok(conf) => conf,
        Err(_) => match ECSMConfig::new() {
            Ok(conf) => match setup::init_project(&conf) {
                Ok(_) => conf,
                Err(err) => panic!("project setup failed: {:?}", err),
            },
            Err(err) => panic!("cannot create config file: {:?}", err),
        },
    };

    // check directories
    match config.check_directories() {
        Ok(_) => (),
        Err(err) => panic!("direcotries check failed: {:?}", err),
    };

    // start auto compiler
    match compiler::start(&config) {
        Ok(_) => (),
        Err(err) => panic!("failed starting compiler: {:?}", err), 
    };
}
