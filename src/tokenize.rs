use crate::token::Token;
use crate::token::TokenType;
use std::{iter::Peekable, str::CharIndices};

pub fn get_tokens(text: &str) -> Vec<Token> {
    let mut it: Peekable<CharIndices> = text.char_indices().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(token) = get_token(&mut it) {
        tokens.push(token);
    }
    tokens.push(Token {
        start_index: 0,
        end_index: 0,
        type_: TokenType::Eof,
    });
    return tokens;
}

fn get_token(it: &mut Peekable<CharIndices>) -> Option<Token> {
    match it.next() {
        None => None,
        Some((byte_index, c)) => match c {
            '[' => match it.next_if(|(_, c)| *c == '<') {
                Some(_) => Some(Token {
                    type_: TokenType::PairOpen,
                    start_index: byte_index,
                    end_index: byte_index + 2,
                }),
                None => Some(Token {
                    type_: TokenType::OneChar('['),
                    start_index: byte_index,
                    end_index: byte_index + 1,
                }),
            },
            '>' => match it.next_if(|(_, c)| *c == ']') {
                Some(_) => Some(Token {
                    type_: TokenType::PairClose,
                    start_index: byte_index,
                    end_index: byte_index + 2,
                }),
                None => Some(Token {
                    type_: TokenType::OneChar('>'),
                    start_index: byte_index,
                    end_index: byte_index + 1,
                }),
            },
            '-' => match it.next_if(|(_, c)| *c == '>') {
                Some(_) => Some(Token {
                    type_: TokenType::Arrow,
                    start_index: byte_index,
                    end_index: byte_index + 2,
                }),
                None => Some(Token {
                    type_: TokenType::OneChar('-'),
                    start_index: byte_index,
                    end_index: byte_index + 1,
                }),
            },
            '/' => match it.next_if(|(_, c)| *c == '/' || *c == '*') {
                Some((_, '/')) => handle_single_comment(it, byte_index),
                Some((_, '*')) => handle_multi_comment(it, byte_index),
                Some(_) | None => Some(Token {
                    type_: TokenType::OneChar('/'),
                    start_index: byte_index,
                    end_index: byte_index + 1,
                }),
            },
            '\'' => handle_single_quote(it, byte_index),
            ' ' | '\t' | '\n' | '\r' => handle_whitespace(it),
            '"' => handle_double_quote(it, byte_index),
            _ => {
                if c.is_ascii_digit() {
                    handle_number(it, byte_index)
                } else if is_identifier_starter(c) {
                    handle_identifier(it, byte_index)
                } else {
                    Some(Token {
                        type_: TokenType::OneChar(c),
                        start_index: byte_index,
                        end_index: byte_index + 1,
                    })
                }
            }
        },
    }
}

fn handle_identifier<'a>(it: &mut Peekable<CharIndices>, start_index: usize) -> Option<Token> {
    let mut end_index: usize = start_index;
    while let Some((i, _)) = it.next_if(|(_, c)| is_identifier_cont(*c)) {
        end_index = i;
    }
    Some(Token {
        type_: TokenType::Identifier,
        start_index,
        end_index: end_index + 1,
    })
}

fn handle_number<'a>(it: &mut Peekable<CharIndices>, start_index: usize) -> Option<Token> {
    let mut end_index: usize = start_index;
    while let Some((i, _)) = it.next_if(|(_, c)| c.is_ascii_digit()) {
        end_index = i;
    }
    Some(Token {
        type_: TokenType::Number,
        start_index,
        end_index: end_index + 1,
    })
}

fn handle_single_comment<'a>(it: &mut Peekable<CharIndices>, start_index: usize) -> Option<Token> {
    let mut end_index: usize = start_index;
    while let Some((i, c)) = it.next() {
        end_index = i;
        if c == '\n' {
            break;
        }
    }
    if let None = it.peek() {
        return Some(Token {
            type_: TokenType::Comment,
            start_index,
            end_index: end_index + 1,
        });
    }
    Some(Token {
        type_: TokenType::Comment,
        start_index,
        end_index: end_index + 1,
    })
}

fn handle_multi_comment<'a>(it: &mut Peekable<CharIndices>, start_index: usize) -> Option<Token> {
    let mut end_index: usize = start_index;
    while let Some((i, c)) = it.next() {
        end_index = i;
        if c == '*'
            && let Some((_, '/')) = it.peek()
        {
            it.next();
            break;
        }
    }
    if let None = it.peek() {
        return Some(Token {
            type_: TokenType::Comment,
            start_index,
            end_index: end_index + 1,
        });
    }
    Some(Token {
        type_: TokenType::Comment,
        start_index,
        end_index: end_index + 2,
    })
}

fn handle_single_quote<'a>(it: &mut Peekable<CharIndices>, start_index: usize) -> Option<Token> {
    let mut end_index: usize = start_index;
    while let Some((i, c)) = it.next() {
        end_index = i;
        if c == '\'' {
            break;
        }
    }
    if let None = it.peek() {
        return Some(Token {
            type_: TokenType::Quote,
            start_index,
            end_index: end_index + 1,
        });
    }
    Some(Token {
        type_: TokenType::Quote,
        start_index,
        end_index: end_index + 1,
    })
}

fn handle_double_quote<'a>(it: &mut Peekable<CharIndices>, start_index: usize) -> Option<Token> {
    let mut end_index: usize = 0;
    while let Some((i, c)) = it.next() {
        end_index = i;
        if c == '"' {
            break;
        }
    }
    if let None = it.peek() {
        return Some(Token {
            type_: TokenType::Quote,
            start_index,
            end_index: end_index + 1,
        });
    }
    Some(Token {
        type_: TokenType::Quote,
        start_index,
        end_index: end_index + 1,
    })
}

fn handle_whitespace<'a>(it: &mut Peekable<CharIndices>) -> Option<Token> {
    while let Some(_) = it.next_if(|(_, c)| matches!(c, ' ' | '\t' | '\n' | '\r')) {}
    get_token(it)
}

fn is_identifier_starter(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_cont(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
