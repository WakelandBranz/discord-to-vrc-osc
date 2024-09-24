use serde::{Deserialize, Serialize};
use std::fs;
use std::fmt::Write;
use toml;

use crate::utils;

const CONFIG_FILEPATHS: [&str ; 4] = [
    "config.toml",
    "Config.toml",
    "config\\config.toml",
    "config\\Config.toml"
];

#[derive(Deserialize, Serialize)]
pub struct Auth {
    pub token: String,
    pub owners: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Options {
    pub prefixes: Vec<String>,
    pub mention_as_prefix: bool,
    pub message: String, // Implemented in some test variations
}

#[derive(Deserialize, Serialize)]
pub struct System {
    pub ephemeral_admin_commands: bool,
    pub vrc_client_logging_channel: String,
}

#[derive(Deserialize, Serialize)]
pub struct VrcClient {
    pub localhost: String,
    pub receiver_port: u16,
    pub transmitter_port: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub auth: Auth,
    pub options: Options,
    pub system: System,
    pub vrc_client: VrcClient,
}

impl Config {
    pub fn new() -> Self {
        for path in CONFIG_FILEPATHS {
            if let Ok(toml_string) = fs::read_to_string(path) {
                match toml::from_str(&toml_string) {
                    Ok(config) => return config,
                    Err(e) => panic!("Failed to parse {}: {}", path, e),
                }
            }
        }
        panic!("Failed to parse config")
    }

    pub fn update(&mut self) {
        *self = Self::new();
    }

    //pub fn set_logging_channel(&mut self, channel: ChannelId) {
    //    self.system.logging.log_channel = channel.to_string();
    //}

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        writeln!(output, "Configuration:").unwrap();

        // Auth section
        writeln!(output, "{}", utils::format_section("Auth")).unwrap();
        writeln!(output, "{}", utils::format_field("Token", &format!("{}...", self.auth.token.chars().take(10).collect::<String>()))).unwrap();
        writeln!(output, "{}", utils::format_list("Owners", &self.auth.owners)).unwrap();
        // Options section
        writeln!(output, "{}", utils::format_section("Options")).unwrap();
        writeln!(output, "{}", utils::format_field("Ephemeral Admin Commands", &self.system.ephemeral_admin_commands.to_string())).unwrap();
        writeln!(output, "{}", utils::format_list("Prefixes", &self.options.prefixes)).unwrap();
        writeln!(output, "{}", utils::format_field("Mention as Prefix", &self.options.mention_as_prefix.to_string())).unwrap();

        output
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}