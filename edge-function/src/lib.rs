wit_bindgen::generate!({world: "edge-function", path: ".edgee/wit", generate_all});
export!(Component);
struct Component;

use exports::wasi::http::incoming_handler::{Guest, ResponseOutparam};
use wasi::http::types::IncomingRequest;
use wasi::http::types::{Fields, OutgoingBody, OutgoingResponse};

pub struct ResponseBuilder {
    headers: Fields,
    status_code: u16,
    body_content: Option<String>,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        ResponseBuilder {
            headers: Fields::new(),
            status_code: 200,
            body_content: None,
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) -> &mut Self {
        let _ = self
            .headers
            .set(key, vec![value.as_bytes().to_vec()].as_slice());
        self
    }

    pub fn set_status_code(&mut self, status_code: u16) -> &mut Self {
        self.status_code = status_code;
        self
    }

    pub fn set_body(&mut self, body: &str) -> &mut Self {
        self.body_content = Some(body.to_string());
        self
    }

    pub fn build(self, resp: ResponseOutparam) {
        let mut resp_tx = OutgoingResponse::new(self.headers);
        let _ = resp_tx.set_status_code(self.status_code);

        if let Some(body_content) = self.body_content {
            let body = resp_tx.body().unwrap();
            ResponseOutparam::set(resp, Ok(resp_tx));

            let mut stream = body.write().unwrap();
            stream
                .blocking_write_and_flush(body_content.as_bytes())
                .unwrap();
            drop(stream);

            let _ = OutgoingBody::finish(body, None);
        }
    }
}

impl Guest for Component {
    fn handle(req: wasi::http::types::IncomingRequest, resp: wasi::http::types::ResponseOutparam) {
        let incoming_headers = IncomingRequest::headers(&req);

        let mut builder = ResponseBuilder::new();
        builder
            .set_header("content-type", "text/html")
            .set_header(
                "content-length",
                &include_str!("index.html").len().to_string(),
            )
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
