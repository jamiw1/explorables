use std::env;
use std::io;
pub mod list_dir;
pub mod travel;

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
    println!("\x1B[31;1m{} v{}\x1B[0m \x1B[37menter a path or 'exit' to quit\x1B[0m", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    loop {
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        match input {
            "exit" => break,
            _ => {
                println!("\x1B[2J\x1B[1;1H");
                travel::travel(input);
                list_dir::list(env::current_dir().unwrap().to_str().unwrap());
                println!("\x1B[35;1m{}\x1B[0m \x1B[2m|->\x1B[0m enter a path or 'exit' to quit\x1B[0m", env::current_dir().unwrap().to_str().expect("Failed to get current directory"));
            },
        }
    }
}
