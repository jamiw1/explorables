use std::env;
use std::io;
pub mod list_dir;
pub mod travel;

fn set_path(input: &str) {
    let message =travel::travel(input);
    list_dir::list(env::current_dir().unwrap().to_str().unwrap());
    if message != "" {
        println!("\x1B[31;1m{}\x1B[0m", message);
    }
}

fn print_prompt_message(first_time: bool) {
    if first_time == false {
        println!("\x1B[35;1m{}\x1B[0m \x1B[2m|->\x1B[0m enter a path, type ':h' for help or ':q' to quit\x1B[0m", env::current_dir().unwrap().to_str().expect("Failed to get current directory"));
    } else {
        println!("\x1B[35;1mC:\\\x1B[0m \x1B[2m|->\x1B[0m \x1B[31;1m{} v{}\x1B[0m \x1B[37menter a path, type ':h' for help or ':q' to quit\x1B[0m", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    
    for arg in args.iter() {
        if arg == "-h" || arg == "--help" {
            println!("usage: {} <args> <path>", env!("CARGO_PKG_NAME"));
            println!("args: -l, --list, -h, --help, -v, --version");
            return;
        }
        if arg == "-l" || arg == "--list" {
            list_dir::list(&args[2]);
            return;
        }
        if arg == "-v" || arg == "--version" {
            println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            return;
        }
    }
    
    set_path("C:\\");
    print_prompt_message(true);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        match input {
            ":q" => break,
            ":h" => {
                println!("\x1B[2J\x1B[1;1H");
                println!("command list:\n:q - quit the program\n:h - brings up this help page\n:v - displays current version");
                print_prompt_message(false);
            },
            ":v" => {
                println!("\x1B[2J\x1B[1;1H");
                println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                print_prompt_message(false);
            },
            _ => {
                println!("\x1B[2J\x1B[1;1H");
                set_path(&input);    
                print_prompt_message(false);
            }
        }
    }
}
