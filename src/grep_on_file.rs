use regex::Regex;
use std::fs;

// i want to avoid calling unwrap() what to do?

pub fn grep_on_file(regex: Regex, path : &str) {
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => {
            println!("Error reading file {}", path);
            return;
        },
    };
    for line in contents.lines() {
        if regex.is_match(line) {
            println!("{}", line);
        }
    }
}