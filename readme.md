# Rust Competitive Programming Template

This Rust template provides a basic structure for writing competitive programming solutions with debugging and file I/O capabilities. It includes modules for debugging, file scanning, and console output. The template uses the standard Rust library and supports reading input from either standard input or files, writing output to either standard output or files, and debugging information to standard error or files.

## Code Explanation

### Debugger Module (`cp::debugger`)

This module provides functionality for debugging, allowing you to print debug information to standard error or files.

- `debugger_to_stderr`: Sets up the debugger to write to standard error.
- `debugger_to_file(file_name: &str)`: Sets up the debugger to write to a specified file.
- `_print<T: std::fmt::Debug, E: std::io::Write>(error: &mut E, value: T)`: Internal function for printing debug information.

### Scanner Module (`cp::scanner`)

This module handles input scanning, allowing you to read input from standard input or files.

- `Scanner<R>`: A generic struct for scanning input from a reader of type `R`.
- `new(reader: R)`: Creates a new `Scanner` instance with the provided reader.
- `next<T: std::str::FromStr>(&mut self) -> T`: Reads the next token from the input and parses it into the specified type.
- `scanner_from_file(file_name: &str)`: Creates a `Scanner` instance reading from a specified file.
- `scanner_to_stdin()`: Creates a `Scanner` instance reading from standard input.

### Console Module (`cp::console`)

This module handles console output, allowing you to write output to standard output or files.

- `console_to_stdout()`: Sets up the console to write to standard output.
- `console_to_file(file_name: &str)`: Sets up the console to write to a specified file.

### Main Program (`main`)

The main program demonstrates how to use the provided modules and macros to implement a simple solution.

- Reads input from either standard input or a file, depending on the configuration.
- Sets up output to either standard output or a file, depending on the configuration.
- Measures the time taken to execute the solution.

### Usage

The template provides flexibility for online and local testing.

- For local testing, modify the file paths in the configuration section and run the program with `cargo run --bin <YOUR_BINARY_NAME>`.
- For online testing, set the feature flag and run the program with `cargo run --features=online --bin <YOUR_BINARY_NAME>`.
