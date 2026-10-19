use crate::display_wrapper::{DisplayWithText, ToDisplayWrapper};

#[derive(Debug)]
pub enum Type {
    Atom(Span),
    Struct(Span),
    Ptr(Box<Type>),
    Const(Box<Type>),
    Static(Box<Type>),
    Arr {
        elem: Box<Type>,
        size: Span,
    },
    Func {
        args: Vec<NameTypePair>,
        ret: Box<Type>,
    },
}

impl DisplayWithText for Type {
    fn fmt_with_text(&self, f: &mut std::fmt::Formatter<'_>, text: &str) -> std::fmt::Result {
        match self {
            Self::Atom(span) => write!(f, "{}", span.with_text(text)),
            Self::Struct(span) => write!(f, "struct<{}>", span.with_text(text)),
            Self::Ptr(t) => write!(f, "ptr<{}>", t.with_text(text)),
            Self::Const(t) => write!(f, "const<{}>", t.with_text(text)),
            Self::Static(t) => write!(f, "static<{}>", t.with_text(text)),
            Self::Arr { elem, size } => {
                write!(f, "arr<{}, {}>", elem.with_text(text), size.with_text(text))
            }
            Self::Func { args, ret } => {
                write!(f, "(")?;
                for (i, ntp) in args.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", ntp.with_text(text))?;
                }
                write!(f, ") -> {}", ret.with_text(text))
            }
        }
    }
}

#[derive(Debug)]
pub struct NameTypePair {
    pub name: Span,
    pub type_: Box<Type>,
}

impl DisplayWithText for NameTypePair {
    fn fmt_with_text(&self, f: &mut std::fmt::Formatter<'_>, text: &str) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name.with_text(text),
            self.type_.with_text(text)
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

impl DisplayWithText for Span {
    fn fmt_with_text(&self, f: &mut std::fmt::Formatter<'_>, text: &str) -> std::fmt::Result {
        write!(f, "{}", &text[self.start..self.end])
    }
}
