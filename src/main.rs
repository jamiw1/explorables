use std::env;
use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode}
};
pub mod list_dir;
pub mod travel;

fn read_char() -> (io::Result<KeyCode>, io::Result<KeyModifiers>) {
    loop {
        if let Event::Key(KeyEvent { code, modifiers, kind: KeyEventKind::Press, .. }) = event::read().unwrap() {
            return (Ok(code), Ok(modifiers));
        }
    }
}

fn set_path(input: &str) {
    let message = travel::travel(input);
    list_dir::list(env::current_dir().unwrap().to_str().unwrap(), "");
    if message != "" {
        eprintln!("\x1B[31;1m{}\x1B[0m", message);
    }
}

fn print_prompt_message(first_time: bool, search: bool, search_input: &str) {
    let current_dir = env::current_dir().unwrap();
    let current_path = current_dir.to_str().unwrap();
    if first_time == false && search == false {
        println!("\x1B[35;1m{}\x1B[0m \x1B[2m|->\x1B[0m enter a path, type 'alt + h' for help or 'alt + q or c' to quit\x1B[0m", &current_path);
    } else if search == true {
        if search_input != "" {
            println!("\x1B[35;1m{}\x1B[0m \x1B[2m|->\x1B[0m results for '{}' - exit search with 'alt + s'\x1B[0m", &current_path, &search_input);
        } else {
            println!("\x1B[35;1m{}\x1B[0m \x1B[2m|->\x1B[0m search for contents in this folder... exit search with 'alt + s'\x1B[0m", &current_path);
        }

    } else {
        println!("\x1B[35;1m{}\x1B[0m \x1B[2m|->\x1B[0m \x1B[31;1m{} v{}\x1B[0m \x1B[37menter a path, type 'alt + h' for help or 'alt + q' to quit\x1B[0m", &current_path, env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    }
}
fn main() {
    let mut searching = false;
   
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

    if cfg!(target_os = "windows") {
        set_path("C:\\");
    } else {
        set_path("/");
    }

    print_prompt_message(true, false, "");
    let mut current_input: String = String::new();
    loop {
        let current_dir = env::current_dir().unwrap();
        let current_path = current_dir.to_str().unwrap();
        let (keycode, modifiers) = read_char();
        if let Ok(c) = keycode {
            if modifiers.is_ok() {
                if modifiers.unwrap().contains(KeyModifiers::ALT) {
                    match c {
                        KeyCode::Char('c') => break,
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') => {
                            println!("\x1B[2J\x1B[1;1H");
                            println!("command list:\nalt + q or c - quit the program\nalt + h - brings up this help page\nalt + v - displays current version\nalt + s - toggle searching contents of folder");
                            print_prompt_message(false,false, "");
                        },
                        KeyCode::Char('v') => {
                            println!("\x1B[2J\x1B[1;1H");
                            println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                            print_prompt_message(false,false, "");
                        },
                        KeyCode::Char('s') => {
                            println!("\x1B[2J\x1B[1;1H");
                            if searching == true {
                                searching = false;
                                set_path(&current_path);
                            } else {
                                searching = true;
                                list_dir::list(&current_path, "");
                            }
                            print_prompt_message(false, searching, "");
                        },
                        _ => {}
                    }
                } else {
                    match c {
                        KeyCode::Esc => break,
                        KeyCode::Backspace => {
                            current_input.truncate(current_input.len() - 1);
                            println!("\x1B[2J\x1B[1;1H");
                            
                            if searching == false {
                                list_dir::list(&current_path, "");
                                print_prompt_message(false, searching, &current_input.as_str());
                                println!("{}", &current_input)
                            } else {
                                list_dir::list(&current_path, &current_input);
                                print_prompt_message(false, searching, &current_input.as_str());
                            }
                        }
                        KeyCode::Enter => {
                            println!("\x1B[2J\x1B[1;1H");
                            if searching == false {
                                set_path(&current_input);
                                current_input = "".to_string();
                            } else {
                                let mut found_file = list_dir::find_file(&current_path, &current_input);
                                if &current_input == ".." { found_file = "..".to_string() }
                                if found_file != "" {
                                    set_path(&found_file);
                                    current_input = "".to_string();
                                } else {
                                    list_dir::list(&current_path, &current_input);
                                }
                            }
                            print_prompt_message(false,searching, &current_input.as_str());
                        },
                        KeyCode::Char(ch) => { 
                            current_input = format!("{}{}", current_input, ch);
                            println!("\x1B[2J\x1B[1;1H");
                            
                            if searching == false {
                                list_dir::list(&current_path, "");
                                print_prompt_message(false, searching, &current_input.as_str());
                                println!("{}", &current_input)
                            } else {
                                list_dir::list(&current_path, &current_input);
                                print_prompt_message(false, searching, &current_input.as_str());
                            }
                        }
                        _ => {}
                    }
                    
                }
            }
        }
        
        
    }
}
