mod pentry;

use crate::pentry::{prompt, read_passwords_from_file, ServiceInfo};
use std::vec::Vec; // Add this line

fn clr() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clr();
    let ascii = r#"
    ______   ______    _______  _______        __     __ ______   __    __ $$ | _$$ |_   
   /      \ /      \  /       |/       |      /  \   /  /      \ /  |  /  |$$ |/ $$   |  
  /$$$$$$  |$$$$$$  |/$$$$$$$//$$$$$$$/       $$  \ /$$/$$$$$$  |$$ |  $$ |$$ |$$$$$$/   
  $$ |  $$ |/    $$ |$$      \$$      \        $$  /$$/ /    $$ |$$ |  $$ |$$ |  $$ | __ 
  $$ |__$$ /$$$$$$$ | $$$$$$  |$$$$$$  |        $$ $$/ /$$$$$$$ |$$ \__$$ |$$ |  $$ |/  |
  $$    $$/$$    $$ |/     $$//     $$/          $$$/  $$    $$ |$$    $$/ $$ |  $$  $$/ 
  $$$$$$$/  $$$$$$$/ $$$$$$$/ $$$$$$$/            $/    $$$$$$$/  $$$$$$/  $$/    $$$$/  
  $$ |                                                                                   
  $$ |                                                                                   
  $$/  
    "#;

    println!("{ascii}");
    loop {
        println!("Password Manager Menu");
        println!("1. Add Password");
        println!("2. List Passwords");
        println!("3. Search Entry");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let entry = ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    prompt("Password: "),
                );
                println!("Entry added successfully.");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|error| {
                    println!("Error reading passwords: {}", error);
                    Vec::new()
                });
                for item in &services {
                    println!(
                        "Service: {}
                        - Username: {}
                        - Password: {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|error| {
                    println!("Error reading passwords: {}", error);
                    Vec::new()
                });

                let search = prompt("Search: ");
                for item in &services {
                    if item.service == search {
                        println!(
                            "Service: {}
                            - Username: {}
                            - Password: {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
        println!("\n\n");
    }
}
