use clap::Parser;
use directories::UserDirs;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{fs, io};
use toml;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "DELEMETER")]
    delemeter: Option<String>,

    #[arg(short, long, value_name = "CONFIG")]
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
    let config_path: String;

    // search for config
    if cli.config.is_some() {
        config_path = cli
            .config
            .expect("Invalid Config Path.")
            .to_str()
            .unwrap()
            .to_string();
    } else {
        config_path = search_for_config()?;
    }

    // get the config file path
    let config_contents: String = match fs::read_to_string(&config_path) {
        Ok(res) => res,
        Err(err) => panic!("Could not open file {}, due to error {}", config_path, err),
    };

    // load the config file into the struct
    let mut conf: Config = match toml::from_str(&config_contents) {
        Ok(res) => res,
        Err(err) => panic!("Invalid Config: {}", err),
    };

    if cli.delemeter.is_some() {
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

fn search_for_config() -> Result<String, Box<dyn Error>> {
    let project_config = "unicode-formatter.toml";

    let user_dirs = UserDirs::new().expect("Unable to get User Directories");
    let home_dir = user_dirs.home_dir();
    let system_config_rel_path = ".config/unicode-formatter/unicode-formatter.toml";
    let system_config_path = PathBuf::from(home_dir).join(system_config_rel_path);

    let proj_list = [project_config, system_config_path.to_str().unwrap()];

    // search for the files in order
    for fp in proj_list {
        // see if file exists
        let fs_exists = fs::exists(fp)?;

        // if so return the file
        if fs_exists == true {
            return Ok(fp.to_string());
        }
    }
    panic!("Cannot find config file.");
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
