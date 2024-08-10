use clap::ArgMatches;
use fancy_regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::exit;
use crate::data::{CgrcAttrib, CgrcColorItem, CgrcConf, CgrcConfItem, COLORS_BACK, COLORS_FORG};
// mod data;
// use data::{CgrcAttrib, CgrcConf, CgrcConfItem, CgrcColorItem, COLORS_ATTRS, COLORS_BACK, COLORS_FORG};

pub fn process_rczat(matches: &ArgMatches, _args: &[String]) {
    let binding = String::new();
    let main_conf_file = matches.get_one::<String>("config").unwrap_or(&binding);
    if main_conf_file.is_empty() {
        eprintln!("No main configuration file specified.");
        exit(1);
    }

    let main_config = load_main_config(main_conf_file);
    if main_config.items.is_empty() {
        eprintln!("Main configuration file is empty or invalid.");
        exit(1);
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let mut processed = false;

                for item in &main_config.items {
                    if let Some(regex) = &item.regex {
                        if let Ok(Some(captures)) = regex.captures(&line) {
                            let colored_line = apply_colors(&line, &captures, &item.colors);
                            println!("{}", colored_line);
                            processed = true;
                            if item.skip.unwrap_or(false) {
                                break;
                            }
                        }
                    }
                }

                if !processed {
                    println!("{}", line);
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}

fn load_main_config(conf_file: &str) -> CgrcConf {
    let file = File::open(conf_file).unwrap_or_else(|e| {
        eprintln!("Failed to open main configuration file: {}. Error: {}", conf_file, e);
        exit(1);
    });
    let reader = BufReader::new(file);
    let mut conf = CgrcConf::new();
    let mut current_item = CgrcConfItem::new();

    for line in reader.lines() {
        match line {
            Ok(line) => parse_config_line(&line.trim(), &mut current_item, &mut conf),
            Err(e) => {
                eprintln!("Error reading configuration file: {}", e);
                exit(1);
            }
        }
    }

    if current_item.regex.is_some() {
        conf.items.push(current_item);
    }

    conf
}

fn parse_config_line(line: &str, current_item: &mut CgrcConfItem, conf: &mut CgrcConf) {
    if line.is_empty() || line.starts_with('#') {
        return;
    }

    if line.starts_with("regexp=") {
        if current_item.regex.is_some() {
            conf.items.push(current_item.clone());
            *current_item = CgrcConfItem::new();
        }
        current_item.regex = Some(Regex::new(&line.replacen("regexp=", "", 1)).unwrap_or_else(|e| {
            eprintln!("Invalid regex pattern: {}. Error: {}", line, e);
            exit(1);
        }));
    } else if line.starts_with("colours=") {
        let colors = line.replacen("colours=", "", 1);
        current_item.colors = parse_colors(&colors);
    } else if line.starts_with("skip=") {
        current_item.skip = Some(line.replacen("skip=", "", 1) == "yes");
    } else if line == "-" {
        conf.items.push(current_item.clone());
        *current_item = CgrcConfItem::new();
    }
}

fn parse_colors(colors_str: &str) -> Vec<CgrcColorItem> {
    colors_str.split(',').map(|color| {
        let attr_set: HashSet<CgrcAttrib> = HashSet::new();
        let forg = COLORS_FORG.get(color.trim()).cloned().unwrap_or(COLORS_FORG["default"]);
        let back = COLORS_BACK.get("on_default").cloned().unwrap_or(COLORS_BACK["on_default"]);
        CgrcColorItem::new(attr_set, forg, back)
    }).collect()
}

fn apply_colors(line: &str, captures: &fancy_regex::Captures, colors: &[CgrcColorItem]) -> String {
    let mut result = line.to_string();
    for (i, capture) in captures.iter().enumerate() {
        if let Some(matched) = capture {
            let color_item = colors.get(i).unwrap_or(&colors[0]);
            let colored = format!("{}{}{}", color_item.escape_seq, matched.as_str(), color_item.clear_seq);
            result = result.replace(matched.as_str(), &colored);
        }
    }
    result
}