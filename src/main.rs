mod compiler;
mod config;
mod html_editor;
mod observer;
mod server;
mod setup;
mod utils;

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

    // start live compiler and dev server
    match observer::start(&config) {
        Ok(_) => (),
        Err(err) => panic!("failed starting compiler: {:?}", err),
    };
}
