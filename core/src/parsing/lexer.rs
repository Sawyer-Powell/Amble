use std::usize;

#[derive(Debug)]
pub enum TokenType {
    Asterisk,     // *
    EqualsSign,   // =
    Hyphen,       // -
    LineBreak,    // \r\n or \n
    Bar,          // |
    RBracket,     // ]
    LBracket,     // [
    Colon,        // :
    HashPlus,     // #+
    Text,         // Normal text that you'd normally write
    NumberPeriod, // '1.', '2.', etc..
    Space,        // ' ' character
    Tab,          // '\t' character
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub tok_type: TokenType,
    pub lexeme: String,
}

pub struct Tokenizer {
    chars: Vec<char>,
    char_index: usize,
}

impl<'a> Tokenizer {
    pub fn new(document: &'a str) -> Tokenizer {
        Tokenizer {
            chars: document.chars().collect(),
            char_index: 0,
        }
    }

    fn peek_next_char(&mut self, index: usize) -> Option<char> {
        if index >= self.chars.len() {
            None
        } else {
            Some(self.chars[index])
        }
    }

    /// Determines the type and terminating index of the upcoming token
    fn peek_next_token(&mut self, index: usize) -> (TokenType, usize) {
        match self.peek_next_char(index) {
            Some('*') => (TokenType::Asterisk, index + 1),
            Some('=') => (TokenType::EqualsSign, index + 1),
            Some('-') => (TokenType::Hyphen, index + 1),
            Some('\n') => (TokenType::LineBreak, index + 1),
            Some('\r') if self.peek_next_char(index + 1) == Some('\n') => {
                (TokenType::LineBreak, index + 2)
            }
            Some('|') => (TokenType::Bar, index + 1),
            Some(' ') => (TokenType::Space, index + 1),
            Some('\t') => (TokenType::Tab, index + 1),
            Some('[') => (TokenType::LBracket, index + 1),
            Some(']') => (TokenType::RBracket, index + 1),
            Some(':') => (TokenType::Colon, index + 1),
            Some('#') if self.peek_next_char(index + 1) == Some('+') => {
                (TokenType::HashPlus, index + 1)
            }
            Some(char) if char.is_numeric() && self.peek_next_char(index + 1) == Some('.') => {
                (TokenType::NumberPeriod, index + 2)
            }
            None => (TokenType::EOF, self.chars.len()),
            _ => {
                let (tok_type, i) = self.peek_next_token(index + 1);
                match tok_type {
                    TokenType::Text => (tok_type, i),
                    _ => (TokenType::Text, index + 1),
                }
            }
        }
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (tok_type, index) = self.peek_next_token(self.char_index);
        let token = Token {
            tok_type,
            lexeme: self.chars[self.char_index..index]
                .iter()
                .collect::<String>(),
        };
        self.char_index = index;
        match token.tok_type {
            TokenType::EOF => None,
            _ => Some(token),
        }
    }
}
