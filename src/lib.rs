use dialoguer::{Input, Select};
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

pub fn run() {
    // Get target path from prompts
    let file_path: PathBuf = prompt_to_get_path();

    // Find out all file names under given dir path
    let file_names = get_file_names(file_path);
    let filtered_file_name = filter_archive_file(file_names);
    // Check if has .rar/.zip/.7z
    if filtered_file_name.is_empty() {
        println!(
            "There's no archive file under this directory or by given file name.\nTerminating..."
        );
        return;
    }

    // Process unzipping work
    let extract = Select::new()
        .with_prompt("Select your extract method")
        .item("Method 1")
        .item("Method 2")
        .item("Method 3");

    // Try to look for password txt file for unarchive
}

fn get_file_names(target_path: PathBuf) -> Vec<String> {
    let mut array: Vec<String> = Vec::new();

    // Check if path is dir/file
    if target_path.is_file() {
        println!("Receiving a file path...");
        if let Some(file_name) = target_path.to_str() {
            println!("Got file path: {}", file_name);
            array.push(String::from(file_name));
        }

        return array;
    }

    println!("Receiving a directory, taking out file names under it...");
    if let Ok(entries) = fs::read_dir(target_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    println!("Got file path: {}", file_name);
                    array.push(String::from(file_name))
                }
            }
        }
    }

    return array;
}

fn filter_archive_file(path_names: Vec<String>) -> Vec<String> {
    let mut valid_path_names: Vec<String> = Vec::new();
    for path_name in path_names {
        let path = Path::new(&path_name);
        if let Some(ext) = path.extension() {
            if ext == "rar" || ext == "7z" || ext == "zip" {
                valid_path_names.push(path_name);
            }
        }
    }

    return valid_path_names;
}

fn get_password_from_file(file_path: &String) -> Result<Vec<String>, Error> {
    let mut passwords: Vec<String> = Vec::new();
    let passwords_file = match File::open("file_path") {
        Ok(file) => file,
        Err(err) => panic!("Failed to open passwords file: {:?}", err),
    };

    passwords_file.read_to_string(passwords);
    return Ok(passwords);
}


pub fn prompt_to_get_path() -> PathBuf {
    loop {
        let file_path: String = match Input::new()
            .with_prompt("Please input your path")
            .interact_text()
        {
            Ok(file_path) => file_path,
            Err(err) => {
                eprintln!("Failed to get input: {:?}", err);
                continue;
            }
        };

        let result = PathBuf::from(file_path);

        if !result.exists() {
            eprintln!("Invalid file path, please try again.");
            continue;
        }

        return result;
    }
}

#[cfg(test)]
mod passwords_tests {
    #[test]
    fn has_password() {}

    #[test]
    fn is_valid_password_file() {}
}

pub fn find_password(file: File) -> bool {
    return false;
}
