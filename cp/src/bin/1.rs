use cp::debugger::*;
use cp::console::*;
use std::io::Write;
use cp::scanner;

fn main() {
    let mut new_scanner;
    let mut output;
    let mut error;

    if cfg!(feature = "debug") {
        new_scanner = scanner::scanner_from_file("input.txt");
        output = console_to_file("output.txt");
        error = debugger_to_file("error.txt");
    } else {
        new_scanner = scanner::scanner_to_stdin();
        output = console_to_stdout();
        error = debugger_to_stderr();
    }

    let n : i32 = new_scanner.next();
    let mut v:Vec<i32> = (0..n).map(|_| new_scanner.next()).collect();
    for element in &mut v {
        write!(output, "{} ",element).ok();
        write!(error,"{}D",element).ok();
    }
    write!(output,"\n").ok();
}



// ==============================================> 
// Testing
// fn main() {
//     if cfg!(feature = "debug") {
//         println!("DEBUG MODE ON !!");
//     }else {
//         println!("online_judge mode !!");
//     }
// }