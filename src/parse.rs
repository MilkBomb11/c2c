use crate::{
    ast::{NTPOrType, NameTypePair, Span, Type},
    error::ParseError,
    repo::Repo,
    token::{Token, TokenType},
    translate::{translate_and_clean_ntp, translate_and_clean_type},
};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) -> &Token {
        let old_pos = self.pos;

        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }

        &self.tokens[old_pos]
    }

    fn peek_prev(&self) -> &Token {
        &self.tokens[self.pos - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().type_ == TokenType::Eof
    }

    fn do_match(&mut self, t: &[TokenType]) -> bool {
        for t in t.iter() {
            if &self.peek().type_ == t {
                self.advance();
                return true;
            }
        }
        false
    }

    fn eat(&mut self, t: &[TokenType]) -> Result<(), ParseError> {
        for t in t.iter() {
            if &self.peek().type_ == t {
                self.advance();
                return Ok(());
            }
        }
        let loc = self.peek().start_index;
        Err(ParseError {
            loc,
            msg: format!("Expected {:?} but got {:?}.", t, self.peek().type_),
        })
    }

    fn backtrack(&mut self) {
        self.pos -= 1;
    }

    fn parse_type(&mut self, text: &str) -> Result<Box<Type>, ParseError> {
        match self.advance().type_ {
            TokenType::Identifier => {
                let start = self.peek_prev().start_index;
                let end = self.peek_prev().end_index;
                match &text[start..end] {
                    "ptr" => {
                        self.eat(&[TokenType::OneChar('<')])?;
                        let inner = self.parse_type(text)?;
                        self.eat(&[TokenType::OneChar('>')])?;
                        Ok(Box::new(Type::Ptr(inner)))
                    }
                    "const" => {
                        self.eat(&[TokenType::OneChar('<')])?;
                        let inner = self.parse_type(text)?;
                        self.eat(&[TokenType::OneChar('>')])?;
                        Ok(Box::new(Type::Const(inner)))
                    }
                    "static" => {
                        self.eat(&[TokenType::OneChar('<')])?;
                        let inner = self.parse_type(text)?;
                        self.eat(&[TokenType::OneChar('>')])?;
                        Ok(Box::new(Type::Static(inner)))
                    }
                    "struct" => {
                        self.eat(&[TokenType::Identifier])?;
                        let start = self.peek_prev().start_index;
                        let end = self.peek_prev().end_index;
                        let name = Span::new(start, end);
                        Ok(Box::new(Type::Struct(name)))
                    }
                    "arr" => {
                        self.eat(&[TokenType::OneChar('<')])?;
                        let elem = self.parse_type(text)?;
                        self.eat(&[TokenType::OneChar(',')])?;
                        self.eat(&[TokenType::Number, TokenType::Identifier])?;
                        let start = self.peek_prev().start_index;
                        let end = self.peek_prev().end_index;
                        let size = Span::new(start, end);
                        self.eat(&[TokenType::OneChar('>')])?;
                        Ok(Box::new(Type::Arr { elem, size }))
                    }
                    "fn" => {
                        let args: Vec<NTPOrType> = self.parse_args(text)?;
                        self.eat(&[TokenType::Arrow])?;
                        let ret: Box<Type> = self.parse_type(text)?;
                        Ok(Box::new(Type::Func { args, ret }))
                    }
                    _ => {
                        let name: Span = Span::new(start, end);
                        Ok(Box::new(Type::Atom(name)))
                    }
                }
            }
            _ => Err(ParseError {
                loc: self.peek_prev().start_index,
                msg: format!("{:?} is not a valid type marker.", self.peek_prev().type_),
            }),
        }
    }

    fn parse_ntp(&mut self, text: &str) -> Result<NameTypePair, ParseError> {
        self.eat(&[TokenType::Identifier])?;
        let start = self.peek_prev().start_index;
        let end = self.peek_prev().end_index;
        let name = Span::new(start, end);
        self.eat(&[TokenType::OneChar(':')])?;
        let type_ = self.parse_type(text)?;
        return Ok(NameTypePair { name, type_ });
    }

    fn parse_args(&mut self, text: &str) -> Result<Vec<NTPOrType>, ParseError> {
        self.eat(&[TokenType::OneChar('(')])?;
        let mut args: Vec<NTPOrType> = Vec::new();
        if self.do_match(&[TokenType::OneChar(')')]) {
            return Ok(args);
        }
        loop {
            let ntp_or_type = self.parse_ntp_or_type(text)?;
            args.push(ntp_or_type);
            if self.do_match(&[TokenType::OneChar(')')]) {
                break;
            }
            self.eat(&[TokenType::OneChar(',')])?;
        }
        Ok(args)
    }

    pub fn parse_ntp_or_type(&mut self, text: &str) -> Result<NTPOrType, ParseError> {
        self.eat(&[TokenType::Identifier])?;
        let ntp_or_type = if self.peek().type_ == TokenType::OneChar(':') {
            self.backtrack();
            let ntp = self.parse_ntp(text)?;
            NTPOrType::NTP(ntp)
        } else {
            self.backtrack();
            let typ = self.parse_type(text)?;
            NTPOrType::Type(typ)
        };
        Ok(ntp_or_type)
    }

    pub fn parse(&mut self, text: &str) -> Result<(Repo<Span>, Repo<String>), ParseError> {
        let mut replace_dest_repo: Repo<Span> = Repo::new();
        let mut replace_src_repo: Repo<String> = Repo::new();
        while !self.is_at_end() {
            self.advance();
            if self.peek_prev().type_ == TokenType::PairOpen {
                let start_index = self.peek_prev().start_index;
                let ntp_or_type = self.parse_ntp_or_type(text)?;
                let s = match &ntp_or_type {
                    NTPOrType::NTP(ntp) => {
                        let name = &text[ntp.name.start..ntp.name.end];
                        translate_and_clean_ntp(name, ntp.type_.as_ref(), text)
                    }
                    NTPOrType::Type(typ) => translate_and_clean_type(typ, text),
                };
                replace_src_repo.push(s);
                self.eat(&[TokenType::PairClose])?;
                let end_index = self.peek_prev().end_index;
                let span = Span::new(start_index, end_index);
                replace_dest_repo.push(span);
            }
        }
        Ok((replace_dest_repo, replace_src_repo))
    }
}
