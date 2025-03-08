use clap::Parser;
use std::io::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::{fs, io};
use regex::Regex;
use toml;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // #[arg(short, long, value_name = "DELEMETER", default_value = "#U")]
    // delemeter: Option<String>,
    #[arg(short, long, value_name = "CONFIG", default_value = "./config.toml")]
    config: Option<PathBuf>,

    // #[arg(short, long, value_name = "FILE")]
    // file: Option<PathBuf>,
}

#[derive(Deserialize)]
struct Config {
    delemeter: String,
    replacements: HashMap<String, String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let config_contents: String =
        fs::read_to_string(cli.config.expect("Invalid Config Path.").as_path())?;

    let conf: Config = toml::from_str(&config_contents)?;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let output = replace(input, conf)?;

    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}

fn replace(mut buffer: String, conf: Config) -> Result<String, Box<dyn Error>> {
    let delemeter = &conf.delemeter;
    for (key, val) in &conf.replacements {
        let substring = delemeter.clone() + key;
        let re = Regex::new(&substring).unwrap();
        buffer = re.replace_all(&buffer, val).to_string();
    }
    Ok(buffer)
}
