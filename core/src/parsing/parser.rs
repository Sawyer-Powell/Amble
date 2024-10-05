use super::lexer::{Token, TokenType, Tokenizer};
use crate::air::{Block, CategoryBlock, RichTextBlock, TextBlock};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    document: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(document: &str) -> Parser {
        let tokens = Tokenizer::new(document).get_tokens();
        Parser { tokens, document }
    }

    // Determines if the next block is a category, if it is, return the category level
    fn determine_category_level(&self, token_index: usize) -> Option<(usize, usize)> {
        let mut headline_level: usize = 0;

        match self.tokens[token_index].tok_type {
            TokenType::Asterisk => (),
            _ => return None,
        }

        let mut index = token_index;

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

        Some((headline_level, index))
    }

    fn parse_category_block(
        &self,
        token_index: usize,
        level: Option<usize>,
    ) -> Option<(CategoryBlock, usize)> {
        let mut index = token_index;

        let lexeme_start = self.tokens[index].lexeme_start;
        let mut lexeme_end = lexeme_start;

        while index < self.tokens.len() {
            let token = &self.tokens[index];
            index += 1;
            match token.tok_type {
                TokenType::LineBreak => {
                    lexeme_end = token.lexeme_end;
                    break;
                }
                _ => lexeme_end = token.lexeme_end,
            }
        }

        let mut block = CategoryBlock {
            id: None,
            level: level.unwrap_or_default(),
            name: &self.document[lexeme_start..lexeme_end],
            children: Vec::new(),
            matches: Vec::new()
        };

        while let Some((new_block, new_index)) = self.parse_next_block(index, level) {
            block.children.push(new_block);
            index = new_index;
        }

        Some((block, index))
    }

    fn parse_rich_text_block(&self, token_index: usize) -> Option<(RichTextBlock, usize)> {
        let mut block = RichTextBlock {
            children: Vec::new(),
        };

        let mut index = token_index;

        let lexeme_start = self.tokens[index].lexeme_start;
        let mut lexeme_end = lexeme_start;

        while index < self.tokens.len() {
            let token = &self.tokens[index];
            index += 1;
            match token.tok_type {
                TokenType::LineBreak => {
                    lexeme_end = token.lexeme_end;
                    break;
                },
                _ => lexeme_end = token.lexeme_end,
            }
        }

        let new_block = Block::Text(TextBlock {
            content: &self.document[lexeme_start..lexeme_end],
        });

        block.children.push(new_block);

        Some((block, index))
    }

    fn parse_next_block(
        &self,
        token_index: usize,
        category_level: Option<usize>,
    ) -> Option<(Block, usize)> {
        if token_index >= self.tokens.len() {
            return None;
        }

        // Determine if the next block is a category, if so, get its level
        if let Some((level, index)) = self.determine_category_level(token_index) {
            if let Some(current_category_level) = category_level {
                if current_category_level >= level {
                    return None;
                }
            }
            if let Some((category, new_index)) = self.parse_category_block(index, Some(level)) {
                return Some((Block::Category(category), new_index));
            }
        } else {
            if let Some((rich_text, new_index)) = self.parse_rich_text_block(token_index) {
                return Some((Block::RichText(rich_text), new_index));
            }
        }

        None
    }

    pub fn parse(&self) -> Vec<Block> {
        let mut index = 0;
        let mut blocks = Vec::new();

        while let Some((block, new_index)) = self.parse_next_block(index, None) {
            blocks.push(block);
            index = new_index;
        }

        blocks
    }
}
