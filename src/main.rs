use std::env;
use std::io;
pub mod list_dir;
pub mod travel;

fn set_path(input: &str) {
    let message = travel::travel(input);
    list_dir::list(env::current_dir().unwrap().to_str().unwrap(), "");
    if message != "" {
        eprintln!("\x1B[31;1m{}\x1B[0m", message);
    }
}

fn print_prompt_message(first_time: bool, search: bool) {
    if first_time == false && search == false {
        println!("\x1B[35;1m{}\x1B[0m \x1B[2m|->\x1B[0m enter a path, type ':h' for help or ':q' to quit\x1B[0m", env::current_dir().unwrap().to_str().expect("Failed to get current directory"));
    } else if search == true {
        println!("\x1B[35;1m{}\x1B[0m \x1B[2m|->\x1B[0m search for contents in this folder...\x1B[0m", env::current_dir().unwrap().to_str().expect("Failed to get current directory"));
    } else {
        println!("\x1B[35;1mC:\\\x1B[0m \x1B[2m|->\x1B[0m \x1B[31;1m{} v{}\x1B[0m \x1B[37menter a path, type ':h' for help or ':q' to quit\x1B[0m", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }
}
fn main() {
    let mut searching = false;
    let mut search_input = String::new();
   
    let args: Vec<String> = env::args().collect();
    
    for arg in args.iter() {
        if arg == "-h" || arg == "--help" {
            println!("usage: {} <args>", env!("CARGO_PKG_NAME"));
            println!("args: -l, --list, -h, --help, -v, --version");
            return;
        }
        if arg == "-l" || arg == "--list" {
            list_dir::list(&args[2], "");
            return;
        }
        if arg == "-v" || arg == "--version" {
            println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            return;
        }
    }
    
    set_path("C:\\");
    print_prompt_message(true, false);
    loop {
        let current_dir = env::current_dir().unwrap();
        let current_path = current_dir.to_str().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        match input {
            ":q" => break,
            ":h" => {
                println!("\x1B[2J\x1B[1;1H");
                println!("command list:\n:q - quit the program\n:h - brings up this help page\n:v - displays current version\n:s - search contents of folder");
                print_prompt_message(false,false);
            },
            ":v" => {
                println!("\x1B[2J\x1B[1;1H");
                println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                print_prompt_message(false,false);
            },
            ":s" => {
                println!("\x1B[2J\x1B[1;1H");
                if searching == true {
                    searching = false;
                    set_path(&current_path); 
                    print_prompt_message(false,false);
                } else {
                    searching = true;
                    list_dir::list(&current_path, "");
                    print_prompt_message(false,true);
                }
            },
            _ => {
                println!("\x1B[2J\x1B[1;1H");
                if searching == false {
                    set_path(&input); 
                    print_prompt_message(false,false);
                } else {
                    search_input = input.to_string();
                    list_dir::list(&current_path, &search_input);
                    print_prompt_message(false,true);
                }
                   
                
            }
        }
    }
}
