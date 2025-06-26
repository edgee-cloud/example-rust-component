mod helpers;
mod world;

use std::collections::HashMap;

use world::bindings::exports::wasi::http::incoming_handler::Guest;
use world::bindings::wasi::http::types::IncomingRequest;
use world::bindings::wasi::http::types::ResponseOutparam;
use world::bindings::Component;

impl Guest for Component {
    fn handle(req: IncomingRequest, resp: ResponseOutparam) {
        let _ = match Settings::new(&req) {
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

        //let example = waki::Client::new()
        //    .get("https://example.com")
        //    .send()
        //    .unwrap()
        //    .body()
        //    .unwrap();

        //let body = String::from_utf8_lossy(&example).to_string();

        let mut builder = helpers::ResponseBuilder::new();
        builder
            .set_header("content-type", "text/html")
            .set_status_code(200)
            .set_body(include_str!("index.html"));
        builder.build(resp);
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub example: String,
}

impl Settings {
    pub fn new(req: &IncomingRequest) -> anyhow::Result<Self> {
        let headers = IncomingRequest::headers(req);
        let map = helpers::parse_headers(&headers);
        let settings = map
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
