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
        helpers::run(req, response_out, |req: http::Request<()>| {
            use helpers::body::Html;

            let _settings = Settings::from_req(&req)?;

            let body = Bytes::from_static(
                br#"
                <html lang="en>
                    <head>
                        <title>Edgee Edge Function Component</title>
                    </head>
                    <body>
                        <p>Hello world</p>
                    </body>
                </html>
            "#,
            );

            // Uncomment the following lines to see how to use the waki client
            // let example = waki::Client::new()
            //     .get("https://example.com")
            //     .send()?
            //     .body()?;
            // let body = String::from_utf8_lossy(&example).to_string();

            Ok(http::Response::builder().status(200).body(Html(body))?)
        });

        // Or use the following handler to see how to handle a JSON request
        // use helpers::body::Json;
        // helpers::run(
        //     req,
        //     response_out,
        //     |req: http::Request<Json<serde_json::Value>>| {
        //         let _settings = Settings::from_req(&req)?;
        //
        //         let Json(data) = req.body();
        //
        //         Ok(http::Response::builder()
        //             .status(200)
        //             .body(Json(serde_json::json!({
        //                 "status": "success",
        //                 "input": data,
        //             })))?)
        //     },
        // );

        // You can even mix up types
        // use helpers::body::{Html, Json};
        // helpers::run(
        //     req,
        //     response_out,
        //     |req: http::Request<Json<serde_json::Value>>| {
        //         let Json(_data) = req.into_body();
        //
        //         let body = Bytes::from_static(include_bytes!(concat!(
        //             env!("CARGO_MANIFEST_DIR"),
        //             "/public/index.html"
        //         )));
        //
        //         Ok(http::Response::builder().status(200).body(Html(body))?)
        //     },
        // );
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
