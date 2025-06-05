// src/scanner.rs

use crate::token::Token;

pub struct Scanner {
    source: Vec<char>, // input code split into characters
    current: usize,    // position in the source
}

// the scanner definition and methods 

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.is_at_end() {
            return Token::EOF;
        }

        let c = self.advance();

        match c {
            '=' => Token::Equals,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '\n' => Token::Newline,
            ch if ch.is_ascii_digit() => self.number(ch),
            ch if ch.is_ascii_alphabetic() => self.identifier(ch),
            _ => Token::Unknown(c),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.source.get(self.current) {
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.current += 1;
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn number(&mut self, first: char) -> Token {
        let mut value = first.to_string();
        while let Some(&ch) = self.source.get(self.current) {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.current += 1;
            } else {
                break;
            }
        }
        Token::Number(value)
    }

    fn identifier(&mut self, first: char) -> Token {
        let mut ident = first.to_string();
        while let Some(&ch) = self.source.get(self.current) {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.current += 1;
            } else {
                break;
            }
        }

        // Check for reserved words like "let"
        match ident.as_str() {
            "let" => Token::Let,
            _ => Token::Identifier(ident),
        }
    }
}











