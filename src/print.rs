use std::fmt::Display;

use crate::{ast::Span, repo::Repo};

pub struct Printer<'a> {
    replace_dests: Repo<Span>,
    replace_srcs: Repo<String>,
    text: &'a str,
}

impl<'a> Printer<'a> {
    pub fn new(replace_dests: Repo<Span>, replace_srcs: Repo<String>, text: &'a str) -> Self {
        Self {
            replace_dests,
            replace_srcs,
            text,
        }
    }
}

impl<'a> Display for Printer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.text.char_indices().peekable();
        let mut replace_srcs_it = self.replace_srcs.data.iter();
        let mut replace_dests_it = self.replace_dests.data.iter();
        while let Some((byte_index, ch)) = it.next() {
            let start_index = replace_dests_it.next();
            let start_index = match start_index {
                Some(span) => span.start,
                None => {
                    write!(f, "{}", ch)?;
                    continue;
                }
            };
            if start_index == byte_index {
                let end_index = replace_dests_it.next().unwrap().end;
                let src = replace_srcs_it.next().unwrap().as_str();
                write!(f, "{}", src)?;
                while let Some((byte_index, _)) = it.peek() {
                    if *byte_index >= end_index {
                        break;
                    }
                    it.next();
                }
                continue;
            }
            write!(f, "{}", ch)?;
        }
        write!(f, "")
    }
}
