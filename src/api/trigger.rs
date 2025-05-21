// This is free and unencumbered software released into the public domain.

#![allow(unused)]

use asimov_module::prelude::{String, Vec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize)]
pub struct TriggerRequest(pub Vec<TriggerInput>);

#[derive(Clone, Debug, Serialize)]
pub struct TriggerInput {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TriggerResponse {
    pub snapshot_id: String,
}
