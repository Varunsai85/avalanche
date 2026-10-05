use avalanche::cli::Cli;
use avalanche::run;
use clap::Parser;
use std::process;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(&cli) {
        eprintln!("error: {e}");
        process::exit(e.exit_code());
    }
}
