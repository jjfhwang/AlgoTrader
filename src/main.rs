// src/main.rs
/*
 * Main executable for AlgoTrader
 */

use clap::Parser;
use algotrader::{Result, run};

#[derive(Parser)]
#[command(version, about = "AlgoTrader - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
