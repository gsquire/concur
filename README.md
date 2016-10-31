# concur
A simple tool to help repeat commands concurrently from the terminal. It was inspired by the
builtin shell command `repeat` but adds support for many threads.

### Building
If you have Rust installed, just run:

```sh
cargo build --release
```

### Usage
You can run it with or without threads as such:

```sh
# Run a command synchronously.
cargo run --release -- 10 curl https://www.rust-lang.org

# Run it with threads.
cargo run --release -- 10 curl https://www.rust-lang.org -j 2
```

### License
MIT
