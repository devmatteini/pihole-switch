use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};

use serde_json;
use url;

pub struct PiHoleServer {
    api_key: String,
}

impl PiHoleServer {
    pub fn new(api_key: String) -> Self {
        PiHoleServer { api_key }
    }

    pub fn start(&self) -> Result<String, Error> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let url = format!("http://{}", listener.local_addr()?.to_string());
        let api_key = self.api_key.clone();

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                handle_connection(stream, &api_key);
            }
        });

        Ok(url)
    }
}

fn handle_connection(mut stream: TcpStream, api_key: &String) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    let (method, url) = parse_request_header(&mut reader);
    let query_params = parse_query_params(url);

    if method != "GET" {
        let response = http_response_builder("405 Method Not Allowed", None, None);
        send_response(&mut stream, response);
        return;
    }

    let params = query_params_to_map(query_params);

    let (status_line, json_content) = process_request(params, api_key);
    let content_type = format!("Content-Type: application/json\r\n");
    let response = http_response_builder(status_line, Some(content_type), Some(json_content.to_string()));

    send_response(&mut stream, response);
}

fn query_params_to_map(query_params: Option<String>) -> HashMap<String, String> {
    match query_params {
        Some(value) => {
            let parsed = url::form_urlencoded::parse(value.as_bytes());
            let params_map: HashMap<String, String> = parsed.into_owned().collect();
            params_map
        }
        None => HashMap::new()
    }
}

fn parse_request_header(reader: &mut dyn BufRead) -> (String, String) {
    let mut request_header = String::from("");
    reader.read_line(&mut request_header).unwrap();

    let request_header: Vec<&str> = request_header.split_whitespace().collect();

    (request_header[0].to_string(), request_header[1].to_string())
}

fn parse_query_params(url: String) -> Option<String> {
    let query_params: Vec<&str> = url.split("?").collect();
    query_params.get(1).map(|x| x.to_string())
}

fn send_response(stream: &mut TcpStream, response: String) {
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn http_response_builder(status_line: &str, headers: Option<String>, body: Option<String>) -> String {
    let headers = match headers {
        Some(value) => value,
        None => "".to_string()
    };

    let body = match body {
        Some(value) => value,
        None => "".to_string()
    };

    format!("HTTP/1.1 {}\r\n{}\r\n{}", status_line, headers, body)
}

fn process_request(params: HashMap<String, String>, api_key: &String) -> (&str, serde_json::Value) {
    let bad_request = ("200 OK", serde_json::json!([]));

    match (params.get("enable"), params.get("auth")) {
        (Some(_), Some(token)) => match token == api_key {
            true => ("200 OK", serde_json::json!({"status": "enabled"})),
            false => bad_request,
        },
        _ => bad_request,
    }
}
