wit_bindgen::generate!({world: "edge-function", path: ".edgee/wit", generate_all});
export!(Component);
struct Component;

use exports::wasi::http::incoming_handler::{Guest, IncomingRequest, ResponseOutparam};
use wasi::http::types::{ErrorCode, Fields, OutgoingBody, OutgoingResponse};

impl Guest for Component {
    fn handle(req: wasi::http::types::IncomingRequest, resp: wasi::http::types::ResponseOutparam) {
        //let incoming_headers = IncomingRequest::headers(&req);
        // let incoming_body = IncomingRequest::consume(&req).unwrap();
        // let incoming_body_stream = incoming_body.stream().unwrap();
        //       let body = incoming_body_stream.read().unwrap();
        let response_body = "hello world\n";
        let response_headers = Fields::new();

        let _ = response_headers.set(
            "content-type",
            vec!["text/plain".as_bytes().to_vec()].as_slice(),
        );
        let _ = response_headers.set(
            "content-length",
            vec![response_body.len().to_string().as_bytes().to_vec()].as_slice(),
        );

        let resp_tx = OutgoingResponse::new(response_headers);
        let _ = resp_tx.set_status_code(200);
        let body = resp_tx.body().unwrap();
        ResponseOutparam::set(resp, Ok(resp_tx));
        let stream = body.write().unwrap();
        stream.write(response_body.as_bytes()).unwrap();
        drop(stream);
        let _ = OutgoingBody::finish(body, None);
    }
}
