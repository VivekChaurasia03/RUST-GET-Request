# Rust HTTP GET Request CLI Tool

Welcome to the Rust HTTP GET Request CLI tool! This project is a simple command-line tool built in Rust, demonstrating how to make a basic GET request using the "Reqwest" crate.

## How to Use

To use the tool, clone the repository and run the following command:

```bash
cargo run
```

## Code Overview

The project uses the "Reqwest" crate for handling HTTP requests. In the `main` function, a GET request is made to the URL "http://httpbin.org/get," and the response is printed to the terminal.

## Dependencies

The primary dependency for this project is the Reqwest crate, which simplifies making HTTP requests in Rust.

## Example Output

The tool will print the HTTP status, headers, and body of the response, providing insights into the server's response.

```plaintext
Status: 200 OK
Headers:
{
    "access-control-allow-credentials": "true",
    "access-control-allow-origin": "*",
    "content-length": "210",
    "content-type": "application/json",
    "date": "Thu, 01 Jan 1970 00:00:00 GMT",
    "server": "gunicorn/19.9.0",
    "via": "1.1 vegur"
}
Body:
{
  "args": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin.org",
    "User-Agent": "reqwest/0.11.6"
  },
  "origin": "x.x.x.x",
  "url": "http://httpbin.org/get"
}
```

## Observations

This tool showcases how to perform a basic GET request in Rust using the Reqwest crate. Feel free to explore and modify the code to integrate more complex features or handle different HTTP methods. Happy coding!
