# HTTP server

## Prerequisites

- Podman
- Rust
- [WasmEdge](https://wasmedge.org/) and [crun](https://github.com/containers/crun) support
```
$ rpm-ostree install wasmedge crun-wasm
```

## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ rustup target add wasm32-wasi
$ cargo build --target wasm32-wasi --release
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge target/wasm32-wasi/release/server-with-wasm.wasm
Listening on http://0.0.0.0:8080
```

From another terminal window, do the following.

```
$ curl http://localhost:8080/
Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`

$ curl http://localhost:8080/echo -X POST -d "Hello WasmEdge"
Hello WasmEdge
```

## Build and publish the image

The `Containerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
We just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ podman build --platform wasi/wasm -t server-with-wasm .
```

Then we can run it:
```
$ podman --runtime /usr/bin/crun-wasm run -dp 8080:8080 --platform=wasi/wasm -t --rm localhost/server-with-wasm
... ...

$ curl http://localhost:8080/
Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`

$ curl http://localhost:8080/echo -X POST -d "Hello WasmEdge"
Hello WasmEdge
```

