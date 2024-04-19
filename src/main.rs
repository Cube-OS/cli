pub mod trxvu;
pub mod example;
pub mod dop;

use cli_macro::*;
use dialoguer::*;
use serde::{Serialize,Deserialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter,Display};

const CLI_IP: &str = "192.168.0.1:8050";

// Purpose: Main entry point for the application
#[derive(Debug, EnumIter, Display, Clone)]
pub enum Services {
    // Trxvu,
    // Example,
    DoP,
}

fn main() {
    let services: Vec<Services> = Services::iter().collect();

    loop {
        match Select::new().items(&services).interact_opt() {
            Ok(Some(s)) => {
                match services[s].clone() {
                    // Services::Trxvu => trxvu::cli("192.168.0.1:8030"),
                    // Services::Example => example::cli("192.168.0.1:8029"),
                    Services::DoP => dop::cli("192.168.0.1:8035"),
                }
            }
            _ => continue,
        }
    }
}