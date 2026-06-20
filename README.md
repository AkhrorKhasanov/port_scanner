Rust Port Scanner
This is a high-performance, asynchronous TCP port scanner written in Rust. It utilizes the tokio runtime to perform concurrent port scanning, allowing it to efficiently scan thousands of ports while providing service fingerprinting (banner grabbing) capabilities.

Features
Asynchronous Architecture: Built with tokio for high-concurrency scanning.

Resource Control: Uses Semaphore to limit the number of concurrent connections, preventing system overload.

Banner Grabbing: Attempts to retrieve service information (e.g., version, server software) from open ports.

Highly Configurable: Manage scan timeouts and thread counts via CLI arguments.

Installation
To get started with the project, ensure you have Rust installed.

Clone the repository:

Bash
git clone <repo-link>
cd port_scanner
Build the project:

Bash
cargo build --release
Usage
Run the scanner using the following command structure:

Bash
cargo run -- --ip <IP_ADDRESS> --port-range <START-END> --threads <NUMBER> --timeout <SECONDS>
Example:
Bash
# Scan 127.0.0.1 from port 1 to 1000 with 50 concurrent threads
cargo run -- --ip 127.0.0.1 --port-range 1-1000 --threads 50 --timeout 2
Arguments:
--ip: The target IP address to scan.

--port-range: The range of ports to scan (e.g., 20-1000).

--threads: Maximum number of concurrent tasks (suggested: 50-100).

--timeout: Connection timeout in seconds.

Technical Implementation
CLI: Built using the clap library for robust argument parsing.

Async Runtime: Utilizes tokio::spawn to manage thousands of concurrent tasks efficiently.

Rate Limiting: Implements tokio::sync::Semaphore to regulate outbound traffic and maintain system stability.

Banner Grabbing Logic: 1. Passive: Immediately attempts to read data once a connection is established.
2. Active: Sends a HEAD / HTTP/1.0 request if no initial data is received, helping identify HTTP-based services.

Security Warning
This tool is intended for educational purposes and authorized security testing of your own networks only. Unauthorized scanning of networks you do not own may be illegal or against service terms.
