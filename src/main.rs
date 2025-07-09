// src/main.rs
/*
 * Main executable for ErcSmartContractVerifier
 */

use clap::Parser;
use ercsmartcontractverifier::{Result, run};

#[derive(Parser)]
#[command(version, about = "ErcSmartContractVerifier - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
