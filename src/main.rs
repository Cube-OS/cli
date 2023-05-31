pub mod trxvu;
pub mod example;

use cmd_import::*;
use dialoguer::*;
use serde::{Serialize,Deserialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter,Display};

const CLI_IP: &str = "172.29.57.165:8031";

// Purpose: Main entry point for the application
#[derive(Debug, EnumIter, Display, Clone)]
pub enum Services {
    Trxvu,
    Example,
}

fn main() {
    let services: Vec<Services> = Services::iter().collect();

    loop {
        match Select::new().items(&services).interact_opt() {
            Ok(Some(s)) => {
                match services[s].clone() {
                    Services::Trxvu => trxvu::cli("172.29.57.165:8030"),
                    Services::Example => example::cli("172.29.57.165:8029"),
                }
            }
            _ => continue,
        }
    }
}