mod compiler;
mod parser;
mod setup;
mod utils;

use utils::config::ECSMConfig;

fn main() {
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

    match config.check_directories() {
        Ok(_) => (),
        Err(err) => panic!("direcotries check failed: {:?}", err),
    }

    let m = compiler::start(&config);
    println!("{:?}", m);
}
