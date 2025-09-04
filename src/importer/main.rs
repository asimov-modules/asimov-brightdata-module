// This is free and unencumbered software released into the public domain.

use clap::Parser;
use clientele::StandardOptions;

/// asimov-brightdata-importer
#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// The output format.
    #[arg(value_name = "FORMAT", short = 'o', long)]
    output: Option<String>,

    urls: Vec<String>,
}

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
    let args = asimov_module::args_os()?;

    // Parse command-line options:
    let options = Options::parse_from(args);

    // Handle the `--version` flag:
    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    // Handle the `--license` flag:
    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    if options.urls.is_empty() {
        return Ok(EX_OK);
    }

    let Ok(manifest) = asimov_module::ModuleManifest::read_manifest("brightdata")
        .inspect_err(|e| eprintln!("failed to read module manifest: {e}"))
    else {
        return Ok(EX_CONFIG);
    };

    // Obtain the Bright Data API key from the environment:
    let Ok(api_key) = manifest
        .variable("brightdata-api-key", None)
        .inspect_err(|e| eprintln!("failed to get API key: {e}"))
    else {
        return Ok(EX_CONFIG); // not configured
    };
    let api = BrightData::new(api_key.into());

    // Process each of the given URL arguments:
    for url in options.urls {
        // Find the appropriate dataset ID based on the URL prefix:
        let Some(dataset) = find_dataset_for(&url) else {
            return Ok(EX_UNAVAILABLE); // not supported
        };

        // Send the request and block while waiting for the response:
        let request = ScrapeRequest::from(vec![ScrapeInput::from_str(&url).unwrap()]);
        let response = api.scrape_dataset(dataset.id, &request)?;
        let response = (dataset.jq_filter)().filter_json_str(response)?;

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
