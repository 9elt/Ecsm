mod compiler;
mod parser;
mod setup;
mod utils;

use utils::config::ECSMConfig;

fn main() {
    let config = match ECSMConfig::parse() {
        Ok(conf) => conf,
        Err(_) => match ECSMConfig::new() {
            Ok(conf) => {
                //setup::init_project(&conf);
                conf
            },
            Err(err) => panic!("cannot create config file: {:?}", err),
        },
    };

    let m = compiler::start(&config);
    println!("{:?}", m);
}
