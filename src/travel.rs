use std::env;
use std::fs;
use std::process::Command;

fn open_file(path: &str) -> Result<(), String> {
    let os = env::consts::OS;
    
    match os {
        "windows" => {
            Command::new("start")
                .arg(path)
                .spawn()
                .map_err(|e| format!("Failed to open file: {}", e))
                .map(|_| ())
        }
        "macos" => {
            Command::new("open")
                .arg(path)
                .spawn()
                .map_err(|e| format!("Failed to open file: {}", e))
                .map(|_| ())
        }
        "linux" => {
            Command::new("xdg-open")
                .arg(path)
                .spawn()
                .map_err(|e| format!("Failed to open file: {}", e))
                .map(|_| ())
        }
        _ => Err(format!("Unsupported OS: {}", os)),
    }
}

fn run_executable(path: &str) {
    let output = Command::new(path)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
       // println!("Command executed successfully");
    } else {
        println!("Command failed with status: {}", output.status);
    }
}

pub fn travel(path: &str) -> String {
    if fs::metadata(&path).is_err() {
        return format!("{} is not a valid file/directory", &path);
    }

    if fs::metadata(&path).unwrap().is_dir() {
        if fs::read_dir(&path).is_err() {
            return format!("{} is not a valid file/directory:", &path);
        } else {
            env::set_current_dir(&path).expect("Failed to change directory");
        }
    } else if fs::metadata(&path).unwrap().is_file() {
        if open_file(&path).is_err() {
            return format!("Failed to open file: {}", &path);
        }
    }
    return String::new();
    
    
}