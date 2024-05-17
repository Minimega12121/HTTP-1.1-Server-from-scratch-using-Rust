# Description

This is a HTTP/1.1 protocol based server made from scratch using Rust. It is a functional HTTP server that can serve web pages with corresponding CSS and JavaScript files.

### Parsing Request

Parsing through the following request:

<img src="https://github.com/Minimega12121/HTTP-1.1-Server-from-scratch-using-Rust/blob/main/ss/get_req.png">

The underlying request logic parses the request with the help of the following request struct and its implementation for ParseError and generates the `Response struct` as follows:

<img src="https://github.com/Minimega12121/HTTP-1.1-Server-from-scratch-using-Rust/blob/main/ss/parsedata.png">

### Generating Response

Generates a response structure using a `new()` constructor that takes in the status code and the body corresponding to the request.

```rust
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}
```

### Generating Response
Generates a response structure using a new() constructure that takes in the status-code and the body corresponding to the request
`pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

### Making a website handler
This is responsible for serving different files from the /public directory depending on the type of request made by the client.

For GET requests:

Serves index.html for the root path /.
Serves hello.html for the path /hello.
Serves other files if they exist in the public directory.
Returns NotFound if the file doesn't exist.
