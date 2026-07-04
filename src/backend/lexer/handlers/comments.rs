pub fn handle_comments(
    current_char: &char,
    index: &mut usize,
    input: &Vec<(usize, char)>,
) -> Option<usize> {
    if *current_char != '#' {
        return None;
    }
    while *index < input.len() && input[*index].1 != '\n' {
        *index += 1;
    }
    Some(*index)
}
