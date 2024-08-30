use super::lexer::{Token, TokenType, Tokenizer};
use crate::air::{Block, CategoryBlock, RichTextBlock, TextBlock};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(document: &str) -> Parser {
        Parser {
            tokens: Tokenizer::new(document).collect(),
        }
    }

    fn parse_category_block(&mut self, token_index: usize) -> Option<(CategoryBlock, usize)> {
        let mut headline_level: usize = 0;
        let mut content: String = "".to_string();

        if !matches!(self.tokens[token_index].tok_type, TokenType::Asterisk) {
            return None;
        }

        let mut index = token_index + 1;

        // Determine the level of the headline
        while index < self.tokens.len() {
            let token = &self.tokens[index];
            index += 1;
            match token.tok_type {
                TokenType::Asterisk => {
                    headline_level += 1;
                }
                TokenType::Space => break,
                _ => return None,
            }
        }

        // Get headline content
        while index < self.tokens.len() {
            let token = &self.tokens[index];
            index += 1;
            match token.tok_type {
                TokenType::LineBreak => break,
                _ => content.push_str(&token.lexeme),
            }
        }

        let mut block = CategoryBlock {
            id: None,
            level: headline_level,
            name: content.clone(),
            children: Vec::new(),
        };

        // Populate headline children
        while let Some((new_block, new_index)) = self.parse_next_block(index) {
            match new_block {
                Block::Category(category_block) if category_block.level <= block.level => {
                    break;
                }
                _ => {
                    block.children.push(new_block);
                    index = new_index;
                }
            }
        }

        Some((block, index))
    }

    fn parse_rich_text_block(&mut self, token_index: usize) -> Option<(RichTextBlock, usize)> {
        let mut block = RichTextBlock {
            children: Vec::new(),
        };

        let mut content: String = "".to_string();

        let mut index = token_index;
        while index < self.tokens.len() {
            let token = &self.tokens[index];
            index += 1;
            match token.tok_type {
                TokenType::LineBreak => break,
                _ => content.push_str(&token.lexeme),
            }
        }

        let new_block = Block::Text(TextBlock { content });

        block.children.push(new_block);

        Some((block, index))
    }

    fn parse_next_block(&mut self, token_index: usize) -> Option<(Block, usize)> {
        if token_index >= self.tokens.len() {
            return None;
        }

        if let Some((category, new_index)) = self.parse_category_block(token_index) {
            Some((Block::Category(category), new_index))
        } else if let Some((rich_text, new_index)) = self.parse_rich_text_block(token_index) {
            Some((Block::RichText(rich_text), new_index))
        } else {
            None
        }
    }

    pub fn parse(&mut self) -> Vec<Block> {
        let mut index = 0;
        let mut blocks = Vec::new();

        while let Some((block, new_index)) = self.parse_next_block(index) {
            blocks.push(block);
            index = new_index;
        }

        blocks
    }
}
