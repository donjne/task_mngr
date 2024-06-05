mod add;
mod view;
mod delete;
mod filter;

use std::io;

fn main() {
    println!("Welcome to Rusty Task Manager!\n");

    loop {
        println!("Commands:");
        println!("- add \\\\<title>\\\\<description>\\\\<due_date>");
        println!("- view <filename>");
        println!("- delete <filename>");
        println!("- filter <due | upcoming>\n");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts[0] {
            "add" => {
                if let Err(err) = add::add_task(&input) {
                    println!("{}", err);
                }
            }
            "view" => {
                if parts.len() != 2 {
                    println!("Invalid input format. Use: view <filename>");
                } else {
                    if let Err(err) = view::view_task(parts[1]) {
                        println!("{}", err);
                    }
                }
            }
            "delete" => {
                if parts.len() != 2 {
                    println!("Invalid input format. Use: delete <filename>");
                } else {
                    if let Err(err) = delete::delete_task(parts[1]) {
                        println!("{}", err);
                    }
                }
            }
            "filter" => {
                if parts.len() != 2 {
                    println!("Invalid input format. Use: filter <completed | upcoming>");
                } else {
                    if let Err(err) = filter::filter_tasks(parts[1]) {
                        println!("{}", err);
                    }
                }
            }
            _ => println!("Invalid command"),
        }
    }
}
