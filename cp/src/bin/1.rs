use cp::debugger::*;
use cp::console::*;
use std::io::Write;
use cp::scanner;

fn main() {
    let mut new_scanner = scanner::scanner_from_file("input.txt");
    let mut output: std::io::BufWriter<std::fs::File> = console_to_file("output.txt");
    let mut error: std::io::BufWriter<std::fs::File> = debugger_to_file("error.txt");
    // let mut output: std::io::BufWriter<std::io::Stdout> = console_to_stdout();
    // let mut error: std::io::BufWriter<std::io::Stderr> = debugger_to_stderr();
    let n : i32 = new_scanner.next();
    let mut v:Vec<i32> = (0..n).map(|_| new_scanner.next()).collect();
    for element in &mut v {
        write!(output, "{} ",element).ok();
        write!(error,"{}D",element).ok();
    }
    write!(output,"\n").ok();
}