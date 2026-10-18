use crate::ast::{Span, Type};
/***
x: int -> int x

***/

fn span_as_str<'a>(span: &Span, text: &'a str) -> &'a str {
    let start = span.start;
    let end = span.end;
    &text[start..end]
}

fn remove_injections(s: String) -> String {
    s.replace("^", "").replace("$", "")
}

fn add_star(s: String) -> String {
    s.replacen("^", "*^", 1)
}

fn add_brackets(s: String, size: &str) -> String {
    s.replacen("$", &format!("$[{}]", size), 1)
}

fn add_star_paren(s: String) -> String {
    s.replacen("^", "(*^", 1).replacen("$", ")$", 1)
}

fn translate_ntp(name: &str, type_: &Type, text: &str) -> String {
    match type_ {
        Type::Atom(typ) => {
            let typ = span_as_str(typ, text);
            format!("{} ^{}$", typ, name)
        }
        Type::Struct(inner) => {
            let inner = span_as_str(inner, text);
            format!("struct {} ^{}$", inner, name)
        }
        Type::Const(inner) => {
            let s = translate_ntp(name, inner.as_ref(), text);
            format!("const {}", s)
        }
        Type::Static(inner) => {
            let s = translate_ntp(name, inner.as_ref(), text);
            format!("static {}", s)
        }
        Type::Ptr(inner) => {
            let s = translate_ntp(name, inner.as_ref(), text);
            match inner.as_ref() {
                Type::Arr { .. } | Type::Func { .. } => add_star_paren(s),
                _ => add_star(s),
            }
        }
        Type::Arr { elem, size } => {
            let s = translate_ntp(name, elem.as_ref(), text);
            let size = span_as_str(size, text);
            add_brackets(s, size)
        }
        Type::Func { args, ret } => {
            let args: Vec<String> = args
                .iter()
                .map(|ntp| {
                    let name = span_as_str(&ntp.name, text);
                    let type_ = ntp.type_.as_ref();
                    remove_injections(translate_ntp(name, type_, text))
                })
                .collect();
            let args = format!("({})", args.join(", "));
            format!("{}{}", translate_ntp(name, ret.as_ref(), text), args)
        }
    }
}

pub fn translate_and_clean_ntp(name: &str, type_: &Type, text: &str) -> String {
    remove_injections(translate_ntp(name, type_, text))
}
