# Micro News - React Frontend

This is the React implementation of the frontend for the Micro News server. It consumes the API provided by one of the servers (Python, Java, Node.js, Rust, C#) which fetches news from the [NewsAPI](https://newsapi.org/).

## Prerequisites

Ensure you have Node.js 10.0.0 or later installed on your machine. You can verify your Node.js version using the following command:

```bash
node --version
```

Ensure one of the Micro News servers (Python, Java, Node.js, Rust, C#) is running. Follow the instructions in the respective server's README to get it running.

## Installation

Follow the steps below to install all the requirements needed to run the frontend:

1. Navigate to the React frontend directory in your terminal.

```bash
cd path_to_your_directory/micro_news/react_frontend
```

2. Install the required Node.js packages using npm.

```bash
npm install
```

## Running the Frontend

To start the frontend, run the following command from the React frontend directory:

```bash
npm start
```

The frontend will start and connect to the running server.

## Testing in the Browser

With the frontend running, you can test it by opening a web browser and navigating to the frontend's URL: `http://localhost:3000`.
