# HTTP client

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
$ wasmedge target/wasm32-wasi/release/client-with-wasm.wasm
Response: 200 OK
Body: b"{\n  \"args\": {}, \n  \"headers\": {\n    \"Host\": \"httpbin.org\", \n    \"X-Amzn-Trace-Id\": \"Root=1-67b86d6b-24defe4f3fa641f60440076d\"\n  }, \n  \"origin\": \"87.223.254.123\", \n  \"url\": \"http://httpbin.org/get\"\n}\n"
```

## Build and publish the image

The `Containerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
We just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ podman build --platform wasi/wasm -t client-with-wasm .
```

Then we can run it:
```
$ podman --runtime /usr/bin/crun-wasm run -dp 8080:8080 --platform=wasi/wasm -t --rm localhost/client-with-wasm
Response: 200 OK
Body: b"{\n  \"args\": {}, \n  \"headers\": {\n    \"Host\": \"httpbin.org\", \n    \"X-Amzn-Trace-Id\": \"Root=1-67b86d6b-24defe4f3fa641f60440076d\"\n  }, \n  \"origin\": \"87.223.254.123\", \n  \"url\": \"http://httpbin.org/get\"\n}\n"
```

