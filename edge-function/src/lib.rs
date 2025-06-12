wit_bindgen::generate!({world: "edge-function", path: ".edgee/wit", generate_all});
export!(Component);
struct Component;

use exports::wasi::http::incoming_handler::{Guest, ResponseOutparam};
use wasi::http::types::{Fields, OutgoingBody, OutgoingResponse};

use wasi::http::outgoing_handler::OutgoingRequest;

use url::Url;

impl Guest for Component {
    fn handle(req: wasi::http::types::IncomingRequest, resp: wasi::http::types::ResponseOutparam) {
        //let incoming_headers = IncomingRequest::headers(&req);
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
        let url = Url::parse("https://example.com").unwrap();
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
