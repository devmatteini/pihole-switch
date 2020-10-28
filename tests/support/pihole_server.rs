use std::collections::HashMap;
use std::io::{BufRead, BufReader, Error, Write};
use std::net::{TcpListener, TcpStream};

use crate::support::pihole_response::PiHoleResponse;

pub struct PiHoleServer {
    api_token: String,
}

impl PiHoleServer {
    pub fn new(api_token: String) -> Self {
        PiHoleServer { api_token }
    }

    pub fn start(&self) -> Result<String, Error> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let url = format!("http://{}", listener.local_addr()?.to_string());
        let api_token = self.api_token.clone();

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                handle_connection(stream, &api_token);
            }
        });

        Ok(url)
    }
}

fn handle_connection(mut stream: TcpStream, api_token: &str) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    let (method, url) = parse_request_header(&mut reader);
    let query_params = parse_query_params(url);

    if method != "GET" {
        let response = http_raw_response_builder(PiHoleResponse::method_not_allowed());
        send_response(&mut stream, response);
        return;
    }

    let params = query_params_to_map(query_params);

    let response = process_request(params, api_token);
    let response = http_raw_response_builder(response);

    send_response(&mut stream, response);
}

fn query_params_to_map(query_params: Option<String>) -> HashMap<String, String> {
    match query_params {
        Some(value) => {
            let parsed = url::form_urlencoded::parse(value.as_bytes());
            let params_map: HashMap<String, String> = parsed.into_owned().collect();
            params_map
        }
        None => HashMap::new(),
    }
}

fn parse_request_header(reader: &mut dyn BufRead) -> (String, String) {
    let mut request_header = String::from("");
    reader.read_line(&mut request_header).unwrap();

    let request_header: Vec<&str> = request_header.split_whitespace().collect();

    (request_header[0].to_string(), request_header[1].to_string())
}

fn parse_query_params(url: String) -> Option<String> {
    let query_params: Vec<&str> = url.split('?').collect();
    query_params.get(1).map(|x| x.to_string())
}

fn send_response(stream: &mut TcpStream, response: String) {
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn http_raw_response_builder(response: PiHoleResponse) -> String {
    let headers = match response.headers {
        Some(value) => value,
        None => "".to_string(),
    };

    let body = match response.body {
        Some(value) => value.to_string(),
        None => "".to_string(),
    };

    format!(
        "HTTP/1.1 {}\r\n{}\r\n{}",
        response.status_line, headers, body
    )
}

fn process_request(params: HashMap<String, String>, api_token: &str) -> PiHoleResponse {
    let disable = params.get("disable");
    if disable.is_some() {
        return response(api_token, disable, params.get("auth"), "disabled");
    }

    let enable = params.get("enable");
    if enable.is_some() {
        return response(api_token, enable, params.get("auth"), "enabled");
    }

    PiHoleResponse::bad_request()
}

fn response(
    api_token: &str,
    operation: Option<&String>,
    auth: Option<&String>,
    return_status: &str,
) -> PiHoleResponse {
    match (operation, auth) {
        (Some(_), Some(token)) => match token == api_token {
            true => PiHoleResponse::ok(serde_json::json!({ "status": return_status })),
            false => PiHoleResponse::bad_request(),
        },
        _ => PiHoleResponse::bad_request(),
    }
}
