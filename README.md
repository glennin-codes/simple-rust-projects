# simple-rust-projects

# 1. File Reader and Writer

- [source code](https://github.com/glennin-codes/simple-rust-projects/tree/main/src)

This Rust project demonstrates file reading and writing operations with custom error handling using the [thiserror](https://crates.io/crates/thiserror) crate.

## Features

- Read content from a specified file
- Then Writes its content to a new file
- Custom error handling for various file operations
- Input validation for file paths

### used ``colo-eyre``.
``color-eyre`` is a Rust library that provides an enhanced error reporting and handling system. It's designed to make error messages more informative and visually appealing, especially in terminal environments. 

Here's what color-eyre does:

1. Colorized Error Messages:
It adds color to error messages, making them easier to read and distinguish in the terminal output.
2. Error Tracing:
It provides detailed error traces, showing the chain of errors that led to the final error.

3. Source Code Snippets:
For errors that occur in your code, it can display relevant source code snippets, helping you quickly locate the issue.

4. Integration with eyre:
It's built on top of the eyre crate, which is an error handling library that provides a Report type (similar to anyhow::Error).

5. Custom Sections:
Allows you to add custom sections to error reports, providing more context about the error.

6. Panic Handling:
It can be set up to handle panics, providing colorized and informative panic messages.

## Prerequisites

- Rust (edition 2021 or later)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:

```sh
git clone https://github.com/glennin-codes/simple-rust-projects.git
```

```sh 
cd simple-rust-projects
```


2. Build the project:
```sh 
cargo build
 ```

## Usage

Run the program with the following command:
```sh
 cargo run -- <input_file> <output_file>
```

- Replace `<input_file>` with the path to the file you want to read, and `<output_file>` with the path where you want to write the content.


Example:
```sh 
cargo run -- home/usr/Documents/input.txt output.txt
```

#### or just run

```sh 
cargo run .
```
- defaults to the `file.txt` in the current directory as the input file and will write to `new.txt` file as the output file.

## Error Handling

This project uses custom error types defined with `thiserror`. The main error types include:

- `FileError::Io`: Wraps standard I/O errors
- `FileError::InvalidInput`: Used for invalid file paths or names
- `FileError::EmptyFile`: Indicates when the input file is empty

## Project Structure

- `main.rs`: Entry point of the application
- `custom_errors.rs`: Defines custom error types using `thiserror`
- `controlers`: Contains functions for reading and writing files - `file_reader.rs` -` write_file.rs`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
