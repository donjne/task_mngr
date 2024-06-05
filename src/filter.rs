use std::fs;
use chrono::{Local, NaiveDate};

pub fn filter_tasks(filter_type: &str) -> Result<(), String> {
    let dir = match fs::read_dir(".") {
        Ok(dir) => dir,
        Err(_) => return Err("Failed to read task directory.".to_string()),
    };

    // Get the current system date
    let current_date = Local::now().naive_local();

    // Flag to check if any tasks were found
    let mut found_tasks = false;

    for entry in dir {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "txt" {
                    let file_name = path.file_stem().unwrap().to_str().unwrap();
                    let contents = fs::read_to_string(&path).unwrap();

                    // Parse due date from file content
                    if let Some(due_date_line) = contents.lines().find(|line| line.starts_with("Due Date:")) {
                        if let Some(due_date_str) = due_date_line.split(":").nth(1) {
                            if let Ok(due_date) = NaiveDate::parse_from_str(due_date_str.trim(), "%Y-%m-%d") {
                                match filter_type {
                                    "due" => {
                                        // Check if the task is completed and due date is in the future
                                        if contents.contains("[Complete]") || due_date <= current_date.into() {
                                            found_tasks = true;
                                            println!("Title: {}", file_name);
                                            println!("Path: {}", path.display());
                                        }
                                    }
                                    "upcoming" => {
                                        // Check if the task is incomplete and due date is in the past
                                        if contents.contains("[Incomplete]") || due_date > current_date.into() {
                                            found_tasks = true;
                                            println!("Title: {}", file_name);
                                            println!("Path: {}", path.display());
                                        }
                                    }
                                    _ => return Err("Invalid filter option.".to_string()),
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !found_tasks {
        println!("No {} tasks found.", filter_type);
    }

    Ok(())
}
