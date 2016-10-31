# concur
A simple tool to help repeat commands concurrently from the terminal. It was inspired by the
builtin shell command `repeat` but adds support for many threads.

### Building
If you have Rust installed, just run:

```sh
cargo build --release
```

You can also install it with cargo making it available as a binary:

```sh
cargo install concur
```

### Usage
You can run it with or without threads as such:

```sh
# Run a command synchronously.
concur 10 curl https://www.rust-lang.org

# Run it with threads.
concur 10 curl https://www.rust-lang.org -j 2

# Run a command with it's own arguments.
concur 5 ls -- -a
```

When running a command with it's own arguments, specifying two dashes before listing them allows them to be
ignored by your program's options.

### License
MIT
