# Rust TCP Service

A simple TCP server written in Rust that listens on port 6666, echoes back received data, and logs connections to `/var/log/arsh/conn.log`. It includes professional logging with rotation and readable timestamps.

## Features
- Listens on TCP port 6666.
- Echoes back any data received from clients.
- Logs successful, closed, and failed connections with timestamps.
- Supports log rotation (10MB per file, keeps 5 files, rotates daily).

## Prerequisites
- Rust (install via `rustup`: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- `nc` (Netcat) for testing (`sudo apt install netcat`)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/tcp_service.git
   cd tcp_service
