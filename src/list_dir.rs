use std::fs;

pub fn list(path: &str) {
    let mut list_of_files: Vec<fs::DirEntry> = Vec::new();
    let mut list_of_folders: Vec<fs::DirEntry> = Vec::new();

    for entry in fs::read_dir(path.trim().to_string()).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() == true {
            // Yellow color for folders
            list_of_folders.push(entry);
            //println!("\x1B[33m{} \x1B[90m<-folder-> \x1B[37m{}\x1B[0m", file_name.into_string().unwrap(), path.display());
        } else {
            // Cyan color for files
            list_of_files.push(entry);
            //println!("\x1B[36m{} \x1B[90m<-file-| \x1B[37m{}\x1B[0m", file_name.into_string().unwrap(), path.display());
        }
    }
    for folder in list_of_folders {
        let path = folder.path();
        let file_name = folder.file_name();
        println!("\x1B[33m{} \x1B[90m<-folder-> \x1B[37m{}\x1B[0m", file_name.into_string().unwrap(), path.display());
    }
    for file in list_of_files {
        let path = file.path();
        let file_name = file.file_name();
        println!("\x1B[36m{} \x1B[90m<-file-| \x1B[37m{}\x1B[0m", file_name.into_string().unwrap(), path.display());
    }
}