use std::fs::File;
use std::io::{Read};



pub struct Config {
    file_path: String
}



impl Config {
    pub fn build(args: &Vec<String>) -> Self {
        if args.len() > 2 {
            eprintln!("Error: Can't Pass more than 1 argument");
            panic!();
        }

        Config {
            file_path: args[1].clone()
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
