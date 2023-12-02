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