use colored::Colorize;
use reqwest::blocking::Client;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Check if a port is open on a host
pub fn check_port(host: &str, port: u16) {
    println!(
        "{} Checking {}:{}",
        "→".cyan(),
        host.yellow(),
        port.to_string().yellow()
    );

    let address = format!("{}:{}", host, port);
    match address.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
                    Ok(_) => println!(
                        "{} Port {} is OPEN",
                        "✓".green().bold(),
                        port.to_string().green()
                    ),
                    Err(_) => println!(
                        "{} Port {} is CLOSED",
                        "✗".red().bold(),
                        port.to_string().red()
                    ),
                }
            } else {
                eprintln!("{} Could not resolve address", "Error:".red().bold());
            }
        }
        Err(e) => eprintln!("{} Failed to resolve address: {}", "Error:".red().bold(), e),
    }
}

/// Ping a host (using HTTP request as fallback)
pub fn ping_host(host: &str) {
    println!("{} Pinging {}", "→".cyan(), host.yellow());

    let url = if host.starts_with("http://") || host.starts_with("https://") {
        host.to_string()
    } else {
        format!("http://{}", host)
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    match client.head(&url).send() {
        Ok(response) => {
            let status = response.status();
            println!("{} Host is reachable", "✓".green().bold());
            println!("  Status: {}", status.to_string().green());
        }
        Err(e) => {
            eprintln!("{} Host is unreachable", "✗".red().bold());
            eprintln!("  Error: {}", e.to_string().yellow());
        }
    }
}

/// Get public IP address
pub fn get_public_ip() {
    println!("{} Fetching public IP address...", "→".cyan());

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    match client.get("https://api.ipify.org").send() {
        Ok(response) => match response.text() {
            Ok(ip) => println!("{} Your public IP: {}", "✓".green().bold(), ip.green()),
            Err(e) => eprintln!("{} Failed to read response: {}", "Error:".red().bold(), e),
        },
        Err(e) => eprintln!("{} Failed to fetch IP: {}", "Error:".red().bold(), e),
    }
}

/// Perform HTTP GET request
pub fn http_get(url: &str) {
    println!("{} GET {}", "→".cyan(), url.yellow());

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    match client.get(url).send() {
        Ok(response) => {
            let status = response.status();
            println!(
                "\n{} {}",
                "Status:".cyan().bold(),
                status.to_string().green()
            );

            // Print headers
            println!("\n{}", "Headers:".cyan().bold());
            for (key, value) in response.headers() {
                println!(
                    "  {}: {}",
                    key.as_str().yellow(),
                    value.to_str().unwrap_or("(binary)")
                );
            }

            // Print body
            match response.text() {
                Ok(body) => {
                    println!("\n{}", "Body:".cyan().bold());
                    let preview = if body.len() > 1000 {
                        format!("{}... (truncated)", &body[..1000])
                    } else {
                        body
                    };
                    println!("{}", preview.green());
                }
                Err(e) => eprintln!("{} Failed to read body: {}", "Error:".red().bold(), e),
            }
        }
        Err(e) => eprintln!("{} Request failed: {}", "Error:".red().bold(), e),
    }
}

/// Perform HTTP POST request
pub fn http_post(url: &str, body: &str, content_type: Option<&str>) {
    println!("{} POST {}", "→".cyan(), url.yellow());

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let mut request = client.post(url);

    if let Some(ct) = content_type {
        request = request.header("Content-Type", ct);
    } else {
        request = request.header("Content-Type", "application/json");
    }

    request = request.body(body.to_string());

    match request.send() {
        Ok(response) => {
            let status = response.status();
            println!(
                "\n{} {}",
                "Status:".cyan().bold(),
                status.to_string().green()
            );

            // Print headers
            println!("\n{}", "Headers:".cyan().bold());
            for (key, value) in response.headers() {
                println!(
                    "  {}: {}",
                    key.as_str().yellow(),
                    value.to_str().unwrap_or("(binary)")
                );
            }

            // Print body
            match response.text() {
                Ok(body) => {
                    println!("\n{}", "Body:".cyan().bold());
                    println!("{}", body.green());
                }
                Err(e) => eprintln!("{} Failed to read body: {}", "Error:".red().bold(), e),
            }
        }
        Err(e) => eprintln!("{} Request failed: {}", "Error:".red().bold(), e),
    }
}

/// DNS lookup
pub fn dns_lookup(hostname: &str) {
    println!("{} Looking up {}", "→".cyan(), hostname.yellow());

    match format!("{}:80", hostname).to_socket_addrs() {
        Ok(addrs) => {
            println!("\n{}", "IP Addresses:".cyan().bold());
            for addr in addrs {
                println!("  {}", addr.ip().to_string().green());
            }
        }
        Err(e) => eprintln!("{} DNS lookup failed: {}", "Error:".red().bold(), e),
    }
}
