use crate::exports::edgee::protocols::data_collection::{Dict, EdgeeRequest, Event, HttpMethod};
use exports::edgee::protocols::data_collection::Guest;

wit_bindgen::generate!({world: "data-collection", path: "wit", generate_all});
export!(Component);

struct Component;

impl Guest for Component {
    fn page(edgee_event: Event, cred_map: Dict) -> Result<EdgeeRequest, String> {
        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: format!("https://example.com/page"),
            headers: Vec::new(),
            body: String::new(),
        })
    }

    fn track(edgee_event: Event, cred_map: Dict) -> Result<EdgeeRequest, String> {
        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: format!("https://example.com/track"),
            headers: Vec::new(),
            body: String::new(),
        })
    }

    fn user(_edgee_event: Event, _cred_map: Dict) -> Result<EdgeeRequest, String> {
        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: format!("https://example.com/user"),
            headers: Vec::new(),
            body: String::new(),
        })
    }
}
