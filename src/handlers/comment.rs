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
        for entry in entries {
            if let Ok(entry) = entry {
                // Get the file name as a String

                if let Some(file_dir) = entry.path().to_str() {
                    if is_directory(file_dir) {
                        execute_cmt(file_dir, req_file_type)
                    } else {
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

                            // let contents = fs::read_to_string(entry.path())
                            //     .expect("Should have been able to read the file");
                        }
                    }
                }
            }
        }
    }
}
