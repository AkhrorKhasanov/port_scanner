use std::net::TcpStream;
use crate::cli::Args;
use crate::utils;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use::time::{timeout, Duration};

pub async fn run_scanner(args: Args) {
    match utils::parse_port_range(&args.port_range) {
        Ok((start, end)) => {
            println!("Scanning started: ports {}-{}", start, end);

            let semaphore = Arc::new(Semaphore::new(args.threads));
            let mut tasks = Vec::new();

            for port in start..=end {
                let ip = args.ip.clone();
                let permit = semaphore.clone().acquire_owned().await().unwrap();
                let timeout_dur = Duration::from_secs(args.timeout);

                let task = tokio::spawn(async move {
                    let _permit = permit;

                    let address = format!("{}:{}", ip, port);

                    if let Ok(Ok(_)) = timeout(timeout_dur, TcpStream::connect(&address)).await {
                        println!("Port {} is OPEN", port);
                    }
                });
                tasks.push(task);
            }
            for task in tasks {
                let _ = task.await;
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
