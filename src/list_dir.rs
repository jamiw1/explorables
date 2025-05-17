use std::fs;
use std::fs::DirEntry;

fn push_to_list(entry: DirEntry, search_input: &str, list: &mut Vec<DirEntry>) {
    if search_input != "" {
        if entry.file_name().to_str().unwrap().to_lowercase().contains(&search_input.to_lowercase()) {
            list.push(entry);
        }
    } else {
        list.push(entry);
    }
}

fn get_list(path: &str, search_input: &str) -> (Vec<DirEntry>, Vec<DirEntry>) {
    let mut list_of_files: Vec<DirEntry> = Vec::new();
    let mut list_of_folders: Vec<DirEntry> = Vec::new();

    for entry in fs::read_dir(path.trim().to_string()).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() == true {
            push_to_list(entry, search_input, &mut list_of_folders);
        } else {
            push_to_list(entry, search_input, &mut list_of_files);
        }
    }
    return (list_of_folders, list_of_files);
}
pub fn find_file(path: &str, name: &str) -> String {
    let init_list = get_list(&path, &name);
    let mut list = init_list.0;
    list.extend(init_list.1);
    if list.len() == 1 {
        return list[0].path().display().to_string();
    }
    for file in list {
        let file_name = file.file_name();
        if file_name.to_str().unwrap() == name {
            return file.path().display().to_string();
        }
    }
    
    "".to_string()
}

pub fn list(path: &str, search_input: &str) {
    let list = get_list(&path, &search_input);
    if list.0.len() == 0 && list.1.len() == 0 {
        if search_input == "" {
            println!("\x1B[31;1m{} contains no files or directories\x1B[0m", &path);
        } else {
            println!("\x1B[31;1mno files or directories containing {}\x1B[0m", &search_input);
        }
        
        return;
    }
    for folder in list.0 {
        let path = folder.path();
        let file_name = folder.file_name();
        println!("\x1B[33m{} \x1B[90m<-folder-> \x1B[37m{}\x1B[0m", file_name.into_string().unwrap(), path.display());
    }
    for file in list.1 {
        let path = file.path();
        let file_name = file.file_name();
        println!("\x1B[36m{} \x1B[90m<-file-| \x1B[37m{}\x1B[0m", file_name.into_string().unwrap(), path.display());
    }
}