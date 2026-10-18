use crate::display_wrapper::{DisplayWithText, DisplayWrapper};

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
            Self::Atom(span) => {
                let wrapper: DisplayWrapper<&Span> = DisplayWrapper::new(text, span);
                write!(f, "{}", wrapper)
            }
            Self::Struct(span) => {
                let wrapper: DisplayWrapper<&Span> = DisplayWrapper::new(text, span);
                write!(f, "struct<{}>", wrapper)
            }
            Self::Ptr(t) => {
                let wrapper: DisplayWrapper<&Type> = DisplayWrapper::new(text, t.as_ref());
                write!(f, "ptr<{}>", wrapper)
            }
            Self::Const(t) => {
                let wrapper: DisplayWrapper<&Type> = DisplayWrapper::new(text, t.as_ref());
                write!(f, "const<{}>", wrapper)
            }
            Self::Static(t) => {
                let wrapper: DisplayWrapper<&Type> = DisplayWrapper::new(text, t.as_ref());
                write!(f, "static<{}>", wrapper)
            }
            Self::Arr { elem, size } => {
                let wrapper_elem: DisplayWrapper<&Type> = DisplayWrapper::new(text, elem.as_ref());
                let wrapper_size: DisplayWrapper<&Span> = DisplayWrapper::new(text, size);
                write!(f, "arr<{}, {}>", wrapper_elem, wrapper_size)
            }
            Self::Func { args, ret } => {
                write!(f, "(")?;
                for (i, ntp) in args.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    let wrapper: DisplayWrapper<&NameTypePair> = DisplayWrapper::new(text, ntp);
                    write!(f, "{}", wrapper)?;
                }
                let wrapper: DisplayWrapper<&Type> = DisplayWrapper::new(text, ret.as_ref());
                write!(f, ") -> {}", wrapper)
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
        let wrapper_name = DisplayWrapper::new(text, &self.name);
        let wrapper_type = DisplayWrapper::new(text, self.type_.as_ref());
        write!(f, "{}: {}", wrapper_name, wrapper_type)
    }
}

#[derive(Debug)]
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
