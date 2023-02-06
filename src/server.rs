use crate::config::ECSMConfig;
use rouille::{Response, Server};
use std::io::Result;

use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

pub fn run(config: &ECSMConfig) -> Result<(JoinHandle<()>, Sender<()>)> {
    let output_path = config.output_path()?;
    let media_path = config.media_path()?;

    let server = Server::new(config.server(), move |request| {
        let mut request_url = request.url();

        let tmp_path = PathBuf::from(&request_url);
        let tmp_ext = match tmp_path.extension() {
            Some(ext) => ext.to_string_lossy().to_string(),
            None => "html".to_string(),
        };

        if request_url.chars().next() == "/".chars().next() {
            request_url.remove(0);
        }

        let request_path = match tmp_ext.as_str() {
            "html" => output_path.join(&request_url),
            "css" => output_path.join(&request_url),
            _ => media_path.join(&request_url),
        };

        if !request_path.starts_with(&output_path) && !request_path.starts_with(&media_path) {
            return Response::html("<h2>403 FORBIDDEN</h2>").with_status_code(404);
        }

        let request_path = match request_path.is_dir() {
            true => request_path.join("index.html"),
            false => request_path,
        };

        if !request_path.exists() {
            return Response::html("<h2>404 NOT FOUND</h2>").with_status_code(404);
        }

        let mime = match request_path.extension() {
            Some(ext) => rouille::extension_to_mime(ext.to_string_lossy().as_ref()),
            None => return Response::html("<h2>500 SERVER ERROR</h2>").with_status_code(404),
        };

        match File::open(request_path.to_owned()) {
            Ok(file) => Response::from_file(mime, file).with_status_code(200),
            Err(_) => Response::html("<h2>500 SERVER ERROR</h2>").with_status_code(404),
        }
    })
    .unwrap();

    Ok(server.stoppable())
}
