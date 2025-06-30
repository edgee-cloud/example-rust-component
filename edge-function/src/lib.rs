mod helpers;
mod world;

use std::collections::HashMap;

use world::bindings::exports::wasi::http::incoming_handler::Guest;
use world::bindings::wasi::http::types::IncomingRequest;
use world::bindings::wasi::http::types::ResponseOutparam;
use world::bindings::Component;

impl Guest for Component {
    fn handle(req: IncomingRequest, resp: ResponseOutparam) {
        let body = include_str!("index.html");

        let _ = match Settings::from_req(&req) {
            Ok(settings) => settings,
            Err(_) => {
                let response = helpers::build_response_html(body, 200);
                response.send(resp);
                return;
            }
        };

        let _ = helpers::parse_body(req);

        // Uncomment the following lines to see how to use the waki client
        // let example = waki::Client::new()
        //    .get("https://example.com")
        //    .send()
        //    .unwrap()
        //    .body()
        //    .unwrap();
        // let body = String::from_utf8_lossy(&example).to_string();

        // Uncomment the following lines to see how to parse the request body and parse it to JSON
        // let request_body = match helpers::parse_body(req) {
        //     Ok(body) => body,
        //     Err(e) => {
        //         let response = helpers::build_response_json_error(&e, 400);
        //         response.send(resp);
        //         return;
        //     }
        // };
        // // parse body to JSON
        // let body_json: serde_json::Value = match serde_json::from_slice(&request_body) {
        //     Ok(json) => json,
        //     Err(_) => {
        //         let response = helpers::build_response_json_error("Invalid JSON in request body", 400);
        //         response.send(resp);
        //         return;
        //     }
        // };

        let response = helpers::build_response_html(body, 200);
        response.send(resp);
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub example: String,
}

impl Settings {
    pub fn from_req(req: &IncomingRequest) -> anyhow::Result<Self> {
        let map = helpers::parse_headers(&IncomingRequest::headers(req));
        Self::new(&map)
    }

    pub fn new(headers: &HashMap<String, Vec<String>>) -> anyhow::Result<Self> {
        let settings = headers
            .get("x-edgee-component-settings")
            .ok_or_else(|| anyhow::anyhow!("Missing 'x-edgee-component-settings' header"))?;

        if settings.len() != 1 {
            return Err(anyhow::anyhow!(
                "Expected exactly one 'x-edgee-component-settings' header, found {}",
                settings.len()
            ));
        }
        let setting = settings[0].clone();
        let setting: HashMap<String, String> = serde_json::from_str(&setting)?;

        let example = setting
            .get("example")
            .map(String::to_string)
            .unwrap_or_default();

        Ok(Self { example })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_new() {
        let mut headers = HashMap::new();
        headers.insert(
            "x-edgee-component-settings".to_string(),
            vec![r#"{"example": "test_value"}"#.to_string()],
        );

        let settings = Settings::new(&headers).unwrap();
        assert_eq!(settings.example, "test_value");
    }
}
