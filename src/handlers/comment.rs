use std::fs;

fn is_directory(file_path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(file_path) {
        metadata.is_dir()
    } else {
        false
    }
}

pub fn execute_cmt(dir: &str, req_file_type: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {

            if is_directory(entry.path().to_str().unwrap_or(".")) {
                execute_cmt(entry.path().to_str().unwrap_or("."), req_file_type);
                continue;
            }

            if let Some(file_name) = entry.file_name().to_str() {
                println!("Found file: {}", file_name);

                if let Some(extension) = entry.path().extension() {
                    if let Some(extension_str) = extension.to_str() {
                        println!("Text after last dot: {}", extension_str);
                    } else {
                        eprintln!("Failed to convert extension to string.");
                    }
                } else {
                    println!("No extension found in the file name.");
                }
            }

        
        }
    }
}
