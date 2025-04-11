# Rust TCP Service

A simple TCP server written in Rust that listens on port 12321, echoes back received data, and logs connections to `/var/log/raptor6/connection.log`. It includes professional logging with rotation and readable timestamps.

## Features
- Listens on TCP port 12321.
- Echoes back any data received from clients.
- Logs successful, closed, and failed connections with timestamps.
- Supports log rotation (10MB per file, keeps 5 files, rotates daily).

## Prerequisites
- Rust (install via `rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- `nc` (Netcat) for testing (`sudo apt install netcat`)
  
## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/spxd6666/raptor6
2. Compiling via cargo:
   ```bash
   cd raptor6
   cargo build --release
   
