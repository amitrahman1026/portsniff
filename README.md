# Portsniff

This Rust CLI Port Scanner is a lightweight, efficient tool designed to scan
open ports on a given IP address. Developed as a small personal project to learn
Rust and explore its features, this scanner utilizes Rust's concurrency model to
handle TCP connections in parallel.

## Prerequisites

Rust, Cargo

## Installation

To install the Rust CLI Port Scanner, follow these steps:

1. Clone the repository to your local machine:

```bash
git clone https://github.com/amitrahman1026/portsniff.git
cd portsniff
```

2. Build the project using Cargo:

```bash
cargo build --release
```

This generates a binary in `target/release/`.

## Usage

To use the port scanner, run the binary with the desired IP address and the
number of threads.

```bash
cargo run -- -t <threads> <IP address>
```

- <threads>: The number of threads to use for the scan. More threads can speed
  up the scan but might increase the load on your network.
- <IP address>: The target IP address to scan.

## Examples

Scan an IP address with the default number of threads (4):

```bash
cargo run -- 127.0.0.1
```

Scan an IP address using 10 threads:

```bash
cargo run -- -t 1000 127.0.0.1
```

## Why Tokio Runtime?
Previously, this project utilized OS threads for concurrency. However, as the
need arose to spawn a large number of threads for efficient port scanning,
switching to Tokio runtime became a more suitable choice. Tokio provides an
asynchronous runtime and futures-based concurrency model, which enables
efficient handling of thousands of concurrent tasks without the overhead of
managing OS threads manually.

## Contributing

As a learning project, contributions, suggestions, and feedback are welcome
especially for more idiomatic syntax.
