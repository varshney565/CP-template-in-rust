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