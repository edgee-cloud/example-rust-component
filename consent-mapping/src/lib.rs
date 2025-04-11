use exports::edgee::components::consent_mapping::Consent;
use exports::edgee::components::consent_mapping::Dict;
use exports::edgee::components::consent_mapping::Guest;

use std::collections::HashMap;

wit_bindgen::generate!({world: "consent-mapping", path: ".edgee/wit", generate_all});
export!(Component);

struct Component;

/*
* Implement the Guest trait for the Component struct
* to create the required functions for the consent mapping protocol
* for your provider.
* The map function is called by the Edgee platform to map the consent from the
* cookie and return the consent object.
* The cookie dict looks like this
* {
*   "cookie_key": "true",
*   "cookie_key_1": "examplevalue1|examplevalue2",
*   ...
* }
*/

impl Guest for Component {
    #[allow(unused_variables)]
    fn map(cookie: Dict, settings_dict: Dict) -> Option<Consent> {
        let my_cookies = MyCookies::new(cookie).unwrap();
        if let Some(cookie_key) = my_cookies.cookie_key {
            if cookie_key == "true" {
                return Some(Consent::Granted);
            } else if cookie_key == "false" {
                return Some(Consent::Denied);
            }
        }
        Some(Consent::Pending)
    }
}

/*
* {
*   "cookie_key": "true",
*   "cookie_key_1": "examplevalue1|examplevalue2",
*   ...
* }
*/
pub struct MyCookies {
    // the cookies you want to parse
    pub cookie_key: Option<String>,
    pub cookie_key_1: Option<String>,
}

impl MyCookies {
    pub fn new(cookies: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = cookies
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        // parse your cookies here
        let cookie_key = settings_map.get("cookie_key").cloned();
        let cookie_key_1 = settings_map.get("cookie_key_1").cloned();
        Ok(Self {
            cookie_key,
            cookie_key_1,
        })
    }
}

pub struct Settings {
    pub example: String,
}

impl Settings {
    pub fn new(settings_dict: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = settings_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        /*
        // required setting
        // also needs -> use anyhow::Context;
        let example = settings_map
            .get("example")
            .context("Missing example setting")?
            .to_string();
        */

        // optional setting
        let example = settings_map
            .get("example")
            .map(String::to_string)
            .unwrap_or_default();

        Ok(Self { example })
    }
}
