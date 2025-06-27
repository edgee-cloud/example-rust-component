mod helpers;
mod world;

use std::collections::HashMap;

use world::bindings::exports::wasi::http::incoming_handler::Guest;
use world::bindings::wasi::http::types::IncomingRequest;
use world::bindings::wasi::http::types::ResponseOutparam;
use world::bindings::Component;

impl Guest for Component {
    fn handle(req: IncomingRequest, resp: ResponseOutparam) {
        let _ = match Settings::from_req(&req) {
            Ok(settings) => settings,
            Err(_) => {
                let mut builder = helpers::ResponseBuilder::new();
                builder
                    .set_header("content-type", "text/html")
                    .set_status_code(200)
                    .set_body(include_str!("index.html"));
                builder.build(resp);
                return;
            }
        };

        let _ = helpers::parse_body(req);

        //let example = waki::Client::new()
        //    .get("https://example.com")
        //    .send()
        //    .unwrap()
        //    .body()
        //    .unwrap();

        //let body = String::from_utf8_lossy(&example).to_string();
        let body = include_str!("index.html");

        let mut builder = helpers::ResponseBuilder::new();
        builder
            .set_header("content-type", "text/html")
            .set_status_code(200)
            .set_body(body);
        builder.build(resp);
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
