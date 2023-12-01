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

pub fn scanner_from_file(file_name : &str) -> Scanner<BufReader<std::fs::File>> {
    let file_path = format!("{}{}", "./../io/", file_name);
    let file_res = match File::open(&file_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file {}: {:?}", file_path, error),
    };
    Scanner::new(BufReader::new(file_res))
}