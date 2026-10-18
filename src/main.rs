use std::fs;

use c2c::{display_wrapper::DisplayWrapper, error::Error, parse::Parser, token::Token, tokenize};

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
        let wrapper: DisplayWrapper<&Token> = DisplayWrapper::new(content.as_str(), token);
        println!("{}", wrapper);
    }

    let mut parser: Parser = Parser::new(tokens);
    match parser.parse(content.as_str()) {
        Ok(()) => (),
        Err(e) => {
            let wrapper: DisplayWrapper<&Error> = DisplayWrapper::new(content.as_str(), &e);
            println!("{}", wrapper);
        }
    }
}
