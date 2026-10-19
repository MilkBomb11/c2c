use std::fmt::Display;

use crate::display_wrapper::DisplayWithText;

pub struct Repo<T> {
    pub data: Vec<T>,
}

impl<T> Repo<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

impl<T> DisplayWithText for Repo<T>
where
    T: DisplayWithText,
{
    fn fmt_with_text(&self, f: &mut std::fmt::Formatter<'_>, text: &str) -> std::fmt::Result {
        write!(f, "---START=Spans to be replaced---\n")?;
        for item in self.data.iter() {
            item.fmt_with_text(f, text)?;
            write!(f, "\n")?;
        }
        write!(f, "---END=Spans to be replaced---\n")
    }
}

impl<T> Display for Repo<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "---START---\n")?;
        for item in self.data.iter() {
            item.fmt(f)?;
            write!(f, "\n")?;
        }
        write!(f, "---END---\n")
    }
}
