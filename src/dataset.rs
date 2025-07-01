// This is free and unencumbered software released into the public domain.

use crate::jq::JsonFilter;

#[derive(Clone, Debug)]
pub struct Dataset {
    pub id: &'static str,
    pub label: &'static str,
    pub jq_filter: fn() -> &'static JsonFilter,
}

impl Default for Dataset {
    fn default() -> Self {
        use crate::jq;
        Self {
            id: "",
            label: "",
            jq_filter: jq::default_filter,
        }
    }
}
