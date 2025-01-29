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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::exports::edgee::protocols::data_collection::{
        Campaign, Client, Context, Data, EventType, PageData, Session, TrackData, UserData,
    };
    use exports::edgee::protocols::data_collection::Consent;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    fn sample_user_data(edgee_id: String) -> UserData {
        UserData {
            user_id: "123".to_string(),
            anonymous_id: "456".to_string(),
            edgee_id,
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
            ],
        }
    }

    fn sample_context(edgee_id: String, locale: String, session_start: bool) -> Context {
        Context {
            page: sample_page_data(),
            user: sample_user_data(edgee_id),
            client: Client {
                city: "Paris".to_string(),
                ip: "192.168.0.1".to_string(),
                locale,
                timezone: "CET".to_string(),
                user_agent: "Chrome".to_string(),
                user_agent_architecture: "fuck knows".to_string(),
                user_agent_bitness: "64".to_string(),
                user_agent_full_version_list: "abc".to_string(),
                user_agent_version_list: "abc".to_string(),
                user_agent_mobile: "mobile".to_string(),
                user_agent_model: "don't know".to_string(),
                os_name: "MacOS".to_string(),
                os_version: "latest".to_string(),
                screen_width: 1024,
                screen_height: 768,
                screen_density: 2.0,
                continent: "Europe".to_string(),
                country_code: "FR".to_string(),
                country_name: "France".to_string(),
                region: "West Europe".to_string(),
            },
            campaign: Campaign {
                name: "random".to_string(),
                source: "random".to_string(),
                medium: "random".to_string(),
                term: "random".to_string(),
                content: "random".to_string(),
                creative_format: "random".to_string(),
                marketing_tactic: "random".to_string(),
            },
            session: Session {
                session_id: "random".to_string(),
                previous_session_id: "random".to_string(),
                session_count: 2,
                session_start,
                first_seen: 123,
                last_seen: 123,
            },
        }
    }

    fn sample_page_data() -> PageData {
        PageData {
            name: "page name".to_string(),
            category: "category".to_string(),
            keywords: vec!["value1".to_string(), "value2".into()],
            title: "page title".to_string(),
            url: "https://example.com/full-url?test=1".to_string(),
            path: "/full-path".to_string(),
            search: "?test=1".to_string(),
            referrer: "https://example.com/another-page".to_string(),
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ],
        }
    }

    fn sample_page_event(
        consent: Option<Consent>,
        edgee_id: String,
        locale: String,
        session_start: bool,
    ) -> Event {
        Event {
            uuid: Uuid::new_v4().to_string(),
            timestamp: 123,
            timestamp_millis: 123,
            timestamp_micros: 123,
            event_type: EventType::Page,
            data: Data::Page(sample_page_data()),
            context: sample_context(edgee_id, locale, session_start),
            consent,
        }
    }

    fn sample_track_data(event_name: String) -> TrackData {
        TrackData {
            name: event_name,
            products: vec![
                vec![("sku".to_string(), "SKU_12345".to_string())],
                vec![("name".to_string(), "Stan and Friends Tee".to_string())],
                vec![(
                    "affiliation".to_string(),
                    "Google Merchandise Store".to_string(),
                )],
                vec![("coupon".to_string(), "SUMMER_FUN".to_string())],
                vec![("discount".to_string(), "2.22".to_string())],
                vec![("index".to_string(), "0".to_string())],
                vec![("brand".to_string(), "Google".to_string())],
                vec![("category".to_string(), "Apparel".to_string())],
                vec![("category2".to_string(), "Adult".to_string())],
                vec![("category3".to_string(), "Shirts".to_string())],
                vec![("category4".to_string(), "Crew".to_string())],
                vec![("category5".to_string(), "Short sleeve".to_string())],
                vec![("list_id".to_string(), "related_products".to_string())],
                vec![("list_name".to_string(), "Related Products".to_string())],
                vec![("variant".to_string(), "green".to_string())],
                vec![(
                    "location_id".to_string(),
                    "ChIJIQBpAG2ahYAR_6128GcTUEo".to_string(),
                )],
                vec![("price".to_string(), "10.1".to_string())],
                vec![("quantity".to_string(), "3".to_string())],
                vec![("custom-property".to_string(), "whatever".to_string())],
            ],
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ],
        }
    }

    fn sample_track_event(
        event_name: String,
        consent: Option<Consent>,
        edgee_id: String,
        locale: String,
        session_start: bool,
    ) -> Event {
        Event {
            uuid: Uuid::new_v4().to_string(),
            timestamp: 123,
            timestamp_millis: 123,
            timestamp_micros: 123,
            event_type: EventType::Track,
            data: Data::Track(sample_track_data(event_name)),
            context: sample_context(edgee_id, locale, session_start),
            consent,
        }
    }

    fn sample_user_event(
        consent: Option<Consent>,
        edgee_id: String,
        locale: String,
        session_start: bool,
    ) -> Event {
        Event {
            uuid: Uuid::new_v4().to_string(),
            timestamp: 123,
            timestamp_millis: 123,
            timestamp_micros: 123,
            event_type: EventType::User,
            data: Data::User(sample_user_data(edgee_id.clone())),
            context: sample_context(edgee_id, locale, session_start),
            consent,
        }
    }

    fn sample_credentials() -> Vec<(String, String)> {
        vec![("ga_measurement_id".to_string(), "abc".to_string())]
    }

    #[test]
    fn page_works_fine() {
        let event = sample_page_event(
            Some(Consent::Granted),
            "abc".to_string(),
            "fr".to_string(),
            true,
        );
        let credentials = vec![("your-credentials".to_string(), "abc".to_string())];
        let result = Component::page(event, credentials);

        assert_eq!(result.is_err(), false);
        let edgee_request = result.unwrap();
        assert_eq!(edgee_request.method, HttpMethod::Post);
        assert_eq!(edgee_request.body.is_empty(), true);
        assert_eq!(edgee_request.url.starts_with("https://example.com/"), true);
        // add more checks (headers, querystring, etc.)
    }
}
