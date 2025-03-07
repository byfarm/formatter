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
    #[arg(short, long, value_name = "DELEMETER")]
    delemeter: Option<String>,

    #[arg(short, long, value_name = "CONFIG", default_value = "/home/byron/Documents/projects/unicode-formatter/config.toml")]
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
    // parse the cli arguments
    let cli = Cli::parse();

    // get the config file path
    let config_contents: String =
        fs::read_to_string(cli.config.expect("Invalid Config Path.").as_path())?;

    // load the config file into the struct
    let mut conf: Config = toml::from_str(&config_contents)?;

    if !cli.delemeter.is_none() {
        conf.delemeter = cli.delemeter.expect("Invalid Delemeter.");
    }

    // read the input from standard in
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // make the replacements
    let output = replace(input, &conf)?;

    // write to standard out
    io::stdout().write_all(output.as_bytes())?;
    Ok(())
}

fn replace(mut buffer: String, conf: &Config) -> Result<String, Box<dyn Error>> {
    let delemeter = &conf.delemeter;
    // let filter_matches = format!("{}(\"(.*)\"|[^ ]*)", delemeter);
    for (key, val) in &conf.replacements {
        let substring = delemeter.clone() + key;
        let re = Regex::new(&substring)?;
        buffer = re.replace_all(&buffer, val).to_string();
    }
    Ok(buffer)
}
