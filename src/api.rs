// This is free and unencumbered software released into the public domain.

mod scrape;
pub use scrape::*;

mod trigger;
pub use trigger::*;

pub use asimov_module::secrecy::{ExposeSecret, SecretString};

use asimov_module::prelude::{Box, Result, String, format};
use core::error::Error;

#[derive(Debug)]
pub struct BrightData {
    pub(crate) api_key: SecretString,
}

impl BrightData {
    pub fn new(api_key: SecretString) -> Self {
        Self { api_key }
    }

    pub fn scrape_dataset(
        &self,
        dataset_id: impl AsRef<str>,
        request: &ScrapeRequest,
    ) -> Result<String, Box<dyn Error>> {
        let mut response = ureq::post("https://api.brightdata.com/datasets/v3/scrape")
            .query("dataset_id", dataset_id.as_ref())
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
