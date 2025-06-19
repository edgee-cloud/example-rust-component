wit_bindgen::generate!({world: "edge-function", path: ".edgee/wit", generate_all});
export!(Component);
struct Component;

use exports::wasi::http::incoming_handler::{Guest, ResponseOutparam};
use wasi::http::types::IncomingRequest;
use wasi::http::types::{Fields, OutgoingBody, OutgoingResponse};

use wasi::http::outgoing_handler::OutgoingRequest;

use serde_json;
use url::Url;

impl Guest for Component {
    fn handle(req: wasi::http::types::IncomingRequest, resp: wasi::http::types::ResponseOutparam) {
        let incoming_headers = IncomingRequest::headers(&req);
        let settings_headers = incoming_headers.get("X-Edgee-Settings");
        // Parse the settings from the headers
        if settings_headers.len() != 1 {
            panic!("Multiple X-Edgee-Settings headers found");
        }
        let settings = settings_headers.get(0).cloned();
        let settings = match settings {
            Some(settings) => String::from_utf8_lossy(&settings).to_string(),
            None => {
                panic!("X-Edgee-Settings header not found");
            }
        };
        let settings = Settings::new(settings).unwrap();
        // let incoming_body = IncomingRequest::consume(&req).unwrap();
        // let incoming_body_stream = incoming_body.stream().unwrap();
        //       let body = incoming_body_stream.read().unwrap();
        let response_headers = Fields::new();

        let _ = response_headers.set(
            "content-type",
            vec!["text/html".as_bytes().to_vec()].as_slice(),
        );

        // request example.com
        let out_req = OutgoingRequest::new(Fields::new());
        let url = Url::parse("https://www.edgee.cloud/_edgee/status").unwrap();
        out_req.set_scheme(Some(&wasi::http::types::Scheme::Https));
        out_req.set_authority(Some(url.authority()));
        out_req.set_path_with_query(Some(url.path()));
        let fut = wasi::http::outgoing_handler::handle(out_req, None).unwrap();
        fut.subscribe().block();
        let fut_resp = fut.get();
        let response = fut_resp.unwrap().unwrap().unwrap().consume().unwrap();
        let example_stream = response.stream().unwrap();

        // stream
        let resp_tx = OutgoingResponse::new(response_headers);
        let _ = resp_tx.set_status_code(200);
        let body = resp_tx.body().unwrap();
        ResponseOutparam::set(resp, Ok(resp_tx));

        // stream the response body
        let stream = body.write().unwrap();
        while let Ok(chunk) = example_stream.read(8192) {
            let _ = stream.write(&chunk);
        }
        // finish the response -> drop flushes the stream
        drop(stream);
        // this tells the host that the response is complete
        let _ = OutgoingBody::finish(body, None);
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
