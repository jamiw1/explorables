use std::env;
use std::fs;
use std::process::Command;
use open::commands;

fn is_executable(path: &str) -> bool {
    let _metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return false,
    };

    if cfg!(target_os = "windows") {
        let ext = path.split('.').last().unwrap_or("");
        return matches!(ext.to_lowercase().as_str(), 
            "exe" | "com" | "bat" | "cmd" | "js" | "vbs" | "wsf" | "ps1"
        );
    }

    false
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
        if is_executable(&path) {
            run_executable(&path);
        } else if commands(&path)[0].status().is_err() {
            return format!("Failed to open file: {}", &path);
        }
        

    }
    return String::new();
    
    
}