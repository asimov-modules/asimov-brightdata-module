// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use asimov_module::prelude::{Result, String, Vec};
use core::str::FromStr;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct ScrapeRequest {
    pub input: Vec<ScrapeInput>,

    #[serde(
        serialize_with = "serialize_custom_output_fields",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub custom_output_fields: Vec<String>,
}

impl From<Vec<ScrapeInput>> for ScrapeRequest {
    fn from(input: Vec<ScrapeInput>) -> Self {
        Self {
            input,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ScrapeInput {
    pub url: String,
}

impl FromStr for ScrapeInput {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(ScrapeInput {
            url: String::from(input),
        })
    }
}

pub(crate) fn serialize_custom_output_fields<S>(t: &[String], s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(&t.join("|"))
}
