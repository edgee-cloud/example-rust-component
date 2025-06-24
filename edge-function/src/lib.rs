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

impl Default for ResponseBuilder {
    fn default() -> Self {
        ResponseBuilder::new()
    }
}

impl ResponseBuilder {
    pub fn new() -> Self {
        ResponseBuilder {
            headers: Fields::new(),
            status_code: 200,
            body_content: None,
        }
    }

    pub fn set_headers(&mut self, headers: Fields) -> &mut Self {
        self.headers = headers;
        self
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
        let resp_tx = OutgoingResponse::new(self.headers);
        let _ = resp_tx.set_status_code(self.status_code);

        if let Some(body_content) = self.body_content {
            let body = resp_tx.body().unwrap();
            ResponseOutparam::set(resp, Ok(resp_tx));

            let stream = body.write().unwrap();
            stream.write(body_content.as_bytes()).unwrap();
            stream.flush().unwrap();

            let _ = OutgoingBody::finish(body, None);
        }
    }
}

impl Guest for Component {
    fn handle(req: wasi::http::types::IncomingRequest, resp: wasi::http::types::ResponseOutparam) {
        let incoming_headers = IncomingRequest::headers(&req);
        //let settings_headers = incoming_headers.get("X-Edgee-Settings");
        //// Parse the settings from the headers
        //if settings_headers.len() != 1 {
        //    panic!("Multiple X-Edgee-Settings headers found");
        //}
        //let settings = settings_headers.get(0).cloned();
        //let settings = match settings {
        //    Some(settings) => String::from_utf8_lossy(&settings).to_string(),
        //    None => {
        //        panic!("X-Edgee-Settings header not found");
        //    }
        //};
        //let settings = Settings::new(settings).unwrap();
        // let incoming_body = IncomingRequest::consume(&req).unwrap();
        // let incoming_body_stream = incoming_body.stream().unwrap();
        //       let body = incoming_body_stream.read().unwrap();
        //

        // request example.com
        //let out_req = OutgoingRequest::new(Fields::new());
        //let url = Url::parse("https://www.edgee.cloud/_edgee/status").unwrap();
        //out_req.set_scheme(Some(&wasi::http::types::Scheme::Https));
        //out_req.set_authority(Some(url.authority()));
        //out_req.set_path_with_query(Some(url.path()));
        //let fut = wasi::http::outgoing_handler::handle(out_req, None).unwrap();
        //fut.subscribe().block();
        //let fut_resp = fut.get();
        //let response = fut_resp.unwrap().unwrap().unwrap().consume().unwrap();
        //let example_stream = response.stream().unwrap();
        //
        let p_q = req.path_with_query().unwrap_or_default();
        println!("Request received: {:?}", p_q);

        let mut builder = ResponseBuilder::new();
        builder
            .set_headers(incoming_headers)
            //.set_header("content-type", "text/html")
            //.set_header(
            //    "content-length",
            //    &include_str!("index.html").len().to_string(),
            //)
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
