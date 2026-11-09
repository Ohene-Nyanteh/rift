use crate::constants::{VERSION, BOLD, RESET, CYAN, DIM};

pub fn print_help() {
    println!("
{CYAN}Rift{RESET} is a lightweight, interpreted scripting language written in Rust. File extension: `.rf`

{BOLD}Rift (Version){RESET}: {DIM} {VERSION} {RESET}

Usage: rift [...file] [...flags]

{BOLD}Commands{RESET}:
    file      ./my-script.rf       Execute a rift file. File should end with .rf


{BOLD}Flags{RESET}:
    --help                         Automatically restart the process on file change
    --version                      Display this menu and exit
    ");
}
