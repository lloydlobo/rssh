# rssh

[![Rust](https://github.com/lloydlobo/rssh/actions/workflows/ci.yml/badge.svg)](https://github.com/lloydlobo/rssh/actions/workflows/ci.yml)

## Profiling

### `cargo flamegraph`

Use `cargo flamegraph` to build and profile a Rust project by specifying the --build and --profile options.

For example:

```sh
cargo flamegraph --build --profile=release --bin <binary_name>
```

This will compile the project in release mode, profile the specified binary and generate a flamegraph for it.
The flamegraph will help you visualize the performance of the program by showing the hotspots in the code, where most of the time is spent.

### `perf`

First, make sure the release build of the program has debugging information enabled by adding the following to the Cargo.toml file:

```toml
[profile.release]
debug = true
```

or set the environment variable:

```sh
export CARGO_PROFILE_RELEASE_DEBUG=true
```

Build the program in release mode:

```sh
cargo build --release
```

Use perf record to capture performance data:

```sh
perf record -g --call-graph dwarf target/release/the-program
```

Use perf report to analyze the performance data:

```sh
perf report
```

This will give a detailed report of the performance of the program and help identify performance bottlenecks.
