use crate::entry::Entry;
use serde_json;
use std::path::Path;
use std::{env, fs};

mod entry;
mod exercise;

fn main() {
    // Collect command line arguments into a vector.
    let args: Vec<String> = env::args().collect();

    // Check if the arguments contain "--input"
    if args.contains(&"--input".to_string()) {
        execute_data().unwrap_or_else(|e| eprintln!("Error: {}", e));
    } else {
        println!(
            "Usage: {} --input",
            args.get(0).unwrap_or(&"program".to_string())
        );
    }
}

fn execute_data() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "data.json";
    let new_exercises = exercise::Exercise::read();

    let all_exercises: Vec<exercise::Exercise> = if Path::new(file_path).exists() {
        // Read the file contents
        let file_content = fs::read_to_string(file_path)?;
        // Deserialize existing data or propagate any serde errors
        let mut existing: Vec<exercise::Exercise> = serde_json::from_str(&file_content)?;
        // Append the new exercises
        existing.extend(new_exercises);
        existing
    } else {
        new_exercises
    };

    // Serialize the updated data into pretty JSON.
    let json = serde_json::to_string_pretty(&all_exercises)?;
    // Write the JSON data back to the file.
    fs::write(file_path, json)?;
    Ok(())
}
