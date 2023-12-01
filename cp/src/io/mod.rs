use std::{fs::File, io::*};

use crate::online_judge;
pub struct Files {
    pub input : Box<dyn BufRead>,
    pub output : Box<dyn Write>,
    pub error : Box<dyn Write>
}

impl Files {
    pub fn new(input : String,output : String,error : String) -> Self {
        if online_judge::ONLINE_JUDGE {
            Files {
                input: Box::new(BufReader::new(stdin())),
                output: Box::new(BufWriter::new(stdout())),
                error: Box::new(BufWriter::new(stderr())),
            }
        } else {
            Files {
                input: Box::new(BufReader::new(Self::open_file_read(input))),
                output: Box::new(BufWriter::new(Self::open_file_write(output))),
                error: Box::new(BufWriter::new(Self::open_file_write(error))),
            }
        }
    }

    fn open_file_read(file_name: String) -> BufReader<File> {
        let file_path = format!("{}{}", "./../../../io/", file_name);
        let file_res = match File::open(&file_path) {
            Ok(file) => BufReader::new(file),
            Err(error) => panic!("Problem opening the data file {}: {:?}", file_path, error),
        };
        file_res
    }

    fn open_file_write(file_name: String) -> File {
        let file_path = format!("{}{}", "./../../../io/", file_name);
        let file_res = match File::open(&file_path) {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the data file {}: {:?}", file_path, error),
        };
        file_res
    }
}