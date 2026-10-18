use crate::display_wrapper::DisplayWithText;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    Number,
    PairOpen,
    PairClose,
    Arrow,
    Comment,
    Quote,
    Eof,
    OneChar(char),
}

#[derive(Debug)]
pub struct Token {
    pub start_index: usize,
    pub end_index: usize,
    pub type_: TokenType,
}

impl DisplayWithText for Token {
    fn fmt_with_text(&self, f: &mut std::fmt::Formatter<'_>, text: &str) -> std::fmt::Result {
        write!(
            f,
            "{:?}[{}..{}]: {:?}",
            self.type_,
            self.start_index,
            self.end_index,
            &text[self.start_index..self.end_index]
        )
    }
}
