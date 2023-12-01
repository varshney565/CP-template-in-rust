use std::fs::File;
use std::io::{self, Write, BufWriter};

trait Output {
    fn new(output_target: &str) -> Self;
}

impl Output for BufWriter<File> {
    fn new(output_target: &str) -> Self {
        let file = File::create(output_target).expect("Failed to create file");
        BufWriter::new(file)
    }
}

impl Output for BufWriter<io::Stdout> {
    fn new(_output_target: &str) -> Self {
        BufWriter::new(io::stdout())
    }
}

impl Output for BufWriter<io::Stderr> {
    fn new(_output_target: &str) -> Self {
        BufWriter::new(io::stderr())
    }
}

fn console_to_output<T: Output>(output_target: &str) -> T {
    T::new(output_target)
}

fn main() {
    let mut output_file: BufWriter<File> = console_to_output("./../io/output.txt");
    let mut error_file: BufWriter<File> = console_to_output("./../io/error.txt");

    let mut output_console: BufWriter<io::Stdout> = console_to_output("");
    let mut error_console: BufWriter<io::Stderr> = console_to_output("");

    // Now you can use output_file, error_file, output_console, error_console as needed
    writeln!(output_file, "Hello, File!").expect("Failed to write to file");
    writeln!(error_file, "Error in File!").expect("Failed to write error to file");

    writeln!(output_console, "Hello, Console!").expect("Failed to write to console");
    writeln!(error_console, "Error in Console!").expect("Failed to write error to console");
}
