# simple-rust-projects

# 1. File Reader and Writer

- [source code](https://github.com/glennin-codes/simple-rust-projects/tree/main/src)

This Rust project demonstrates file reading and writing operations with custom error handling using the [thiserror](https://crates.io/crates/thiserror) crate.

## Features

- Read content from a specified file
- Then Writes its content to a new file
- Custom error handling for various file operations
- Input validation for file paths

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
