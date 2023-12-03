use std::time::Instant;

pub mod cp {
    pub mod debugger {
        use std::io::BufWriter;
        use std::io::Write;

        // code for setting up the debugger
        pub fn debugger_to_stderr() -> BufWriter<Box<dyn Write>> {
            BufWriter::new(Box::new(std::io::stderr()))
        }

        pub fn debugger_to_file(file_name : &str) -> BufWriter<Box<dyn Write>> {
            let file_path = format!("{}{}", "./../io/", file_name);
            let file_obj = std::fs::File::create(file_path);
            let file = match file_obj {
                Ok(file) => file,
                Err(err) => panic!("Error while opening the file : {}",err)
            };
            BufWriter::new(Box::new(file))
        }

        // logic for doing the debugging
        pub fn _print<T: std::fmt::Debug,E : std::io::Write>(error : &mut E,value: T) {
            write!(error,"{:?}",value).ok();
        }

        #[macro_export]
        macro_rules! debug {
            ($x:expr , $y:expr) => {
                if cfg!(feature = "online")
                {
                    write!($x,"{} = ", stringify!($y)).ok();
                    _print($x,$y);
                    writeln!($x).ok();
                }
            };
        }
    }

    pub mod scanner {
        use std::io::*;
        use std::fs::File;
        #[derive(Default)]
        pub struct Scanner<R> {
            reader : R,
            pub buffer: Vec<String>
        }

        impl<R> Scanner<R> 
        where R : BufRead {

            pub fn new(reader : R) -> Self {
                Scanner { reader, buffer: vec![] }
            }

            pub fn next<T: std::str::FromStr>(&mut self) -> T {
                loop {
                    if let Some(token) = self.buffer.pop() {
                        return token.parse().ok().expect("Failed parse");
                    }
                    let mut input = String::new();
                    self.reader.read_line(&mut input).expect("Failed read");
                    self.buffer = input.split_whitespace().rev().map(String::from).collect();
                }
            }
        }

        pub fn scanner_from_file(file_name : &str) -> Scanner<BufReader<Box<dyn std::io::Read>>> {
            let file_path = format!("{}{}", "./../io/", file_name);
            let file_res = match File::open(&file_path) {
                Ok(file) => file,
                Err(error) => panic!("Problem opening the data file {}: {:?}", file_path, error),
            };
            Scanner::new(BufReader::new(Box::new(file_res) as Box<dyn std::io::Read>))
        }

        pub fn scanner_to_stdin() -> Scanner<BufReader<Box<dyn std::io::Read>>> {
            Scanner::new(BufReader::new(Box::new(std::io::stdin())))
        }
    }

    pub mod console {
        use std::io::BufWriter;
        use std::io::Write;
        pub fn console_to_stdout() -> BufWriter<Box<dyn Write>> {
            BufWriter::new(Box::new(std::io::stdout()))
        }

        pub fn console_to_file(file_name : &str) -> BufWriter<Box<dyn Write>>{
            let file_path = format!("{}{}", "./../io/", file_name);
            let file_obj = std::fs::File::create(file_path);
            let file = match file_obj {
                Ok(file) => file,
                Err(err) => panic!("Error while opening the file : {}",err)
            };
            BufWriter::new(Box::new(file))
        }
    }
}


// use cp::debug;
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
    let start_time = Instant::now();
    // let mut t : i64 = new_scanner.next();
    let mut t = 1;
    while t != 0 {
        solve(&mut new_scanner, &mut output, &mut error);
        t = t-1;
    }
    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    let time_taken = format!("{}ms",elapsed_time.as_millis());
    debug!(&mut error,time_taken);
}