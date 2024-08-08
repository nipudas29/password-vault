use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            service,
            username,
            password,
        }
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }

    pub fn from_user_input() -> Self {
        let service = prompt("Enter Service:");
        let username = prompt("Enter Username:");
        let password = prompt("Enter Password:");

        ServiceInfo::new(service, username, password)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to convert to JSON")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json")
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error writing to file: {}", e);
                } else {
                    println!("Successfully wrote to passwords.json");
                }
            }
            Err(e) => eprintln!("Error opening file: {}", e),
        }
    }
}

pub fn read_passwords_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    let file = File::open("passwords.json")?;
    let reader = BufReader::new(file);
    let mut services = Vec::new();

    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                services.push(service_info);
            }
        }
    }
    Ok(services)
}

pub fn prompt(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    // Example usage
    let service_info = ServiceInfo::from_user_input();
    service_info.write_to_file();

    match read_passwords_from_file() {
        Ok(services) => {
            for service in services {
                println!("{:?}", service);
            }
        }
        Err(e) => eprintln!("Error reading from file: {}", e),
    }
}
