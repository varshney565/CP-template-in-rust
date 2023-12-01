use std::io::BufWriter;

pub fn debugger_to_stderr() -> BufWriter<std::io::Stderr> {
    BufWriter::new(std::io::stderr())
}

pub fn debugger_to_file(file_name : &str) -> BufWriter<std::fs::File>{
    let file_path = format!("{}{}", "./../io/", file_name);
    let file_obj = std::fs::File::create(file_path);
    let file = match file_obj {
        Ok(file) => file,
        Err(err) => panic!("Error while opening the file : {}",err)
    };
    BufWriter::new(file)
}