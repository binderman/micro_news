# Micro News - Java Server

This is the Java implementation of the Micro News server. It fetches news from the [NewsAPI](https://newsapi.org/) and provides a simple API for frontends to consume.

## Prerequisites

Ensure you have Java Development Kit (JDK) 8 or later installed on your machine. You can verify your JDK version using the following command:

```bash
java -version
```

## Installation

Follow the steps below to install all the requirements needed to run the server:

1. Navigate to the Java server directory in your terminal.

```bash
cd path_to_your_directory/micro_news/java_server
```

2. Compile the Java files in the directory. If you're using Maven as a build tool, you can do this with:

```bash
mvn clean install
```

## Running the Server

To start the server, run the following command from the Java server directory:

```bash
java -jar target/name_of_your_jar.jar
```

The server will start and listen for incoming connections.

## Testing in the Browser

With the server running, you can test it by opening a web browser and navigating to the server's URL: `http://localhost:8080/api/getTopHeadlines`.
