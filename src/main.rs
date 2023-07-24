use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The path to the protocol params JSON file
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut path = std::env::current_dir().expect("Error");
    path.push(&args.path);
    cardano_sandbox::print_utxo_cost_per_byte(path);
}
