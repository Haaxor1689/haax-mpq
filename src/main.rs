mod mpq;
use clap::Parser;
use env_logger;
use log;
use tokio::runtime::Runtime;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Paths
    #[arg(num_args = 1..)]
    items: Vec<String>,
}

fn main() {
    env_logger::init();

    log::info!("[RUST] build_mpq started");
    let args = Args::parse();

    let input = match args.items.get(0) {
        Some(path) => path,
        None => {
            log::error!("No input file provided.");
            return;
        }
    };

    let output = match args.items.get(1) {
        Some(path) => path,
        None => {
            log::error!("No output file provided.");
            return;
        }
    };

    // Create a Tokio runtime
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    if input.ends_with(".mpq") || input.ends_with(".MPQ") {
        if output.ends_with(".mpq") || output.ends_with(".MPQ") {
            log::error!("Input and output cannot both be MPQ files.");
            return;
        }

        // Use block_on to execute the async function synchronously
        rt.block_on(async {
            // Create an async block to await your function
            if let Err(e) = mpq::extract(input, output).await {
                log::error!("Failed to extract mpq: {}", e);
            }
        });
    } else {
        if !(output.ends_with(".mpq") || output.ends_with(".MPQ")) {
            log::error!("Input and output cannot both be folders.");
            return;
        }

        // Use block_on to execute the async function synchronously
        rt.block_on(async {
            // Create an async block to await your function
            if let Err(e) = mpq::build(output, input).await {
                log::error!("Failed to build mpq: {}", e);
            }
        });
    }
}
