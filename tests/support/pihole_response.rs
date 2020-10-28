use serde_json::Value as JsonValue;

pub struct PiHoleResponse {
    pub status_line: String,
    pub headers: Option<String>,
    pub body: Option<JsonValue>,
}

impl PiHoleResponse {
    pub fn ok(json: JsonValue) -> PiHoleResponse {
        PiHoleResponse::from_json("200 OK".to_string(), json)
    }

    pub fn bad_request() -> PiHoleResponse {
        PiHoleResponse::from_json("200 OK".to_string(), serde_json::json!([]))
    }

    fn from_json(status_line: String, json: JsonValue) -> PiHoleResponse {
        PiHoleResponse {
            status_line,
            headers: Some("Content-Type: application/json\r\n".to_string()),
            body: Some(json),
        }
    }

    pub fn method_not_allowed() -> PiHoleResponse {
        PiHoleResponse {
            status_line: "405 Method Not Allowed".to_string(),
            headers: None,
            body: None,
        }
    }
}
