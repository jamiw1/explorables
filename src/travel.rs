use std::env;
use std::fs;
use std::process::Command;

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

pub fn travel(path: &str) {
    if fs::metadata(&path).is_err() {
        println!("{} is not a valid path", &path);
        return;
    }

    if fs::metadata(&path).unwrap().is_dir() {
        if fs::read_dir(&path).is_err() {
            println!("{} is not a valid directory", &path);
        } else {
            env::set_current_dir(&path).expect("Failed to change directory");
        }
    } else if fs::metadata(&path).unwrap().is_file() {
        run_executable(&path);
    }
    
    
}