use std::collections::HashMap;

use bytes::Bytes;

use bindings::wasi::http::types::{IncomingRequest, ResponseOutparam};

mod bindings {
    wit_bindgen::generate!({
        path: ".edgee/wit",
        world: "edge-function",
        generate_all,
        pub_export_macro: true,
        default_bindings_module: "$crate::bindings",
    });
}
mod helpers;

struct Component;
bindings::export!(Component);

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(req: IncomingRequest, response_out: ResponseOutparam) {
        helpers::run(req, response_out, |req| {
            let _settings = Settings::from_req(&req)?;

            let body = Bytes::from_static(include_bytes!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/public/index.html"
            )));

            // Uncomment the following lines to see how to use the waki client
            // let example = waki::Client::new()
            //    .get("https://example.com")
            //    .send()
            //    .unwrap()
            //    .body()
            //    .unwrap();
            // let body = String::from_utf8_lossy(&example).to_string();

            Ok(http::Response::builder()
                .status(200)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(body)?)
        });

        // Or use the following handler to see how to handle a JSON request
        // helpers::run_json::<_, serde_json::Value, _>(req, response_out, |req| {
        //     let _settings = Settings::from_req(&req)?;
        //
        //     Ok(http::Response::builder()
        //         .status(200)
        //         .body(serde_json::json!({
        //             "status": "success",
        //             "input": req.body(),
        //         }))?)
        // });
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    example: String,
}

impl Settings {
    pub fn new(headers: &http::header::HeaderMap) -> anyhow::Result<Self> {
        let value = headers
            .get("x-edgee-component-settings")
            .ok_or_else(|| anyhow::anyhow!("Missing 'x-edgee-component-settings' header"))
            .and_then(|value| value.to_str().map_err(Into::into))?;
        let data: HashMap<String, String> = serde_json::from_str(value)?;

        Ok(Self {
            example: data["example"].to_string(),
        })
    }

    pub fn from_req<B>(req: &http::Request<B>) -> anyhow::Result<Self> {
        Self::new(req.headers())
    }
}
