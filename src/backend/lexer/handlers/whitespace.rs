pub fn handle_whitespace(current_char: &char, index: &mut usize) -> Option<usize> {
    if current_char.is_whitespace() {
        *index += 1;
        Some(*index) // skip and return updated index
    } else {
        None
    }
}
