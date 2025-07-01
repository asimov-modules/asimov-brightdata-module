// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_brightdata_module::{
        api::{BrightData, ScrapeInput, ScrapeRequest},
        find_dataset_for, jq,
    };
    use asimov_module::getenv;
    use clientele::SysexitsError::*;
    use std::{io::stdout, str::FromStr};

    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Configure logging:
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    // Parse URLs from command-line arguments:
    let urls: Vec<String> = args
        .iter()
        .skip(1)
        .map(|arg| String::from_str(&arg.to_string_lossy()).unwrap())
        .collect();
    if urls.is_empty() {
        return Ok(EX_OK);
    }

    // Obtain the Bright Data API key from the environment:
    let Some(api_key) = getenv::var_secret("BRIGHTDATA_API_KEY") else {
        return Ok(EX_CONFIG); // not configured
    };
    let api = BrightData::new(api_key);

    // Process each of the given URL arguments:
    for url in urls {
        // Find the appropriate dataset ID based on the URL prefix:
        let Some(dataset) = find_dataset_for(&url) else {
            return Ok(EX_UNAVAILABLE); // not supported
        };

        // Send the request and block while waiting for the response:
        let request = ScrapeRequest::from(vec![ScrapeInput::from_str(&url).unwrap()]);
        let response = api.scrape_dataset(dataset.id, &request)?;
        let response = match dataset.id {
            "gd_ld7ll037kqy322v05" => jq::airbnb_rooms().filter_json_str(response)?,
            "gd_l7q7dkf244hwjntr0" => jq::amazon().filter_json_str(response)?,
            "gd_lhotzucw1etoe5iw1k" => jq::amazon_seller().filter_json_str(response)?,
            "gd_l1vijqt9jfj7olije" => jq::crunchbase_org().filter_json_str(response)?,
            "gd_ltr9mjt81n0zzdk1fb" => jq::ebay_item().filter_json_str(response)?,
            "gd_m14sd0to1jz48ppm51" => jq::facebook_events().filter_json_str(response)?,
            "gd_lz11l67o2cb3r0lkj3" => jq::facebook_groups().filter_json_str(response)?,
            "gd_lvt9iwuh6fbcwmx1a" => jq::facebook_marketplace_item().filter_json_str(response)?,
            "gd_lyclm1571iy3mv57zw" => jq::facebook_share_p().filter_json_str(response)?,
            "gd_ltppk50q18kdw67omz" => jq::google_shopping_product().filter_json_str(response)?,
            "gd_l7qekxkv2i7ve6hx1s" => jq::indeed_cmp().filter_json_str(response)?,
            "gd_lk5ns7kz21pck8jpis" => jq::instagram_p().filter_json_str(response)?,
            "gd_l1vikfch901nx3by4" => jq::instagram_profile().filter_json_str(response)?,
            "gd_lyclm20il4r5helnj" => jq::instagram_reel().filter_json_str(response)?,
            "gd_l1vikfnt1wgvvqz95w" => jq::linkedin_company().filter_json_str(response)?,
            "gd_lpfll7v5hcqtkxl6l" => jq::linkedin_jobs().filter_json_str(response)?,
            "gd_lyy3tktm25m4avu764" => jq::linkedin_posts().filter_json_str(response)?,
            "gd_l1viktl72bvl7bjuj0" => jq::linkedin_profile().filter_json_str(response)?,
            "gd_l95fol7l1ru6rlo116" => jq::walmart_ip().filter_json_str(response)?,
            "gd_m7ke48w81ocyu4hhz0" => jq::walmart_seller().filter_json_str(response)?,
            "gd_lwxmeb2u1cniijd7t4" => jq::x_profile().filter_json_str(response)?,
            "gd_lwxkxvnf1cynvib9co" => jq::x_status().filter_json_str(response)?,
            "gd_lmrpz3vxmz972ghd7" => jq::yahoo_finance().filter_json_str(response)?,
            "gd_lk538t2k2p1k3oos71" => jq::youtube_profile().filter_json_str(response)?,
            "gd_lk56epmy2i5g7lzu0k" => jq::youtube_watch().filter_json_str(response)?,
            _ => {
                return Ok(EX_UNAVAILABLE); // not supported
            }
        };

        // Serialize the response data:
        if cfg!(feature = "pretty") {
            colored_json::write_colored_json(&response, &mut stdout())?;
            println!();
        } else {
            println!("{}", serde_json::to_string(&response).unwrap());
        }
    }

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-brightdata-importer requires the 'std' feature")
}
