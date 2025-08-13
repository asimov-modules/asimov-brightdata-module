// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_brightdata_module::{
        api::{BrightData, ScrapeInput, ScrapeRequest},
        find_dataset_for,
    };
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

    let manifest = match asimov_module::ModuleManifest::read_manifest("brightdata") {
        Ok(manifest) => manifest,
        Err(e) => {
            eprintln!("failed to read module manifest: {e}");
            return Ok(EX_CONFIG);
        }
    };

    // Obtain the Bright Data API key from the environment:
    let api_key = match manifest.variable("brightdata-api-key", None) {
        Ok(api_key) => api_key,
        Err(e) => {
            eprintln!("failed to get API key: {e}");
            return Ok(EX_CONFIG); // not configured
        }
    };
    let api = BrightData::new(api_key.into());

    // Process each of the given URL arguments:
    for url in urls {
        // Find the appropriate dataset ID based on the URL prefix:
        let Some(dataset) = find_dataset_for(&url) else {
            return Ok(EX_UNAVAILABLE); // not supported
        };

        // Send the request and block while waiting for the response:
        let request = ScrapeRequest::from(vec![ScrapeInput::from_str(&url).unwrap()]);
        let response = api.scrape_dataset(dataset.id, &request)?;

        // Serialize the response data:
        if cfg!(feature = "pretty") {
            let response_json: serde_json::Value = serde_json::from_str(&response)?;
            colored_json::write_colored_json(&response_json, &mut stdout())?;
            println!();
        } else {
            println!("{}", response);
        }
    }

    Ok(EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-brightdata-fetcher requires the 'std' feature")
}
