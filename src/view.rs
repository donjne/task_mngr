use std::fs::File;
use std::io::{ BufRead, BufReader};

pub fn view_task(filename: &str) -> Result<(), String> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Err("Failed to open task file.".to_string()),
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

    Ok(())
}
