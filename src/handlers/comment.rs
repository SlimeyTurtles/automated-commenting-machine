use std::fs;

fn is_directory(file_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.is_dir()
    } else {
        false
    }
}

fn modify_file(dir: &str) {
    match fs::read_to_string(dir) {
        Ok(contents) => {
            // Successfully read the file contents
            println!("File contents:\n{}", contents);
        }
        Err(e) => {
            // Handle the error if the file cannot be read
            eprintln!("Error reading file: {}", e);
        }
    }
}

pub fn execute_cmt(dir: &str, req_file_type: &str) {
    if !is_directory(dir) {
        modify_file(dir);
        return;
    }
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            execute_cmt(entry.path().to_str().unwrap_or("."), req_file_type);
            continue;
        }
    }
}
