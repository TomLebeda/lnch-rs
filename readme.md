This is a very simple CLI tool that will run a child process with group-id set to 0, which allows for that child process to keep running even when the original parent process is killed. This works only on Unix systems.

This can be usually done by using `nohup` or detaching the process (using `&` in Bash), but sometimes this doesn't work well when the parent process ends too quickly (before the child process can properly start and detach),
Also not all shells provide a way to simply detach the process (notably Nushell).

*Inspired by https://github.com/oem/lnch.*

**Usage**:

It's very simple, just write the command after `lnch-rs`. For example, to launch GIMP, simply execute:
```bash
$ lnch-rs gimp
```

**Installation**:

You can download the pre-compiled binaries from [here](https://github.com/TomLebeda/lnch-rs/releases) or clone this repo and run `cargo build --release` (assuming you have [Rust and Cargo installed and set up](https://rustup.rs/)).
