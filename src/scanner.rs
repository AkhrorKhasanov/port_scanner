use crate::cli::Args;
use crate::utils;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::time::{Duration, timeout};
pub async fn run_scanner(args: Args) {
    match utils::parse_port_range(&args.port_range) {
        Ok((start, end)) => {
            println!("Scanning started: ports {}-{}", start, end);

            let semaphore = Arc::new(Semaphore::new(args.threads));
            let mut tasks = Vec::new();

            for port in start..=end {
                let ip = args.ip.clone();
                let permit = semaphore.clone().acquire_owned().await.unwrap();
                let timeout_dur = Duration::from_secs(args.timeout);

                let task = tokio::spawn(async move {
                    let _permit = permit;

                    let address = format!("{}:{}", ip, port);

                    if let Ok(stream) = timeout(timeout_dur, TcpStream::connect(&address)).await {
                        if let Ok(mut stream) = stream {
                            println!("Port {} is OPEN", port);

                            let _ = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await;

                            let mut buffer = [0; 1024];
                            let mut has_received_data = false;

                            if let Ok(Ok(n)) =
                                timeout(Duration::from_secs(1), stream.read(&mut buffer)).await
                            {
                                if n > 0 {
                                    println!(
                                        "  -> Banner: {}",
                                        String::from_utf8_lossy(&buffer[..n]).trim()
                                    );
                                    has_received_data = true;
                                }
                            }

                            if !has_received_data {
                                let _ = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await;
                                if let Ok(Ok(n)) =
                                    timeout(Duration::from_secs(1), stream.read(&mut buffer)).await
                                {
                                    if n > 0 {
                                        println!(
                                            "  -> HTTP Response: {}",
                                            String::from_utf8_lossy(&buffer[..n]).trim()
                                        );
                                    }
                                }
                            }
                            // if let Ok(n) =
                            //     timeout(Duration::from_secs(2), stream.read(&mut buffer)).await
                            // {
                            //     if let Ok(bytes_read) = n {
                            //         if bytes_read > 0 {
                            //             let banner = String::from_utf8_lossy(&buffer[..bytes_read]);
                            //             println!(" -> Data from {}: {}", port, banner.trim());
                            //         }
                            //     }
                            // }
                        }
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
