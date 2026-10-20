use crate::display_wrapper::DisplayWithText;
use std::fmt::Display;

#[derive(Debug)]
pub struct ParseError {
    pub loc: usize,
    pub msg: String,
}

impl ParseError {
    pub fn new(loc: usize, msg: String) -> Self {
        Self { loc, msg }
    }

    pub fn find_row_col(&self, text: &str) -> (usize, usize) {
        let mut total: usize = 0;
        for (row, line) in text.lines().enumerate() {
            for (col, _) in line.char_indices() {
                total += 1;
                if total == self.loc {
                    return (row + 1, col + 1);
                }
            }
            total += 1;
        }
        (0, 0)
    }
}

impl DisplayWithText for ParseError {
    fn fmt_with_text(&self, f: &mut std::fmt::Formatter<'_>, text: &str) -> std::fmt::Result {
        let (row, col) = self.find_row_col(text);
        write!(f, "[{}:{}] -> {}", row, col, self.msg)
    }
}

pub struct CliError {
    msg: String,
}

impl CliError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
