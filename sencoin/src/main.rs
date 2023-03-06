#![forbid(unsafe_code)]

use clap::Parser;
use sencoin::Cli;
use std::process;

/// entry point of the `sencoin` command line program.
///
/// ensure not to use `#[tokio::main]`
fn main() {
    let cli = Cli::parse();
    if let Err(err) = cli.execute() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
