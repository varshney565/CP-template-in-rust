use std::io::{BufWriter,stdout,Write};
use cp::scanner;

fn main() {
    // let files = Files::new(String::from("input.txt"),String::from("output.txt"),String::from("error.txt"));
    let mut new_scanner = scanner::scanner_from_file("input.txt");
    // let input = files.input;
    // let mut output = files.output;
    // let mut error = files.error;
    let mut output = BufWriter::new(std::io::stdout());
    let n : i32 = new_scanner.next();
    let mut v:Vec<i32> = (0..n).map(|_| new_scanner.next()).collect();
    for element in &mut v {
        write!(output, "{} ",element).ok();
    }
    write!(output,"\n").ok();
}