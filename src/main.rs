use cmd_import::*;
use dialoguer::*;
use serde::{Serialize,Deserialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter,Display};
use std::str::FromStr;

cmd_import!();

fn main() {
    let mut selection: Vec<Commands> = Commands::iter().collect();
    
    loop {
        match Select::new().items(&selection).interact_opt() {
            Ok(Some(s)) => {
                let json = handle_command(selection[s].clone());                  
                let socket = std::net::UdpSocket::bind("172.29.57.165:8031").unwrap();            
                println!("Start socket on: {:?}", socket);
                match socket.send_to(json.as_bytes(),"172.29.57.165:8030") {
                    Ok(_) => {
                        let mut buf = [0; 1024];
                        match socket.recv(&mut buf) {
                            Ok(b) => {
                                let response = String::from_utf8_lossy(&buf[..b]);
                                println!("Response: {}", response);
                            }
                            Err(e) => println!("Error: {}",e),
                        }
                    }
                    Err(e) => println!("Error: {}",e),
                }
            }
            _ => continue,
        }
    }
}