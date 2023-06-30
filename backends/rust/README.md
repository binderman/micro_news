# Micro News - Rust Server

This is the Rust implementation of the Micro News server. It fetches news from the [NewsAPI](https://newsapi.org/) and provides a simple API for frontends to consume.

## Prerequisites

Ensure you have Rust 1.31 or later installed on your machine. You can verify your Rust version using the following command:

```bash
rustc --version
```

## Installation

Follow the steps below to install all the requirements needed to run the server:

1. Navigate to the Rust server directory in your terminal.

```bash
cd path_to_your_directory/micro_news/rust_server
```

2. Build the project using Cargo, Rust's build system and package manager.

```bash
cargo build
```

## Running the Server

To start the server, run the following command from the Rust server directory:

```bash
cargo run
```

The server will start and listen for incoming connections.

## Testing in the Browser

With the server running, you can test it by opening a web browser and navigating to the server's URL: `http://localhost:8080/api/getTopHeadlines`.
