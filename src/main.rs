use clap::{Arg, ArgAction, Command};
// use clap_complete::{generate, Shell};
use std::env;
use std::io::{self};
use std::sync::{Arc, atomic::{AtomicBool}};
use std::process::exit;

mod config;
mod signal_handler;
mod runner;
mod rczat;
mod data;

fn main() {
    // Initialize the command-line argument parser
    let matches = Command::new("RustColorizer")
        .version("1.0")
        .about("A tool to colorize command outputs")
        .arg(Arg::new("stderr")
            .short('e')
            .long("stderr")
            .action(ArgAction::SetTrue)
            .help("Redirect stderr"))
        .arg(Arg::new("stdout")
            .short('s')
            .long("stdout")
            .action(ArgAction::SetTrue)
            .help("Redirect stdout, even if -e is selected"))
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            // .takes_value(true)
            .help("Use this configuration file for rczat"))
        .arg(Arg::new("colour")
            .long("colour")
            // .takes_value(true)
            .help("Set colour mode (on, off, auto)"))
        .arg(Arg::new("pty")
            .long("pty")
            .action(ArgAction::SetTrue)
            .help("Run command in pseudoterminal (experimental)"))
        .arg(Arg::new("multiline")
            .long("multiline")
            .action(ArgAction::SetTrue)
            .help("Enable multi-line mode for command execution"))
        .arg(Arg::new("interactive")
            .long("interactive")
            .action(ArgAction::SetTrue)
            .help("Enter interactive mode to configure new commands"))
        .arg(Arg::new("debug")
            .long("debug")
            .action(ArgAction::SetTrue)
            .help("Enable debugging output"))
        .arg(Arg::new("aliases")
            .long("aliases")
            .action(ArgAction::SetTrue)
            .help("Generate shell aliases for configured commands"))
        .arg(Arg::new("completion")
            .long("completion")
            // .takes_value(true)
            // .possible_values(Shell::possible_values())
            .help("Generate shell completions for the specified shell"))
        .arg(Arg::new("COMMAND")
            .action(ArgAction::Append)
            .help("Command and arguments to execute")
            .required_unless_present_any(&["interactive", "debug", "aliases", "completion"]))
        .get_matches();

    // Handle shell completion generation
    // if let Some(shell) = matches.value_of("completion") {
    //     generate_completions(shell);
    //     return;
    // }

    // Handle interactive mode
    if matches.get_flag("interactive") {
        println!("Entering interactive mode...");
        run_interactive_mode();
        return;
    }

    // Handle alias generation
    if matches.get_flag("aliases") {
        generate_aliases();
        return;
    }

    // Setup signal handling for graceful termination
    let term_now = Arc::new(AtomicBool::new(false));
    signal_handler::setup_signal_handler(Arc::clone(&term_now));

    // Get the command and its arguments
    let args: Vec<String> = matches
        .get_many::<String>("COMMAND")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

    // Process the command with colorization or multi-line mode
    if matches.get_flag("multiline") {
        println!("Entering multi-line mode...");
        run_multiline_mode(&args, term_now);
    } else {
        let config_file = config::find_config_file(&matches, &args);
        println!("Executing command with config file: {}", config_file);
        let status = runner::execute_command(&args, &config_file, &matches, term_now);
        exit(status);
    }
}

// Function to generate shell completions
fn generate_completions(shell: &str) {
    let mut app = Command::new("RustColorizer");
    // let shell: Shell = shell.parse().unwrap();
    // generate(shell, &mut app, "rcz", &mut io::stdout());
}

// Function to generate shell aliases
fn generate_aliases() {
    let config_path = "rcz.conf"; // Adjust this path as necessary
    let commands = config::extract_commands_from_config(config_path);

    let shell = match env::var("SHELL") {
        Ok(val) => {
            if val.contains("zsh") {
                "zsh"
            } else if val.contains("fish") {
                "fish"
            } else {
                "bash"
            }
        }
        Err(_) => "bash",
    };

    let aliases = generate_aliases_for_shell(&commands, &shell);

    for alias in aliases {
        println!("{}", alias);
    }
}

// Function to create aliases based on shell type
fn generate_aliases_for_shell(commands: &[String], shell: &str) -> Vec<String> {
    commands.iter().map(|cmd| {
        match shell {
            "bash" | "zsh" => format!("alias {}='rcz {}'", cmd, cmd),
            "fish" => format!("alias {} 'rcz {}'", cmd, cmd),
            _ => format!("alias {}='rcz {}'", cmd, cmd), // Default to bash-style
        }
    }).collect()
}

// Function to handle multi-line mode
fn run_multiline_mode(args: &[String], term_now: Arc<AtomicBool>) {
    println!("Multi-line mode is currently a basic placeholder implementation.");
    // Implement logic for handling multi-line input and processing here
    // For now, we'll simply echo the input lines.
    while !term_now.load(std::sync::atomic::Ordering::Relaxed) {
        for arg in args {
            println!("{}", arg);
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// Function to handle interactive mode
fn run_interactive_mode() {
    println!("Interactive mode is currently a basic placeholder implementation.");
    // Implement the interactive configuration logic here
    // For now, we'll just simulate interaction
    let mut buffer = String::new();
    println!("Enter command configuration details:");
    std::io::stdin().read_line(&mut buffer).expect("Failed to read line");
    println!("Received configuration: {}", buffer);
}