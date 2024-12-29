# HTTP server

## Prerequisites

- Podman
- Rust
- [WasmEdge](https://wasmedge.org/) and [crun](https://github.com/containers/crun) support
```
> rpm-ostree install wasmedge crun-wasm
```

## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi --release
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge target/wasm32-wasi/release/server.wasm
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
$ docker buildx build --provenance=false --platform wasi/wasm -t secondstate/rust-example-server .
```

Then we can run it:
```
$ podman run -dp 8080:8080 --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm secondstate/rust-example-server:latest
... ...

$ curl http://localhost:8080/
Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`

$ curl http://localhost:8080/echo -X POST -d "Hello WasmEdge"
Hello WasmEdge
```

