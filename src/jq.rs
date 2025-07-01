// This is free and unencumbered software released into the public domain.

pub use ::jq::*;

/// Macro to generate JSON filter functions for both std and no_std environments.
macro_rules! define_json_filter {
    ($($name:ident => $path:literal),* $(,)?) => {
        $(
            /// Returns a JSON filter for the specified data source.
            ///
            /// In `std` environments, the filter is lazily initialized and cached.
            /// In `no_std` environments, the filter is parsed on each call.
            ///
            /// # Panics
            /// Panics if the filter file at the specified path cannot be parsed.
            #[cfg(feature = "std")]
            pub fn $name() -> &'static JsonFilter {
                use std::sync::OnceLock;
                static ONCE: OnceLock<JsonFilter> = OnceLock::new();
                ONCE.get_or_init(|| include_str!($path).parse().unwrap())
            }

            /// Returns a JSON filter for the specified data source.
            ///
            /// In `no_std` environments, the filter is parsed on each call.
            ///
            /// # Panics
            /// Panics if the filter file at the specified path cannot be parsed.
            #[cfg(not(feature = "std"))]
            pub fn $name() -> JsonFilter {
                include_str!($path).parse().unwrap()
            }
        )*
    };
}

/// Default JSON filter that returns empty output.
#[cfg(feature = "std")]
pub fn default_filter() -> &'static JsonFilter {
    use std::sync::OnceLock;
    static ONCE: OnceLock<JsonFilter> = OnceLock::new();
    ONCE.get_or_init(|| ". | select(false)".parse().unwrap())
}

/// Default JSON filter that returns empty output.
#[cfg(not(feature = "std"))]
pub fn default_filter() -> JsonFilter {
    ". | select(false)".parse().unwrap()
}

// Define JSON filters using the macro.
define_json_filter! {
    airbnb_rooms => "jq/airbnb_rooms.jq",
    yahoo_finance => "jq/yahoo_finance.jq",
    amazon => "jq/amazon.jq",
    amazon_seller => "jq/amazon_seller.jq",
    crunchbase_org => "jq/crunchbase_org.jq",
    ebay_item => "jq/ebay_item.jq",
    facebook_events => "jq/facebook_events.jq",
    facebook_groups => "jq/facebook_groups.jq",
    facebook_marketplace_item => "jq/facebook_marketplace_item.jq",
    facebook_share_p => "jq/facebook_share_p.jq",
    google_shopping_product => "jq/google_shopping_product.jq",
    indeed_cmp => "jq/indeed_cmp.jq",
    instagram_profile => "jq/instagram_profile.jq",
    instagram_p => "jq/instagram_p.jq",
    instagram_reel => "jq/instagram_reel.jq",
    linkedin_company => "jq/linkedin_company.jq",
    linkedin_profile => "jq/linkedin_profile.jq",
    linkedin_jobs => "jq/linkedin_jobs.jq",
    linkedin_posts => "jq/linkedin_posts.jq",
    linkedin_pulse => "jq/linkedin_pulse.jq",
    walmart_seller => "jq/walmart_seller.jq",
    walmart_ip => "jq/walmart_ip.jq",
    youtube_profile => "jq/youtube_profile.jq",
    youtube_watch => "jq/youtube_watch.jq",
    x_profile => "jq/x_profile.jq",
    x_status => "jq/x_status.jq"
}
