pub fn handle_whitespace(current_char: &char, index: &mut usize) -> Option<usize> {
    if current_char.is_whitespace() {
        *index += 1;
        return Some(*index); // only return Some if it actually was whitespace
    }
    None
}
