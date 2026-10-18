use std::fmt::Display;

pub trait DisplayWithText {
    fn fmt_with_text(&self, f: &mut std::fmt::Formatter<'_>, text: &str) -> std::fmt::Result;
}

pub struct DisplayWrapper<'a, I> {
    text: &'a str,
    item: I,
}

impl<'a, I> DisplayWrapper<'a, I> {
    pub fn new(text: &'a str, item: I) -> Self {
        Self { text, item }
    }
}

impl<'a, T> DisplayWithText for &'a T
where
    T: DisplayWithText,
{
    fn fmt_with_text(&self, f: &mut std::fmt::Formatter<'_>, text: &str) -> std::fmt::Result {
        (*self).fmt_with_text(f, text)
    }
}

impl<'a, I> Display for DisplayWrapper<'a, I>
where
    I: DisplayWithText,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.item.fmt_with_text(f, self.text)
    }
}
