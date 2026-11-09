use std::fs::File;
use std::io::{Read};



pub struct Config {
    pub file_path: String
}



impl Config {
    pub fn build(file_path: String) -> Self {

        Config {
            file_path: file_path
        }
    }

    pub fn run(&self) -> String {
        let mut file = match File::open(self.file_path.clone()) {
            Ok(file_data)=> file_data,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(content) => content,
            Err(error) => panic!("Couldn't Read File Content: {error:?}")
        };


        return contents
    }
}
