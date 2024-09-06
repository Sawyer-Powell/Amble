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
    pub lexeme_start: usize,
    pub lexeme_end: usize
}

pub struct Tokenizer {
    chars: Vec<char>,
    num_chars: usize,
    char_index: usize,
}

impl Tokenizer {
    pub fn new(document: &str) -> Tokenizer {
        Tokenizer {
            chars: document.chars().collect(),
            num_chars: document.chars().count(),
            char_index: 0,
        }
    }

    fn peek_next_char(&mut self, index: usize) -> Option<char> {
        if index >= self.num_chars {
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
            None => (TokenType::EOF, self.num_chars),
            _ => {
                let (tok_type, i) = self.peek_next_token(index + 1);
                match tok_type {
                    TokenType::Text => (tok_type, i),
                    _ => (TokenType::Text, index + 1),
                }
            }
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let (tok_type, index) = self.peek_next_token(self.char_index);
            let token = Token {
                tok_type,
                lexeme_start: self.char_index,
                lexeme_end: index,
            };
            self.char_index = index;
            match token.tok_type {
                TokenType::EOF => break,
                _ => tokens.push(token)
            }
        }

        tokens
    }
}
