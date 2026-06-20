use clap::Parser;
#[derive(Parser, Debug)]
#[command(
    author = "Axror Hasanov. ahrorhasanov815@gmail.com",
    version = "1.0.0",
    about = "rscan - A fast, multi-threaded CLI port scanner written in Rust for network discovery and security auditing"
)]
pub struct Args {
    #[arg(short = 'i', long)]
    pub ip: String,
    #[arg(short = 'p', long, default_value = "1-1024")]
    pub port_range: String,
    #[arg(short = 't', long, default_value = "2")]
    pub timeout: u64,
    #[arg(short = 'f', long)]
    pub fast: bool,
    #[arg(short = 'n', long, default_value = "500")]
    pub threads: usize,
}
