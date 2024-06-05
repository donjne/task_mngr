use std::fs;

pub fn delete_task(filename: &str) -> Result<(), String> {
    if let Err(_) = fs::remove_file(filename) {
        return Err("Failed to delete task file.".to_string());
    }

    println!("Task '{}' deleted successfully.", filename);
    Ok(())
}
