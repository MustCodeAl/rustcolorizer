use clap::ArgMatches;
use std::process::{Command, exit};
use std::sync::{Arc, atomic::AtomicBool};

pub fn execute_command(args: &[String], config_file: &str, matches: &ArgMatches, term_now: Arc<AtomicBool>) -> i32 {
    let command = &args[0];
    let command_args = &args[1..];

    let mut child = Command::new(command)
        .args(command_args)
        .spawn()
        .expect("Failed to start command");

    while !term_now.load(std::sync::atomic::Ordering::Relaxed) {
        match child.try_wait() {
            Ok(Some(status)) => return status.code().unwrap_or(1),
            Ok(None) => std::thread::sleep(std::time::Duration::from_millis(100)),
            Err(e) => {
                eprintln!("Failed to wait on child process: {}", e);
                return 1;
            }
        }
    }

    // If a termination signal was received, terminate the child process
    match child.kill() {
        Ok(_) => {
            println!("Command terminated due to signal.");
            1
        }
        Err(e) => {
            eprintln!("Failed to terminate command: {}", e);
            1
        }
    }
}