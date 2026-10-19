use std::fs;

use c2c::{
    display_wrapper::ToDisplayWrapper, parse::Parser, print::Printer, token::Token, tokenize,
};

fn main() {
    let content = match fs::read_to_string("tests/test.c2c") {
        Ok(s) => s,
        Err(_) => {
            println!("Failed to load file.");
            return;
        }
    };

    let tokens: Vec<Token> = tokenize::get_tokens(content.as_str());
    for token in tokens.iter() {
        println!("{}", token.with_text(content.as_str()));
    }

    let mut parser: Parser = Parser::new(tokens);
    let (replace_dests, replace_srcs) = match parser.parse(content.as_str()) {
        Ok(repos) => repos,
        Err(e) => return println!("{}", e.with_text(content.as_str())),
    };
    println!("{}", replace_dests.with_text(content.as_str()));
    println!("{}", replace_srcs);
    let printer: Printer = Printer::new(replace_dests, replace_srcs, content.as_str());
    println!("{}", printer)
}
