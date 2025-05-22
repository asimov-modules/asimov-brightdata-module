// This is free and unencumbered software released into the public domain.

#![no_std]
#![forbid(unsafe_code)]

pub mod api;

mod dataset;
mod datasets;

use dataset::Dataset;

pub fn find_dataset_for(url: impl AsRef<str>) -> Option<&'static Dataset> {
    let url = url.as_ref();
    for (url_prefix, dataset) in datasets::URL_PREFIX_TO_DATASET.iter().rev() {
        if url.starts_with(url_prefix) {
            return Some(dataset);
        }
    }
    None // not found
}
