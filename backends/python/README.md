# Micro News - Python Server

This is the Python implementation of the Micro News server. It fetches news from the [NewsAPI](https://newsapi.org/) and provides a simple API for frontends to consume.

## Prerequisites

Ensure you have Python 3.7 or later installed on your machine. You can verify your Python version using the following command:

```bash
python --version
```

## Installation

Follow the steps below to install all the requirements needed to run the server:

1. Navigate to the Python server directory in your terminal.

```bash
cd path_to_your_directory/micro_news/python_server
```

2. Install the required Python packages using pip. It is recommended to use a virtual environment.

```bash
pip install -r requirements.txt
```

## Running the Server

To start the server, run the following command from the Python server directory:

```bash
python server.py
```

The server will start and listen for incoming connections.

## Testing in the Browser

With the server running, you can test it by opening a web browser and navigating to the server's URL: `http://localhost:8080/api/getTopHeadlines`.
