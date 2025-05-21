// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_brightdata_module::{
        api::{BrightData, ScrapeInput, ScrapeRequest},
        find_dataset_for,
    };
    use clientele::SysexitsError;
    use secrecy::SecretString;
    use std::str::FromStr;

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
        return Ok(SysexitsError::EX_OK);
    }

    // Obtain the Bright Data API key from the environment:
    let api_key = SecretString::from(std::env::var("BRIGHTDATA_API_KEY")?);
    let api = BrightData::new(api_key);

    // Scrape each of the given URL arguments:
    for url in urls {
        // Find the appropriate dataset ID based on the URL prefix:
        let Some(dataset_id) = find_dataset_for(&url) else {
            return Ok(SysexitsError::EX_UNAVAILABLE); // not supported
        };

        // Send the request and block while waiting for the response:
        let request = ScrapeRequest::from(vec![ScrapeInput::from_str(&url).unwrap()]);
        let response = api.scrape_dataset(dataset_id, &request)?;
        println!("{}", response);
    }

    Ok(SysexitsError::EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-brightdata-importer requires the 'std' feature")
}
