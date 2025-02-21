# Rust/Tokio Server

This directory contains an [Tokio](https://tokio.rs/) server running on Unikraft.

## Set Up

To run this example, [install Unikraft's companion command-line toolchain `kraft`](https://unikraft.org/docs/cli), clone this repository and `cd` into this directory.

## Run and Use

Use `kraft` to run the image and start a Unikraft instance:

```bash
$ kraft run --rm -p 8080:8080 --plat qemu --arch x86_64 -M 512M .
```

If the `--plat` argument is left out, it defaults to `qemu`.
If the `--arch` argument is left out, it defaults to your system's CPU architecture.

Once executed, it will open port `8080` and wait for connections.
To test it, you can use `curl`:

```bash
$ curl localhost:8080
Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`
```

## Inspect and Close

To list information about the Unikraft instance, use:

```bash
$ kraft ps
NAME           KERNEL                          ARGS     CREATED         STATUS   MEM   PORTS                   PLAT
hardcore_ivan  oci://unikraft.org/base:latest  /server  11 seconds ago  running  488M  0.0.0.0:8080->8080/tcp  qemu/x86_64
```

To close the Unikraft instance, close the `kraft` process (e.g., via `Ctrl+c`) or run:

```bash
kraft rm hardcore_ivan
```

Note that depending on how you modify this example your instance **may** need more memory to run.
To do so, use the `kraft run`'s `-M` flag, for example:

```bash
kraft run --rm -p 8080:8080 --plat qemu --arch x86_64 -M 1024M .
```

## Learn More

- [How to run unikernels locally](https://unikraft.org/docs/cli/running)
- [Building `Dockerfile` Images with `BuildKit`](https://unikraft.org/guides/building-dockerfile-images-with-buildkit)
