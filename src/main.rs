use std::{error::Error, io::{self, Write}};
use tokio;
use clearscreen::clear;
use scrap_lib::scraper::scrap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // interface básica
    loop{
        let mut command = String::new();
        println!("Commands : scrap <product> <number pages>| help | clear | exit");
        print!(">");
        io::stdout().flush().expect("erro");
        io::stdin()
            .read_line(&mut command)
            .expect("erro");
        let command : Vec<&str> = command.split_whitespace().collect();
        match command.as_slice(){
            [command_, product,number] => {
                match *command_{
                    "scrap" => {
                        let number : u32 = match (*number).to_string().parse(){
                            Ok(num) => num,
                            Err(_) => {
                                println!("Invalid number:  Type 'help' for available arguments.");
                                continue;
                            },
                        };
                        match scrap(&*product.trim().to_lowercase(), number).await{
                            Ok(state) => {
                                match state {
                                    None => eprintln!("Error: Failed to initialize the driver."),  
                                    Some(_) => println!("The scrap was successfully completed."),   
                                };
                            },
                            Err(erro) => {
                                eprintln!("Error: Failed to complete scraping operation. Reason: {}", erro.to_string());
                            },
                        }
                    },
                    _ => println!("Unknown command: '{}'. Type 'help' for available commands.", *command_),
                }
            },
            [command_] => {
                match *command_{
                    "help" => {
                        println!("\n=== HELP - Web Scraper (Study Project) ===");
                        println!("This program uses browser automation (GeckoDriver) to collect data.");
                        println!("---------------------------------------------------------------------");
                        println!("AVAILABLE COMMANDS:");
                        println!("  scrap <product> <number>: Starts Firefox, searches for the product, collects");
                        println!("                            Name and Price from the specified number of pages, and saves them");
                        println!("                            to a local JSON file.");
                        println!("  clear            : Clears the terminal screen.");
                        println!("  help             : Displays this help message.");
                        println!("  exit             : Exits the application.");
                        println!("---------------------------------------------------------------------");
                        println!("NOTE: Sleep delays are currently used to handle page loading.");
                        println!("      The output file will be named 'product_name.json'.\n");
                    }
                    "clear" => {
                        match clear(){
                            Err(_) => eprintln!("Error: Failed to clear the terminal screen."),
                            _ => (),
                        };
                    },
                    "exit" => {
                        break;
                    }
                    _ => println!("Unknown command: '{}'. Type 'help' for available commands.", *command_),
                }
            },
            _ =>  println!("Unknown command: Type 'help' for available commands."),
        }
    }
    Ok(())
}