use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::path::Path;
use std::thread;
use log::{info, error, warn};
use simplelog::{Config, WriteLogger, LevelFilter, Rotation, SizeRotation};
use chrono::Utc;

// Initialize logging system
fn init_logging() {
    let log_dir = "/var/log/arsh";
    let log_file = "/var/log/arsh/conn.log";

    // Create log directory if it doesn't exist
    if !Path::new(log_dir).exists() {
        fs::create_dir_all(log_dir).expect("Failed to create log directory");
    }

    // Set up logging with rotation (max 10MB per file, keep 5 files, rotate daily)
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        fs::File::create(log_file).expect("Failed to create log file"),
        SizeRotation::new(10_000_000, 5, Rotation::DAILY),
    ).expect("Failed to initialize logger");
}

// Log connection details
fn log_connection(status: &str, details: &str) {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("[{}] {}: {}", timestamp, status, details);
    match status {
        "Established" => info!("{}", log_entry),
        "Closed" => warn!("{}", log_entry),
        "Failed" => error!("{}", log_entry),
        _ => info!("{}", log_entry), // Default case
    }
}

fn handle_client(mut stream: TcpStream) {
    // Log successful connection
    let peer_addr = stream.peer_addr().unwrap().to_string();
    log_connection("Established", &format!("Connection from {}", peer_addr));

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(n) if n > 0 => {
            stream.write_all(&buffer[..n]).unwrap();
            stream.flush().unwrap();
        }
        _ => {
            println!("Connection closed or error occurred");
            log_connection("Closed", &format!("Connection from {} closed or failed", peer_addr));
        }
    }
}

fn main() {
    // Initialize logging
    init_logging();

    let listener = TcpListener::bind("0.0.0.0:6666").expect("Failed to bind to port 6666");
    println!("Server listening on port 6666...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
                log_connection("Failed", &format!("Failed to accept connection: {}", e));
            }
        }
    }
}
