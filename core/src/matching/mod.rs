use std::{char, collections::HashMap};
use crate::parsing::{Token, TokenType};

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
    fn parse(source: &str, tokens: &Vec<Token>) -> ValueMatcher {
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

    fn capture(&self, input: &str) -> Option<HashMap<String, String>> {
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

#[cfg(test)]
mod tests {
    use crate::{matching::{get_position_of_substring, ValueMatcher}, parsing::Tokenizer};

    //cargo watch -x 'test test_value_matcher -- --nocapture'
    #[test]
    fn test_value_matcher() {
        let name = "\"TODO\" title \"BUT\" condition";

        let test =
            "TODO I want to to take the dog outside BUT i need to do it before friday";

        let mut tokenizer = Tokenizer::new(&name);
        let tokens = tokenizer.get_tokens();

        let matcher = ValueMatcher::parse(name, &tokens);

        dbg!(matcher.capture(test));
    }


    #[test]
    fn find_position_of_substring() {
        let tests = vec![
            (
                "Hello this is a sentence that contains a special word that we have to extract",
                "contains",
                Some((30, 38)),
            ),
            (
                "This string does not have the substring in it",
                "contains",
                None,
            ),
            (
                "contains is at the beginning here",
                "contains",
                Some((0, 8)),
            ),
            ("At the end there lies contains", "contains", Some((22, 30))),
        ];

        for test in tests {
            assert!(get_position_of_substring(test.0, test.1) == test.2);
        }
    }
}
