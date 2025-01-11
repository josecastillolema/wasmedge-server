# Rust examples for WasmEdge

Simple Rust HTTP server from the [WasmEdge Rust SDK examples](https://github.com/second-state/wasmedge-rustsdk-examples) modified to run with and without WasmEdge (and with [Unikraft](https://unikraft.org/) also).

## Code

The [`src/main.rs`](with-wasm/src/main.rs) source code shows how to start an async server using the `hyper` crate.

* The `main()` function is now an `async` function and annotated with the `tokio` macro. That means the tokio controller can spawn multiple instances of the `main()` app.
* Each instance of the `main()` app listens at port 8080 without blocking the port for everyone else. It receives a data `stream` for each incoming HTTP connection it captures.
  * The `accept()` function runs in a loop. It accepts an incoming connection, processes its data, and then start over.
  * Since the data connection could be slow, the `accept()` function could take a long time to return. But it is async, meaning that multiple instances of `accept()` could run concurrently to receive data from multiple connections on the same port.
  * The `await` blocks each instance of `accept()` so that the statements after `accept()` would not run until `accept()` receives all the data in a connection and returns.
* The `tokio::task::spawn` API designate the `handle_request()` function to be called whenever the data `stream` is available.
* The data in the `stream`, which is a HTTP request is passed to `handle_request()`, and the function returns an HTTP response struct.
* Inside `handle_request()`, it matches both the HTTP request method and path.
  * If the HTTP request is a GET at `/`, it returns a response with a help message.
  * If the HTTP request is a POST at `/echo`, it extracts the HTTP body and echoes it back as the HTTP response.
  * If no match is found, it returns an HTTP 404 error code.
