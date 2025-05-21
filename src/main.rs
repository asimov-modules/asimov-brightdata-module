// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<clientele::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_brightdata_module::api::{Dataset, ScrapeInput, ScrapeRequest};
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
    let urls: Vec<ScrapeInput> = args
        .iter()
        .skip(1)
        .map(|arg| ScrapeInput::from_str(&arg.to_string_lossy()).unwrap())
        .collect();
    if urls.is_empty() {
        return Ok(SysexitsError::EX_OK);
    }

    // Obtain the Bright Data API key from the environment:
    let api_key = SecretString::from(std::env::var("BRIGHTDATA_API_KEY")?);
    let dataset_id = "gd_l1viktl72bvl7bjuj0"; // TODO: this is for LinkedIn only
    let dataset = Dataset::new(dataset_id, api_key);

    // Send the request and block while waiting for the response:
    let request = ScrapeRequest::from(urls);
    let response = dataset.scrape(&request)?;
    println!("{}", response);

    Ok(SysexitsError::EX_OK)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-brightdata-importer requires the 'std' feature")
}
