pub fn handle_newline(current_char: &char, index: &mut usize, row: &mut usize) -> Option<usize> {
    if *current_char == '\n' {
        *row += 1; // skip and return updated index
        *index += 1;
        Some(*row)
    }
    else {
        None
    }


}
