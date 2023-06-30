# Micro News - Node.js Server

This is the Node.js implementation of the Micro News server. It fetches news from the [NewsAPI](https://newsapi.org/) and provides a simple API for frontends to consume.

## Prerequisites

Ensure you have Node.js 10.0.0 or later installed on your machine. You can verify your Node.js version using the following command:

```bash
node --version
```

## Installation

Follow the steps below to install all the requirements needed to run the server:

1. Navigate to the Node.js server directory in your terminal.

```bash
cd path_to_your_directory/micro_news/node_server
```

2. Install the required Node.js packages using npm.

```bash
npm install
```

## Running the Server

To start the server, run the following command from the Node.js server directory:

```bash
node server.js
```

The server will start and listen for incoming connections.

## Testing in the Browser

With the server running, you can test it by opening a web browser and navigating to the server's URL: `http://localhost:8080/api/getTopHeadlines`.
