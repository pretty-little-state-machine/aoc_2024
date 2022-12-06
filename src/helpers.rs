use std::collections::VecDeque;

/// Parses a sparsely populated text matrix with a filter character to extract input rows and a
/// given text offset for chunking.
///
/// Example:
/// `parse_text_matrix(input, '[', 4, 1)` with the given input:
/// ```
///     [D]
/// [N] [C]
/// [Z] [M] [P]
///  1   2   3
/// ```
///
/// Would return the following Vec<VecDeque<char>>:
/// `[['Z', 'N'], ['M', 'C', 'D'], ['P']]`
///
/// Note: I hate this function, it's really dirty!
pub fn parse_text_matrix(
    input: &str,
    filter: char,
    chunk_size: usize,
    skip_count: usize,
) -> Vec<VecDeque<char>> {
    let mut stacks = vec![VecDeque::new(); 10];
    // Current these stacks are built in row-order so we must flip to columns
    input
        .lines()
        .filter(|line| line.contains(filter))
        .for_each(|line| {
            let columns = parse_text_matrix_line(line, chunk_size, skip_count);
            for (idx, column) in columns.into_iter().enumerate() {
                match column {
                    ' ' => (),
                    _ => stacks.get_mut(idx).unwrap().push_front(column),
                }
            }
        });
    stacks
}

fn parse_text_matrix_line(input: &str, chunk_size: usize, skip_count: usize) -> VecDeque<char> {
    input
        .chars()
        .collect::<VecDeque<char>>()
        .iter()
        .skip(skip_count)
        .step_by(chunk_size)
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text_matrix_line() {
        let two_entries = parse_text_matrix_line("[N] [C]", 4, 1);
        assert_eq!('N', *two_entries.get(0).unwrap());
        assert_eq!('C', *two_entries.get(1).unwrap());
        let first_empty = parse_text_matrix_line("    [D]", 4, 1);
        assert_eq!(' ', *first_empty.get(0).unwrap());
        assert_eq!('D', *first_empty.get(1).unwrap());
    }
}
