use std::fs;

pub fn list(path: &str) {
    for entry in fs::read_dir(path.trim().to_string()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = entry.file_name();
        if entry.file_type().unwrap().is_dir() == true {
            // Yellow color for folders
            println!("\x1B[33m{} \x1B[90m<-folder-> \x1B[37m{}\x1B[0m", file_name.into_string().unwrap(), path.display());
        } else {
            // Cyan color for files
            println!("\x1B[36m{} \x1B[90m<-file-| \x1B[37m{}\x1B[0m", file_name.into_string().unwrap(), path.display());
        }
    }   
}