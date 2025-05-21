// This is free and unencumbered software released into the public domain.

mod scrape;
pub use scrape::*;

mod trigger;
pub use trigger::*;

pub use secrecy::{ExposeSecret, SecretString};

use asimov_module::prelude::{Box, Result, String, format};
use core::error::Error;

pub struct Dataset {
    pub id: String,
    pub(crate) api_key: SecretString,
}

impl Dataset {
    pub fn new(id: impl AsRef<str>, api_key: SecretString) -> Self {
        Self {
            id: id.as_ref().into(),
            api_key,
        }
    }

    pub fn scrape(&self, request: &ScrapeRequest) -> Result<String, Box<dyn Error>> {
        let mut response = ureq::post("https://api.brightdata.com/datasets/v3/scrape")
            .query("dataset_id", &self.id)
            .query("include_errors", "true")
            .query("format", "json")
            .header(
                "Authorization",
                format!("Bearer {}", self.api_key.expose_secret()),
            )
            .send_json(request)?;
        let response_body = response.body_mut().read_to_string()?;
        Ok(response_body)
    }
}
