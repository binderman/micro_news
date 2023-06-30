# Micro News - C# Server

This is the C# implementation of the Micro News server. It fetches news from the [NewsAPI](https://newsapi.org/) and provides a simple API for frontends to consume.

## Prerequisites

Ensure you have .NET Core 3.1 or later installed on your machine. You can verify your .NET version using the following command:

```bash
dotnet --version
```

## Installation

Follow the steps below to install all the requirements needed to run the server:

1. Navigate to the C# server directory in your terminal.

```bash
cd path_to_your_directory/micro_news/csharp_server
```

2. Restore the necessary .NET packages using the .NET CLI.

```bash
dotnet restore
```

## Running the Server

To start the server, run the following command from the C# server directory:

```bash
dotnet run
```

The server will start and listen for incoming connections.

## Testing in the Browser

With the server running, you can test it by opening a web browser and navigating to the server's URL: `http://localhost:8080/api/getTopHeadlines`.
