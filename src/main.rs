use serde::Deserialize;
use std::fs;
// use std::io;
// use std::process::Command;


#[derive(Deserialize)]
struct Alias {
    name: String,
    command: String,
}

fn main() {
    let config = fs::read_to_string("config.json").expect("Unable to read config file");
    let aliases: Vec<Alias> = serde_json::from_str(&config).expect("Unable to parse config JSON");

    println!("Available aliases:");
    for alias in &aliases {
        println!("- {}", alias.name  );

    }
}
