//! Minimal launcher that detaches the child process so it keeps running when the parent process is killed

use std::{env, os::unix::process::CommandExt, process::Command};

fn main() {
    let mut args = env::args().skip(1);
    let cmd = args.next().unwrap_or_else(|| {
        eprintln!("Usage: lnch-rs <command> <optional parameters>");
        std::process::exit(1);
    });
    let args: Vec<String> = args.collect();

    match Command::new(cmd).args(&args).process_group(0).spawn() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to spawn process: {e}");
            std::process::exit(1);
        }
    }
}
