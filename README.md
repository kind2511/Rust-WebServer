# Lab 10: Hello JSON in Rust

# Hello JSON in Rust

The task is to build a simple webserver serving some simple API points, and handling JSON input/output.

**Exercise: Concurrent Web Server in Rust**

**Objective:** Build a concurrent web server in Rust that can handle multiple incoming requests simultaneously.

**Requirements:**
1. The web server should be able to handle HTTP GET and POST requests.
2. Implement a basic routing mechanism to serve different endpoints with appropriate responses:
   * GET "/hello" returns string: "Hello, World!" and 200 OK
   * GET "/" returns 404 page not found
   * GET "/greet/Mariusz" returns JSON `{ "greet": "Hello", "name": "Chris"}` and `200 OK`
      * Note, the URL path above is only an example, the last element of the URL path is a parameter, that will be used a the name.
      * Your code should work for "/greet/Adam" (or whatever different name) and return appropriately, based on the parameter passed into it.
   * POST "/greetme" with JSON input `{ "input": "whatever text", "name": "Chris"}` returns JSON `{"greet": "whatever text", "name": "Chris"}`
3. Utilize Rust's concurrency primitives, such as threads or async/await, to handle multiple incoming requests concurrently.
4. Ensure proper error handling for various scenarios, such as invalid requests or server errors.

**Challenges:**
1. Understanding Rust's ownership model and implementing it effectively to handle shared mutable state safely between threads.
2. Using Rust's concurrency primitives correctly to maximize performance and scalability.
3. Handling errors gracefully and efficiently, leveraging Rust's Result and Option types.
4. Implementing HTTP and JSON parsing and response generation using Rust libraries or building them from scratch.


## External crates to use

Use the following crates in your implementation:

0. Use **axum** crate for building the web server. Axum is a thin layer on top of 
low-level networking API for HTTP, based on **hyper**. 

1. **tokio:** This crate provides asynchronous I/O primitives, scheduler, and runtime for building highly concurrent applications in Rust. It's commonly used in conjunction with `hyper` or `axum` for implementing asynchronous web servers.

2. **serde:** This crate is essential for serializing and deserializing data structures in Rust. Students can use `serde` to parse incoming HTTP requests and generate appropriate responses.

3. **serde_json:** You will need to work with JSON data. Use this crate for serializing and deserializing JSON in Rust.

