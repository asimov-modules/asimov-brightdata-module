// This is free and unencumbered software released into the public domain.

pub use ::jq::*;

#[cfg(feature = "std")]
pub fn default_filter() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| ". | select(false)".parse().unwrap()) // Returns empty output
}

#[cfg(not(feature = "std"))]
pub fn default_filter() -> JsonFilter {
    ". | select(false)".parse().unwrap() // Returns empty output
}

#[cfg(feature = "std")]
pub fn airbnb_rooms() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/airbnb_rooms.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn airbnb_rooms() -> JsonFilter {
    include_str!("jq/airbnb_rooms.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn yahoo_finance() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/yahoo_finance.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn yahoo_finance() -> JsonFilter {
    include_str!("jq/yahoo_finance.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn amazon() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/amazon.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn amazon() -> JsonFilter {
    include_str!("jq/amazon.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn amazon_seller() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/amazon_seller.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn amazon_seller() -> JsonFilter {
    include_str!("jq/amazon_seller.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn crunchbase_org() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/crunchbase_org.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn crunchbase_org() -> JsonFilter {
    include_str!("jq/crunchbase_org.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn ebay_item() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/ebay_item.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn ebay_item() -> JsonFilter {
    include_str!("jq/ebay_item.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn facebook_events() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/facebook_events.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn facebook_events() -> JsonFilter {
    include_str!("jq/facebook_events.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn facebook_groups() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/facebook_groups.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn facebook_groups() -> JsonFilter {
    include_str!("jq/facebook_groups.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn facebook_marketplace_item() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/facebook_marketplace_item.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn facebook_marketplace_item() -> JsonFilter {
    include_str!("jq/facebook_marketplace_item.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn facebook_share_p() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/facebook_share_p.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn facebook_share_p() -> JsonFilter {
    include_str!("jq/facebook_share_p.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn google_shopping_product() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/google_shopping_product.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn google_shopping_product() -> JsonFilter {
    include_str!("jq/google_shopping_product.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn indeed_cmp() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/indeed_cmp.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn indeed_cmp() -> JsonFilter {
    include_str!("jq/indeed_cmp.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn instagram_profile() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/instagram_profile.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn instagram_profile() -> JsonFilter {
    include_str!("jq/instagram_profile.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn instagram_p() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/instagram_p.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn instagram_p() -> JsonFilter {
    include_str!("jq/instagram_p.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn instagram_reel() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/instagram_reel.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn instagram_reel() -> JsonFilter {
    include_str!("jq/instagram_reel.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn linkedin_company() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/linkedin_company.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn linkedin_company() -> JsonFilter {
    include_str!("jq/linkedin_company.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn linkedin_profile() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/linkedin_profile.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn linkedin_profile() -> JsonFilter {
    include_str!("jq/linkedin_profile.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn linkedin_jobs() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/linkedin_jobs.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn linkedin_jobs() -> JsonFilter {
    include_str!("jq/linkedin_jobs.jq").parse().unwrap()
}


#[cfg(feature = "std")]
pub fn linkedin_posts() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/linkedin_posts.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn linkedin_posts() -> JsonFilter {
    include_str!("jq/linkedin_posts.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn linkedin_pulse() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/linkedin_pulse.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn linkedin_pulse() -> JsonFilter {
    include_str!("jq/linkedin_pulse.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn walmart_seller() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/walmart_seller.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn walmart_seller() -> JsonFilter {
    include_str!("jq/walmart_seller.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn walmart_ip() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/walmart_ip.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn walmart_ip() -> JsonFilter {
    include_str!("jq/walmart_ip.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn youtube_profile() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/youtube_profile.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn youtube_profile() -> JsonFilter {
    include_str!("jq/youtube_profile.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn youtube_watch() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/youtube_watch.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn youtube_watch() -> JsonFilter {
    include_str!("jq/youtube_watch.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn x_profile() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/x_profile.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn x_profile() -> JsonFilter {
    include_str!("jq/x_profile.jq").parse().unwrap()
}

#[cfg(feature = "std")]
pub fn x_status() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| include_str!("jq/x_status.jq").parse().unwrap())
}

#[cfg(not(feature = "std"))]
pub fn x_status() -> JsonFilter {
    include_str!("jq/x_status.jq").parse().unwrap()
}
