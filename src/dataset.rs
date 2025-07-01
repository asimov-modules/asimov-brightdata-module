// This is free and unencumbered software released into the public domain.

use crate::jq::JsonFilter;

#[cfg(feature = "std")]
#[derive(Clone, Debug)]
pub struct Dataset {
    pub id: &'static str,
    pub label: &'static str,
    pub jq_filter: fn() -> &'static JsonFilter,
}

#[cfg(not(feature = "std"))]
#[derive(Clone, Debug)]
pub struct Dataset {
    pub id: &'static str,
    pub label: &'static str,
    pub jq_filter: fn() -> JsonFilter,
}

#[cfg(feature = "std")]
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

#[cfg(not(feature = "std"))]
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
