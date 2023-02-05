use crate::config::ECSMConfig;
use rouille::{Response, Server};
use std::io::Result;

use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub fn create(config: &ECSMConfig) -> Result<(JoinHandle<()>, Sender<()>)> {
    let output_path = config.output_path()?;

    let server = Server::new(config.server(), move |request| {
        let mut request_url = request.url();

        let is_dir = request_url.clone().pop() == Some("/".chars().next().unwrap());

        request_url.remove(0);

        let res_path = output_path.join(PathBuf::from(match is_dir {
            true => format!("{request_url}index.html"),
            false => request_url,
        }));

        match File::open(res_path.to_owned()) {
            Ok(file) => {
                let mime = match &res_path.extension() {
                    Some(ext) => match ext.to_string_lossy().as_ref() {
                        "html" => "text/html; charset=utf8",
                        "css" => "text/css",
                        _ => "text/html; charset=utf8",
                    },
                    None => "text/html; charset=utf8",
                };

                Response::from_file(mime, file).with_status_code(200)
            }
            Err(_) => Response::html("404 error.").with_status_code(404),
        }
    })
    .unwrap();

    Ok(server.stoppable())
}
