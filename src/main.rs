use dialoguer::{Input, Select};
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("Welcome to Unzipper!\n---");
    let file_path: PathBuf = prompt_to_get_path();
    let file_names = get_file_names(file_path);

    let extract = Select::new()
    .with_prompt("Select your extract method")
    .item("Method 1")
    .item("Method 2")
    .item("Method 3");



    // let target_file_names = prompt_to_get_file_names()

    println!("Unzip files were done! Goodbye!\nUnzipper is ending...");
}

fn prompt_to_get_path() -> PathBuf {
    let file_path_str: String = Input::new()
    .with_prompt("Please input your path")
    .interact_text()
    .expect("Failed to get input");

    let result = PathBuf::from(file_path_str);
    
    if !result.exists() {
        eprintln!("Invalid file path, please try again.");
        return prompt_to_get_path();
    }

    return result;  
}

fn get_file_names(target_path: PathBuf) -> Vec<String> {
    let mut array: Vec<String> = Vec::new();

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