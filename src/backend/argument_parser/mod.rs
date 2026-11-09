use crate::backend::argument_parser::functions::{help::print_help, version::print_version};

pub mod functions;


pub struct ArgumentParser {
    pub arguments: Vec<String>
}

#[derive(Debug, Clone)]
pub enum ArgumentEnum {
    Help,
    Version,
    File(String)
}



impl ArgumentParser {
    pub fn new(args: &Vec<String>) -> ArgumentParser {
        Self { arguments: args.clone() }
    }


    pub fn parse_arguments(&mut self) -> Vec<ArgumentEnum> {
        let mut arguments_enum: Vec<ArgumentEnum> = vec![];

        // its gurranteed that the first argument is going to exist so keep it as it is
        let (_, arguments) = &self.arguments.split_first().unwrap();

        for argument in *arguments {
            match argument.as_str() {
                "--help" => arguments_enum.push(ArgumentEnum::Help),
                "--version" => arguments_enum.push(ArgumentEnum::Version),
                file => arguments_enum.push(ArgumentEnum::File(String::from(file))),
            }
        }

        arguments_enum
    }


    pub fn execute_arguments(&mut self) -> Option<String> {
        let arguments_variants = self.parse_arguments();


        for arg in arguments_variants {
            match arg {
                ArgumentEnum::Version => {
                    print_version();
                    return None
                }
                ArgumentEnum::File(file) => {
                    return Some(file)
                }
                ArgumentEnum::Help => {
                    print_help();
                    return None
                }
            }
        }

        None
    }



}
