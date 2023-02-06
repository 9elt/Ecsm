use crate::config::ECSMConfig;
use rouille::{Response, Server};
use std::io::Result;

use std::fs::File;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub fn run(config: &ECSMConfig) -> Result<(JoinHandle<()>, Sender<()>)> {
    let output_path = config.output_path()?;

    let server = Server::new(config.server(), move |request| {
        let mut request_url = request.url();

        if request_url.chars().next() == "/".chars().next() {
            request_url.remove(0);
        }

        let request_path = output_path.join(&request_url);

        let request_path = match request_path.is_dir() {
            true => request_path.join("index.html"),
            false => request_path,
        };

        if !request_path.exists() {
            return Response::html("<h2>404 NOT FOUND</h2>").with_status_code(404);
        }

        let mime = match request_path.extension() {
            Some(ext) => match ext.to_string_lossy().as_ref() {
                "html" => "text/html; charset=utf8",
                "css" => "text/css",
                // for my future self... add more mime
                _ => "text/html",
            },
            None => "text/html; charset=utf8",
        };

        match File::open(request_path.to_owned()) {
            Ok(file) => Response::from_file(mime, file).with_status_code(200),
            Err(_) => Response::html("<h2>500 SERVER ERROR</h2>").with_status_code(404),
        }
    })
    .unwrap();

    Ok(server.stoppable())
}
