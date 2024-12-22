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
                TokenType::String(s) =>
                    return write!(f, "String \"{s}\" {}", TokenType::unescape(s)),
            }
        )
    }
}

impl TokenType<'_> {
    pub fn unescape<'de>(s: &'de str) -> Cow<'de, str> {
        todo!()
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
        let mut chars = self.rest.chars();
        let c = chars.next()?;
        self.rest = chars.as_str();
        self.offset += c.len_utf8();

        enum Started {
            String,
            Number,
            Ident,
        }

        let start =
            match c {
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
                '"' => Started::String,
                '0'..='9' => Started::Number,
                'a'..='z' | '_' => Started::Ident,

                c => return Some(Err(miette::miette! {
                    labels = vec![
                        LabeledSpan::at(self.offset - c.len_utf8()..self.offset, "this character")
                    ],
                    "Unexpected token {c} in input",
                }
                .with_source_code(self.whole.to_string()))),
            };

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
                return Some(Ok(TokenType::Ident(ident)));
            }
            _ => todo!(),
        }
        todo!()
    }
}
