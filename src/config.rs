use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::builder::Str;

pub fn extract_commands_from_config(config_path: &str) -> Vec<String> {
    let file = File::open(config_path).unwrap();
    let reader = BufReader::new(file);
    let mut commands = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            let command = parts[0].trim();
            commands.push(command.to_string());
        }
    }

    commands
}

pub fn find_config_file(matches: &clap::ArgMatches, args: &[String]) -> String {
    let config_path: String = matches.get_one::<String>("config").expect("").to_string();
    // matches.get_raw("config").unwrap().is_present();
    let command_name = &args[0];

    let file = File::open(config_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() == 2 {
            let regex_pattern = parts[0].trim();
            let config_file = parts[1].trim();
            if regex::Regex::new(regex_pattern)
                .unwrap()
                .is_match(command_name)
            {
                return config_file.to_string();
            }
        }
    }

    String::new()
}