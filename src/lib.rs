// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

pub mod api;

mod datasets;

pub fn find_dataset_for(url: impl AsRef<str>) -> Option<&'static str> {
    let url = url.as_ref();
    for (url_prefix, dataset_id) in datasets::URL_PREFIX_TO_DATASET_ID {
        if url.starts_with(url_prefix) {
            return Some(dataset_id);
        }
    }
    None // not found
}
