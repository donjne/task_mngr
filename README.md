# A Task Manager Command Line Tool

This is a basic project for beginners to practice the rust concepts learned after reading, the Book.

Note: This project expects you to have basic knowledge of rust. It will not go into the details of language features but will guide you through the process of building.

Let's build!

## How to get started

Firstly, clone this repository so you can locally access a copy of this markdown file.

```shell
git clone "https://github.com/donjne/task_mngr.git"
```

Next, let's talk about what we're building.

## What we're building

Task manager is a basic rust project that allows you to:

Create tasks that would be saved in a file - using the command below

```shell
add \\<title>\\<description>\\<due_date>
```

View tasks that have been created - using the command,

```shell
view <nameoffile>
```

Delete tasks from directory - using the command,

```shell
delete <nameoffile>
```

Filter tasks that are due, complete, upcoming or incomplete.

```shell
filter <due || incomplete>
```

## Important things to note

A brief explanation on these keywords used for these projects;

pub, use, std, chrono, mod, loop, match

You can find out more about them in the rust documentation

let's talk about each of the modules before we talk about our main.rs (root crate)

## add.rs

### Crates Used

- `std::fs::File`: Provides functionality for creating and managing files.
- `std::io`: Offers input/output operations.
- `chrono::prelude::*`: Provides date and time handling functionalities. We had to add it as a dependency to our project's Cargo.toml

### Adding chrono as a dependency

Chrono will be the only dependency we will have to add to our Cargo.toml file. Other crates are provided by the standard library.

```rust
[dependencies]
chrono = "0.4"
```

## Parsing and checking our input

1. **Parsing Input**: Split the input string into parts: title, description, and due date.
2. **Input Validation**: Check if the input format is correct.
3. **Date Parsing**: Convert the due date string into the `NaiveDate` object.

## Creating a file for a new task

4. **File Creation**: Prompt the user for a filename, create a file with the provided name, and handle errors.
5. **Write Task Details**: Write task details (title, description, and due date) into the file.

## Printing a success message or throwing an error

6. **Success Message**: Inform the user about the successful addition of the task.

### Parsing Input

The input string is split using the escape sequence `\\\\` to extract the title, description, and due date parts.

```rust
    let parts: Vec<&str> = input.split("\\\\").collect();
```

### Input Validation

The length of the parts vector is checked to ensure that all required fields are present. If not, an error message is returned from the `Result`.

```rust
if parts.len() != 4 {
    return Err("Invalid input format. Use: add \\\\<title>\\\\<description>\\\\<due_date>".to_string());
}
```

### Date Parsing

The due date string is parsed into a `NaiveDate` object using the specified date format `%Y-%m-%d`. If parsing fails, an error message is returned. The `NaiveDate` is from our chrono crate which we added to our crate as a dependency and in our add.rs file using the use keyword.

```rust
use chrono::prelude::*;
```

You can find out more information about chrono and other objects in the documentation.

```rust
let due_date = NaiveDate::parse_from_str(parts[3].trim(), "%Y-%m-%d");
```

### File Creation

The user is prompted to input a filename. Then, a file with the provided name is created. Error handling is performed using the `match` expression.

```rust
let mut filename = String::new();
io::stdin().read_line(&mut filename).expect("Failed to read line");
let filename = filename.trim();

let mut file = match File::create(filename) {
    Ok(file) => file,
    Err(_) => return Err("Failed to create task file.".to_string()),
};
```

### Write Task Details

The task details (title, description, and due date) are written to the file using the `writeln!` macro. If an error occurs during writing, an error message is returned.

```rust
if let Err(_) = writeln!(file, "Title: {}\nDescription: {}\nDue Date: {}", title, description, parts[3]) {
    return Err("Failed to write task details to file.".to_string());
}
```

Note: We used the `.to_string()` method to convert our &str to a `String` type because our `Result` enum returns an error of type `String`

### Add success

After creating a successful task, the message would print, Task task_name added successfully.

```rust
println!("Task '{}' added successfully.", title);
```

We can test by creating a task named 'work task'.

#### Adding a new task

```shell
add \\work task\\Just research and smart contract development\\2024-07-17
```

#### Giving our task file a name

```shell
Give your file a name and extension you will easily remember:
workTask.txt
```

```shell
Task 'work task' added successfully.
```

## Alternative methods of error handling

The code uses simple error handling with `Result`. Another method is to use `panic!` or other custom error types or even crates such as `thiserror` or `anyhow` could be used. However, for our program, we'll be sticking to the `Result` enum for error handling.

## if let statement

Because of the excessive use of the `if let` statement in our program, let's examine it briefly.

The if let statement is a concise way to handle pattern matching when you're only interested in one specific pattern and want to ignore all others.

Using code samples:

```rust
if let Ok(_due_date) = due_date
```

This line below checks if the `due_date` variable contains a successful parsing result `(Ok)`. If so, it extracts the value from the Ok variant into the `_due_date` variable (the underscore indicates that we're not using the value in this case). If the `due_date` contains an `Err` from our `Result`, the `if let` block is skipped, and the program proceeds to the `else` block.

## delete.rs

### File deletion and Error handling

The `fs::remove_file` function is used to attempt to delete the task file specified by the filename. The result of the file deletion operation is checked using `if let Err(_) = fs::remove_file(filename)`. If an error occurs during file deletion, an error message is returned.

```rust
    if let Err(_) = fs::remove_file(filename) {
        return Err("Failed to delete task file.".to_string());
    }
```

### Deletion success

If the file deletion operation is successful, a success message is printed to inform the user.

```rust
println!("Task '{}' deleted successfully.", filename);
```

### The unit type

In Rust, `()` is the unit type, and it represents absence of a meaningful value. It's similar to void in other languages like C or C++.

Returning `Ok(())` from a function with a `Result<(), T>` type indicates that the function executed successfully and doesn't have any meaningful value to return. Essentially, it's a way to signal success without returning any data.

```rust
Ok(())
```

`Ok` signifies that the operation was successful while the unit type (a primitive type provided by rust) `()` indicates that there is no meaningful data associated with the success. It's used here because the function `delete_task` doesn't need to return any specific data upon success. It just needs to indicate whether the deletion was successful or not.

## filter.rs

### Read Task Directory

The `fs::read_dir(".")` function is used to read the current directory and obtain a directory iterator.

```rust
    // Read the current directory
    let dir = match fs::read_dir(".") {
        Ok(dir) => dir,
        Err(_) => return Err("Failed to read task directory.".to_string()),
    };
```

### Get Current Date

`Local::now().naive_local()` is used to get the current system date and time in the local timezone and we assign it to a variable `current_date`. The `naive_local()` method converts it into a `NaiveDateTime` which is suitable for comparison with due dates.

```rust
let current_date = Local::now().naive_local();
```

### Iterate Over Files

The `for` loop iterates over each entry in the directory iterator, where each entry represents a file in the directory. The code uses a `for` loop instead of other iteration methods like `map` or `filter` for simplicity and readability. While other methods may offer more concise code, using a `for` loop in this case keeps the code straightforward and easier to understand, especially for those less familiar with Rust or functional programming concepts.

```rust
 for entry in dir {
    // --snip--
 }
 ```

### Filter Tasks

Depending on the specified `filter_type`, tasks are filtered either by their due date (`"due"`) or upcoming tasks (`"upcoming"`). For each task file, its contents are checked to extract the due date. Based on the filter type, tasks are filtered and printed accordingly.

```rust
match filter_type {
    // --snip--
}
```

### Display Filtered Tasks

Filtered tasks are printed with their title and path. We had also used the `.into()` method for our `current_date` variable so the types we're comparing would match

```rust
    // Check if the task is completed and due date is in the future
    if contents.contains("[Complete]") || due_date <= current_date.into() {
    found_tasks = true;
    println!("Title: {}", file_name);
    println!("Path: {}", path.display());
}
```

### filter success

If no tasks are found based on the specified filter type, a message indicating the absence of tasks is printed.

```rust
    // Print a message if no tasks are found
    if !found_tasks {
        println!("No {} tasks found.", filter_type);
    }
```

## view.rs

### Opening Task File

The `File::open` function is used to open the specified task file. If successful, a `File` object representing the opened file is returned.

### Reading Task Content

A `BufReader` is created to efficiently read the contents of the file. The `lines` method of `BufReader` returns an iterator over the lines of the file. `BufReader` is used to wrap a `File` or any other type that implements the `Read` trait. It provides buffered reading capabilities, which can improve I/O performance by reducing the number of system calls made to read data from the underlying source.

```rust
    let reader = BufReader::new(file);
```

### Printing Task Content

Each line of the task content is printed to the console using a `for` loop.

```rust
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
```

### View success

If viewing the task content is successful, an `Ok(())` is returned.

```rust
    Ok(())
```

## main.rs - Main program

This is the control center for our program

### Importing our modules

We split our program into different modules so it can be easily managed.

```rust
mod add;
mod view;
mod delete;
mod filter;
```

We also used the `std::io` crate.

```rust
use std::io
```

### Welcome Message

The program starts by displaying a welcome message to the user, indicating the start of the Rusty Task Manager.

```rust
println!("Welcome to Rusty Task Manager!\n");
```

### Command Loop

The program enters a loop where it continuously prompts the user for commands and waits for input.

```rust
loop {
    // --snip--
}
```

### Command Parsing

User input is read from the standard input (`stdin`) and split into parts based on whitespace. The first part of the input represents the command, and subsequent parts represent the command arguments.

```rust
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
```

### Command Execution

Based on the parsed command, the program executes the corresponding function from the `add`, `view`, `delete`, or `filter` modules. Error handling is implemented for each command to handle potential errors that could occur.

```rust
    match parts[0] {
        "add" => {
            if let Err(err) = add::add_task(&input) {
                println!("{}", err);
            }
        }
        // --snip--
    }
```

## Running our program

Now that we're done understanding the program, let's run our program using the command:

```shell
cargo run
```

After compiling:

```shell
Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.00s
Running `target\debug\task_mngr.exe`
Welcome to Rusty Task Manager!

Commands:
- add \\<title>\\<description>\\<due_date>
- view <filename>
- delete <filename>
- filter <due | upcoming>
```

## The End

That's a wrap. Hat's off to you for following up till the end.
