# HTTP server

## Prerequisites

- Podman
- Rust

## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ cargo run
Listening on http://0.0.0.0:8080
```

From another terminal window, do the following.

```
$ curl http://localhost:8080/
Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`

$ curl http://localhost:8080/echo -X POST -d "Hello WasmEdge"
Hello WasmEdge
```

## Build and publish on Docker

The `Containerfile` follows the above steps to build and package a lightweight OCI-compliant container image.
```
$ podman build -t server-without-wasm .
... ...
```

Then we can run it:
```
$ podman run -dp 8080:8080 --rm server-without-wasm
... ...

$ curl http://localhost:8080/
Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`

$ curl http://localhost:8080/echo -X POST -d "Hello WasmEdge"
Hello WasmEdge
```

