use crate::constants::{VERSION, CYAN, RESET};

pub fn print_version() {
    println!("{CYAN}Rift (Version){RESET}: {}", VERSION);
}
