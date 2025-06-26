mod helpers;
mod world;

use world::bindings::exports::wasi::http::incoming_handler::Guest;
use world::bindings::wasi::http::types::IncomingRequest;
use world::bindings::wasi::http::types::ResponseOutparam;
use world::bindings::Component;

impl Guest for Component {
    fn handle(req: IncomingRequest, resp: ResponseOutparam) {
        let _ = helpers::parse_headers(&IncomingRequest::headers(&req));

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
    pub fn new(settings_headers: String) -> anyhow::Result<Self> {
        serde_json::from_str(&settings_headers)
            .map_err(|e| anyhow::anyhow!("Failed to parse settings: {}", e))
    }
}
