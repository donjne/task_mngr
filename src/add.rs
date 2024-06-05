use std::fs::File;
use std::io::{self, Write};
use chrono::prelude::*;

pub fn add_task(input: &str) -> Result<(), String> {
    let parts: Vec<&str> = input.split("\\\\").collect();
    if parts.len() != 4 {
        return Err("Invalid input format. Use: add \\\\<title>\\\\<description>\\\\<due_date>".to_string());
    }

    let title = parts[1].trim();
    let description = parts[2].trim();
    let due_date = NaiveDate::parse_from_str(parts[3].trim(), "%Y-%m-%d");

    if let Ok(_due_date) = due_date {
        println!("Give your file a name and extension you will easily remember:");
        let mut filename = String::new();
        io::stdin().read_line(&mut filename).expect("Failed to read line");
        let filename = filename.trim();

        let mut file = match File::create(filename) {
            Ok(file) => file,
            Err(_) => return Err("Failed to create task file.".to_string()),
        };

        // Write task details to the file
        if let Err(_) = writeln!(file, "Title: {}\nDescription: {}\nDue Date: {}", title, description, parts[3]) {
            return Err("Failed to write task details to file.".to_string());
        }

        println!("Task '{}' added successfully.", title);
        Ok(())
    } else {
        Err("Invalid date format. Use format: YYYY-MM-DD".to_string())
    }
}
