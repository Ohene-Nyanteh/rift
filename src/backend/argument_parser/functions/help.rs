use crate::constants::{VERSION, BOLD, RESET, CYAN, DIM};

pub fn print_help() {
    println!("
{CYAN}Rift{RESET} is a lightweight, interpreted scripting language written in Rust. File extension: `.rf`

{BOLD}Rift (Version){RESET}: {DIM} {VERSION} {RESET}

Usage: rift <file> [flags]

{BOLD}Commands{RESET}:
    <file>      ./my-script.rf         Execute a Rift file

{BOLD}Flags{RESET}:
    --help                             Show this help menu and exit
    --version                          Show the version and exit
    --upgrade                          Update Rift if a newer version exists
    ");
}
