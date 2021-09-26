use unicode_width::UnicodeWidthStr;

/// Split a string on either the whitespace character closest
/// to max length, or split it on max length if there is no 
/// whitespace available.
fn split_to_len(mut line: &str, max_width: usize) -> (&str, &str) {
    let split_pos = &line[..max_width]
        .rfind(char::is_whitespace)
        .unwrap_or(max_width);
    let (lhs, rhs) = line.split_at(*split_pos);
    (lhs, rhs)
}

/// Split lines to fit the screen.
fn split_lines(mut line: &str, max_width: usize) -> Vec<&str> {
    let mut lines = Vec::new();

    while line.width() > max_width {
        let (lhs, rhs) = split_to_len(line, max_width);

        lines.push(lhs.trim_start());
        line = rhs;
    }

    lines.push(line.trim_start());

    lines
}

/// Split the input into lines that will fit on screen,
/// also break on newline chars.
pub(super) fn lines(input: &str, max_width: usize) -> Vec<&str> {
    let lines = input
        .split('\n')
        .map(|line| split_lines(line, max_width))
        .flatten()
        .collect::<Vec<_>>();
    lines
}



