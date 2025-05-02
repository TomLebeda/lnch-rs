This is a very simple CLI tool that will run a child process with group-id set to 0, which allows for that child process to keep running even when the original parent process is killed. This works only on Unix systems.

*Inspired by https://github.com/oem/lnch.*

**Installation**:

You can download the pre-compiled binaries from [here](https://github.com/TomLebeda/chroma_code/releases) or clone this repo and run `cargo build --release` (assuming you have [Rust and Cargo installed and set up](https://rustup.rs/)).
