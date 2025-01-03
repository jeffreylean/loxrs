use core::fmt;
use std::borrow::Cow;

use miette::{Error, LabeledSpan};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType<'de> {
    // Single char token
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,

    // One or two character token
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    String(&'de str),

    // Identifier
    Ident(&'de str),
    Number(&'de str, f64),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl fmt::Display for TokenType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenType::Plus => "PLUS + null",
                TokenType::Minus => "MINUS - null",
                TokenType::Comma => "COMMA , null",
                TokenType::Semicolon => "SEMICOLON ; null",
                TokenType::Lparen => "LEFT_PAREN ( null",
                TokenType::Rparen => "RIGHT_PAREN ) null",
                TokenType::Lbrace => "LEFT_BRACE { null",
                TokenType::Rbrace => "RIGHT_BRACE } null",
                TokenType::Star => "STAR * null",
                TokenType::Dot => "DOT . null",
                TokenType::String(s) => return write!(f, "String {s} {}", TokenType::unescape(s)),
                TokenType::Ident(s) => return write!(f, "IDENTIFIER {} null", s),
                TokenType::Number(s, dec) => todo!(),
                TokenType::Bang => "BANG ! null",
                TokenType::BangEqual => "BANG_EQUAL != null",
                TokenType::Equal => "EQUAL = null",
                TokenType::EqualEqual => "EQUAL_EQUAL == null",
                TokenType::Greater => "GREATER > null",
                TokenType::GreaterEqual => "GREATER_EQUAL >= null",
                TokenType::Less => "LESS < null",
                TokenType::LessEqual => "LESS_EQUAL <= null",
                TokenType::Class => todo!(),
                TokenType::And => todo!(),
                TokenType::Else => todo!(),
                TokenType::False => todo!(),
                TokenType::For => todo!(),
                TokenType::Fun => todo!(),
                TokenType::If => todo!(),
                TokenType::Nil => todo!(),
                TokenType::Or => todo!(),
                TokenType::Print => todo!(),
                TokenType::Return => todo!(),
                TokenType::Super => todo!(),
                TokenType::This => todo!(),
                TokenType::True => todo!(),
                TokenType::Var => todo!(),
                TokenType::While => todo!(),
            }
        )
    }
}

impl TokenType<'_> {
    pub fn unescape<'de>(s: &'de str) -> Cow<'de, str> {
        // trim starting and ending "
        let s = &s[1..s.len() - 1];
        if !s.contains('\\') {
            return Cow::Borrowed(s);
        }

        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.next() {
                    Some('n') => result.push('\n'),
                    Some('r') => result.push('\r'),
                    Some('t') => result.push('\t'),
                    Some('\\') => result.push('\\'),
                    Some('"') => result.push('"'),
                    Some(ch) => {
                        result.push('\\');
                        result.push(ch);
                    }
                    None => result.push('\\'),
                }
            } else {
                result.push(ch);
            }
        }
        Cow::Owned(result)
    }
}

pub struct Lexer<'de> {
    rest: &'de str,
    offset: usize,
    whole: &'de str,
}

impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            rest: input,
            whole: input,
            offset: 0,
        }
    }
}

impl<'de> Iterator for Lexer<'de> {
    type Item = anyhow::Result<TokenType<'de>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        enum Started {
            String,
            Number,
            Ident,
            Bang,
            Equal,
            Greater,
            Less,
        }

        loop {
            let mut chars = self.rest.chars();
            let c = chars.next()?;
            let current_onwards = self.rest;
            self.rest = chars.as_str();
            self.offset += c.len_utf8();

            let start = match c {
                '(' => return Some(Ok(TokenType::Lparen)),
                ')' => return Some(Ok(TokenType::Rparen)),
                '{' => return Some(Ok(TokenType::Lbrace)),
                '}' => return Some(Ok(TokenType::Rbrace)),
                ',' => return Some(Ok(TokenType::Comma)),
                '.' => return Some(Ok(TokenType::Dot)),
                '-' => return Some(Ok(TokenType::Minus)),
                '+' => return Some(Ok(TokenType::Plus)),
                ';' => return Some(Ok(TokenType::Semicolon)),
                '*' => return Some(Ok(TokenType::Star)),
                '!' => Started::Bang,
                '=' => Started::Equal,
                '>' => Started::Greater,
                '<' => Started::Less,
                '"' => Started::String,
                '0'..='9' => Started::Number,
                'a'..='z' | '_' => Started::Ident,
                c if c.is_whitespace() => continue,

                c => return Some(Err(miette::miette! {
                    labels = vec![
                        LabeledSpan::at(self.offset - c.len_utf8()..self.offset, "this character")
                    ],
                    "Unexpected token {c} in input",
                }
                .with_source_code(self.whole.to_string()))),
            };

            // multi character handling
            match start {
                Started::Ident => {
                    let mut chars = self.rest.chars().peekable();
                    let mut len = 0;

                    while let Some(&ch) = chars.peek() {
                        match ch {
                            'a'..='z' | '1'..='9' | '_' => {
                                len += ch.len_utf8();
                                chars.next();
                            }
                            _ => break,
                        }
                    }

                    let ident = &self.whole[self.offset - c.len_utf8()..len + self.offset];
                    self.offset += len;
                    self.rest = &self.rest[len..];
                    return Some(Ok(TokenType::Ident(ident)));
                }
                Started::String => {
                    // Find closing string quote
                    if !self.rest.contains('"') {
                        return Some(Err(miette::miette! {
                            labels = vec![
                                LabeledSpan::at(self.offset - c.len_utf8()..self.offset, "this opening double quote")
                        ],
                            "Missing closing double quote"
                        }));
                    }
                    if let Some(idx) = self.rest.rfind('"') {
                        let string = &current_onwards[..idx + 2];
                        self.rest = &self.rest[idx + 1..];
                        self.offset += idx + 1;
                        return Some(Ok(TokenType::String(string)));
                    }
                    return None;
                }
                Started::Bang => {
                    if let Some(ch) = self.rest.chars().next() {
                        match ch {
                            '=' => {
                                self.rest = &self.rest[1..];
                                self.offset += ch.len_utf8();
                                return Some(Ok(TokenType::BangEqual));
                            }
                            _ => return Some(Ok(TokenType::Bang)),
                        }
                    }
                    return Some(Err(miette::miette! {
                    labels = vec![
                        LabeledSpan::at(self.offset - c.len_utf8()..self.offset, "this character")
                    ],
                    "Unexpected token {c} in input",
                    }
                    .with_source_code(self.whole.to_string())));
                }
                Started::Equal => {
                    if let Some(ch) = self.rest.chars().next() {
                        match ch {
                            '=' => {
                                self.rest = &self.rest[1..];
                                self.offset += ch.len_utf8();
                                return Some(Ok(TokenType::EqualEqual));
                            }
                            _ => return Some(Ok(TokenType::Equal)),
                        }
                    }
                    return Some(Err(miette::miette! {
                    labels = vec![
                        LabeledSpan::at(self.offset - c.len_utf8()..self.offset, "this character")
                    ],
                    "Unexpected token {c} in input",
                    }
                    .with_source_code(self.whole.to_string())));
                }
                Started::Greater => {
                    if let Some(ch) = self.rest.chars().next() {
                        match ch {
                            '=' => {
                                self.rest = &self.rest[1..];
                                self.offset += ch.len_utf8();
                                return Some(Ok(TokenType::GreaterEqual));
                            }
                            _ => return Some(Ok(TokenType::Greater)),
                        }
                    }
                    return Some(Err(miette::miette! {
                    labels = vec![
                        LabeledSpan::at(self.offset - c.len_utf8()..self.offset, "this character")
                    ],
                    "Unexpected token {c} in input",
                    }
                    .with_source_code(self.whole.to_string())));
                }
                Started::Less => {
                    if let Some(ch) = self.rest.chars().next() {
                        match ch {
                            '=' => {
                                self.rest = &self.rest[1..];
                                self.offset += ch.len_utf8();
                                return Some(Ok(TokenType::LessEqual));
                            }
                            _ => return Some(Ok(TokenType::Less)),
                        }
                    }
                    return Some(Err(miette::miette! {
                    labels = vec![
                        LabeledSpan::at(self.offset - c.len_utf8()..self.offset, "this character")
                    ],
                    "Unexpected token {c} in input",
                    }
                    .with_source_code(self.whole.to_string())));
                }
                _ => todo!(),
            }
        }
    }
}
