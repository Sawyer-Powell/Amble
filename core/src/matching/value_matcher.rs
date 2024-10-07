use crate::parsing::{Token, TokenType};
use std::{char, collections::HashMap};

pub fn get_position_of_substring(source: &str, substring: &str) -> Option<(usize, usize)> {
    let source_chars: Vec<char> = source.chars().collect();
    let substring_chars: Vec<char> = substring.chars().collect();

    let start = source_chars
        .windows(substring_chars.len())
        .position(|window| window == substring_chars.as_slice());

    match start {
        Some(start) => Some((start, start + substring_chars.len())),
        None => None,
    }
}

#[derive(Debug)]
enum ValueComponent {
    Literal(String),
    Capture(String),
}

fn parse_literal(source: &str, tokens: &Vec<Token>, index: usize) -> (String, usize) {
    let mut index = index;

    let lexeme_start = tokens[index].lexeme_start;
    let mut lexeme_end = lexeme_start;

    while index < tokens.len() {
        let token = &tokens[index];
        index += 1;

        match token.tok_type {
            TokenType::DoubleQuote => {
                break;
            }
            _ => lexeme_end = token.lexeme_end,
        }
    }

    return (source[lexeme_start..lexeme_end].to_string(), index);
}

fn parse_capture(source: &str, tokens: &Vec<Token>, index: usize) -> (String, usize) {
    let mut index = index;

    let lexeme_start = tokens[index].lexeme_start;
    let mut lexeme_end = lexeme_start;

    while index < tokens.len() {
        let token = &tokens[index];

        match token.tok_type {
            TokenType::Text => lexeme_end = token.lexeme_end,
            _ => {
                break;
            }
        }

        index += 1;
    }

    return (source[lexeme_start..lexeme_end].to_string(), index);
}

#[derive(Debug)]
pub struct ValueMatcher {
    components: Vec<ValueComponent>,
}

impl ValueMatcher {
    pub fn parse(source: &str, tokens: &Vec<Token>) -> ValueMatcher {
        let mut index = 0;

        let mut components: Vec<ValueComponent> = Vec::new();

        while index < tokens.len() {
            let token = &tokens[index];

            match token.tok_type {
                TokenType::DoubleQuote => {
                    let (val, idx) = parse_literal(source, tokens, index + 1);
                    index = idx;
                    components.push(ValueComponent::Literal(val));
                }
                TokenType::Text => {
                    let (val, idx) = parse_capture(source, tokens, index);
                    index = idx;
                    components.push(ValueComponent::Capture(val))
                }
                _ => (),
            }

            index += 1;
        }

        ValueMatcher { components }
    }

    pub fn generate(&self, captures: &HashMap<String, String>) -> String {
        let mut out_string = "".to_string();

        for component in self.components.as_slice() {
            match component {
                ValueComponent::Literal(literal) => {
                    out_string.push_str(&format!("{} ", literal));
                },
                ValueComponent::Capture(capture) => {
                    let capture_val = captures.get(capture);

                    if let Some(val) = capture_val {
                        out_string.push_str(&format!("{} ", val));
                    }
                }
            }
        }

        out_string
    }

    pub fn capture(&self, input: &str) -> Option<HashMap<String, String>> {
        let mut captures: HashMap<String, String> = HashMap::new();

        let mut last_slice: (usize, usize) = (0, 0);
        let mut capture_pending: Option<&str> = None;
        for component in &self.components {
            match component {
                ValueComponent::Literal(literal) => {
                    let slice = get_position_of_substring(input, literal)?;

                    // Ensure ordering
                    if slice.0 < last_slice.1 {
                        return None;
                    }

                    if let Some(capture) = capture_pending {
                        if slice.0 > last_slice.1 {
                            captures.insert(
                                capture.to_string(),
                                input[last_slice.1..slice.0].trim().to_string(),
                            );
                        }

                        capture_pending = None;
                    }

                    last_slice = slice;
                }
                ValueComponent::Capture(capture) => {
                    capture_pending = Some(capture);
                }
            }
        }

        // Ensure we resolve any dangling captures
        if let Some(capture) = capture_pending {
            captures.insert(
                capture.to_string(),
                input[last_slice.1..input.len()].trim().to_string(),
            );
        }

        Some(captures)
    }
}
