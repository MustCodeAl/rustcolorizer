use clap::{Arg, ArgAction, Command};
// use clap_complete_fig::{generate, Shell};
// use clap_complete_fig::Fig;
use std::env;
use std::io::{self};
// use std::sync::Arc, atomic::{AtomicBool}};
use std::process::exit;
use log::info;
use std::sync::atomic::AtomicBool;

use std::sync::Arc;



mod config;
mod signal_handler;
mod runner;
mod rczat;
mod data;

fn main() {
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
            .help("Use this configuration file for rczat"))
        .arg(Arg::new("colour")
            .long("colour")
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
            // .possible_values(Shell::possible_values().map(|s| s.to_string()).chain(std::iter::once("fig".into())))
            .help("Generate shell completions for the specified shell"))
        .arg(Arg::new("COMMAND")
            .action(ArgAction::Append)
            .help("Command and arguments to execute")
            .required_unless_present_any(&["interactive", "debug", "aliases", "completion"]))
        .get_matches();

    if let Some(shell) = matches.value_of("completion") {
        generate_completions(shell);
        return;
    }

    if matches.get_flag("interactive") {
        info!("Entering interactive mode");
        run_interactive_mode();
        return;
    }

    if matches.get_flag("aliases") {
        generate_aliases();
        return;
    }

    let term_now = Arc::new(AtomicBool::new(false));
    signal_handler::setup_signal_handler(Arc::clone(&term_now));

    let args: Vec<String> = matches
        .get_many::<String>("COMMAND")
        .unwrap_or_default()
        .map(|s| s.to_string())
        .collect();

    if args.iter().any(|arg| arg == "rczat") {
        info!("Processing with rczat");
        rczat::process_rczat(&matches, &args);
    } else {

        if matches.get_flag("multiline") {
            info!("Entering multi-line mode");
            run_multiline_mode(term_now);
        } else {
            let cfile = config::find_config_file(&matches, &args);
            info!("Executing command with config file: {}", cfile);
            let status = runner::execute_command(&args, &cfile, &matches, term_now);
            exit(status);
        }
    }
}

fn generate_completions(shell: &str) {
    let mut app = Command::new("RustColorizer");

    if shell == "fig" {
        let fig = Fig::new(app.clone());
        generate(fig, &mut app, "rcz", &mut io::stdout());
    } else {
        let shell: Shell = shell.parse().unwrap();
        generate(shell, &mut app, "rcz", &mut io::stdout());
    }
}

fn generate_aliases() {
    let config_path = "rcz.conf"; // Adjust this path as necessary
    let commands = config::extract_commands_from_config(config_path);

    let shell = match std::env::var("SHELL") {
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

    let aliases = generate_aliases_for_shell(&commands, shell);

    for alias in aliases {
        println!("{}", alias);
    }
}

fn generate_aliases_for_shell(commands: &[String], shell: &str) -> Vec<String> {
    commands.iter().map(|cmd| {
        match shell {
            "bash" | "zsh" => format!("alias {}='rcz {}'", cmd, cmd),
            "fish" => format!("alias {} 'rcz {}'", cmd, cmd),
            _ => format!("alias {}='rcz {}'", cmd, cmd), // Default to bash-style
        }
    }).collect()
}







