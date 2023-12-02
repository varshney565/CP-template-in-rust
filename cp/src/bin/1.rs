use cp::debug;
use cp::debugger::*;
use cp::console::*;
use cp::scanner::Scanner;
use std::io::Write;
use cp::scanner;

fn solve<R : std::io::BufRead, W : std::io::Write>(scanner : &mut Scanner<R>,output : &mut W,error : &mut W) {

}

fn main() {
    let mut new_scanner: Scanner<std::io::BufReader<Box<dyn std::io::Read>>>;
    let mut output: std::io::BufWriter<Box<dyn Write>>;
    let mut error: std::io::BufWriter<Box<dyn Write>>;

    if cfg!(feature = "online") {
        new_scanner = scanner::scanner_from_file("input.txt");
        output = console_to_file("output.txt");
        error = debugger_to_file("error.txt");
    } else {
        new_scanner = scanner::scanner_to_stdin();
        output = console_to_stdout();
        error = debugger_to_stderr();
    }
    let mut t : i64 = new_scanner.next();
    // let mut t = 1;
    debug!(&mut error,t);
    while t != 0 {
        solve(&mut new_scanner, &mut output, &mut error);
        t = t-1;
    }
}