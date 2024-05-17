# Description

This is a HTTP/1.1 protocol based server made from scratch using Rust. It is a functional HTTP server that can serve web pages with corresponding CSS and JavaScript files.

### Parsing Request
```rust
pub struct Request<'buf>{
    path : &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

```
- The request is simplified in the form of the above Request structure.

Parsing through the following request:

<img src="https://github.com/Minimega12121/HTTP-1.1-Server-from-scratch-using-Rust/blob/main/ss/get_req.png">

The underlying request logic parses the request with the help of the above Request struct and in case of any problem with parsing generates `ParseError` accordingly.

<img src="https://github.com/Minimega12121/HTTP-1.1-Server-from-scratch-using-Rust/blob/main/ss/parsedata.png">

### Generating Response

Generates a response structure using a `new()` constructor that takes in the status code and the body corresponding to the request.

```rust
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}
```

### Making a Website Handler

This is responsible for serving different files from the `/public` directory depending on the type of request made by the client.
It also checks for `Directory Traversal Attacks` and prevents it by logging the attempt and returning `Page Not Found: 404`
For GET requests:

- Serves `index.html` for the root path `/`.
- Serves `hello.html` for the path `/hello`.
- Serves other files if they exist in the public directory.
- Returns `NotFound` if the file doesn't exist.