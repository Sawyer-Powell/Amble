mod value_matcher;
mod fold;

#[cfg(test)]
mod tests {
    use crate::{Block, Parser};

    use crate::{
        matching::value_matcher::{get_position_of_substring, ValueMatcher}, parsing::Tokenizer
    };

    use super::fold::*;

    //cargo watch -x 'test test_value_matcher -- --nocapture'
    #[test]
    fn test_value_matcher() {
        let name = "\"TODO\" title \"BUT\" condition";

        let test = "TODO I want to to take the dog outside BUT i need to do it before friday";

        let mut tokenizer = Tokenizer::new(&name);
        let tokens = tokenizer.get_tokens();

        let matcher = ValueMatcher::parse(name, &tokens);

        dbg!(matcher.capture(test));
    }

        #[test]
    fn parse_fold() {
        let test = r#"* TODO My life is amazing
* TODO Do this other thing
* FOLD
** FROM
*** "TODO" title
**** "LAST" last-title
** INTO
*** "UHOH" title"#;

        let parser = Parser::new(test);
        let blocks = parser.parse();

        for block in blocks {
            match block {
                Block::Category(category) => {
                    if let Some(fold) = Fold::parse(&category) {
                        dbg!(fold);
                    }
                }
                _ => (),
            }
        }
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
