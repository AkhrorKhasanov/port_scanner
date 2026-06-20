mod cli;
mod scanner;
mod utils;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = cli::Args::parse();

    scanner::run_scanner(args).await;
}
