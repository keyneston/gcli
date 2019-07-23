extern crate reqwest;
extern crate serde_json;

use serde_json::value::Value;
use std::collections::HashMap;

pub struct Client {
    client: reqwest::Client,
    base: String,
}

impl Client {
    pub fn new(base_endpoint: &str) -> Client {
        return Client {
            client: reqwest::Client::new(),
            base: base_endpoint.to_string(),
        };
    }

    pub fn get_schema(&self) {
        let mut req = HashMap::new();

        req.insert(
            "query",
            "query {
            search(needle: \"*\") {
              id
              label
            }
        }",
        );

        let mut resp = self.client.post(&self.base).json(&req).send().unwrap();

        let resp_body: HashMap<String, Value> = resp.json().unwrap();

        println!("{:#?}", resp);
        println!("{:#?}", resp_body);
    }
}
