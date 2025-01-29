use crate::exports::edgee::protocols::data_collection::{Dict, EdgeeRequest, Event, HttpMethod};
use exports::edgee::protocols::data_collection::Guest;

wit_bindgen::generate!({world: "data-collection", path: "wit", generate_all});
export!(Component);

struct Component;

/*
* Implement the Guest trait for the Component struct
* to create the required functions for the data collection protocol
* for your provider.
* The functions are page, track, and user.
* The page function is called when the page event is triggered.
* The track function is called when the track event is triggered.
* The user function is called when the user event is triggered.
* The functions should return an EdgeeRequest or an error message.
* The EdgeeRequest contains the method, url, headers, and body of the request.
*/

impl Guest for Component {
    fn page(_edgee_event: Event, _cred_map: Dict) -> Result<EdgeeRequest, String> {
        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: format!("https://example.com/{}", "page"),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Authorization".to_string(), "Bearer XYZ".to_string()),
            ],
            body: String::new(),
        })
    }

    fn track(_edgee_event: Event, _cred_map: Dict) -> Result<EdgeeRequest, String> {
        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: format!("https://example.com/{}", "track"),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Authorization".to_string(), "Bearer XYZ".to_string()),
            ],
            body: String::new(),
        })
    }

    fn user(_edgee_event: Event, _cred_map: Dict) -> Result<EdgeeRequest, String> {
        Ok(EdgeeRequest {
            method: HttpMethod::Post,
            url: format!("https://example.com/{}", "user"),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Authorization".to_string(), "Bearer XYZ".to_string()),
            ],
            body: String::new(),
        })
    }
}
