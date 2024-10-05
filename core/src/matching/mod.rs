pub mod fold;
mod value_matcher;

#[cfg(test)]
mod tests {
    use crate::{Block, CategoryBlock, Parser};

    use crate::{
        matching::value_matcher::{get_position_of_substring, ValueMatcher},
        parsing::Tokenizer,
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

    fn get_flat_matchers<'a>(matcher: &'a CategoryMatcher) -> Vec<&'a CategoryMatcher> {
        let mut children: Vec<&'a CategoryMatcher> = Vec::new();

        for child in matcher.body.as_slice() {
            children.extend(get_flat_matchers(child));
        }

        children.push(&matcher);
        children
    }

    fn annotate<'a>(category: &mut CategoryBlock<'a>, matchers: &Vec<&'a CategoryMatcher>) {
        for child in category.children.as_mut_slice() {
            match child {
                Block::Category(child) => {
                    annotate(child, matchers);
                }
                _ => (),
            }
        }


        for matcher in matchers {
            if let Some(m) = matcher.do_match(category) {
                dbg!(m);
                category.matches.push(*matcher);
            }
        }
    }

    #[test]
    fn parse_fold() {
        let test = r#"
* TODO Do this other thing
** DUE next week 
** EFFORT 23pts
* FOLD
** FROM
*** "TODO" title
**** "DUE" due-date
**** "EFFORT" effort
** INTO
*** "UHOH" title"#;

        let parser = Parser::new(test);
        let mut blocks = parser.parse();

        let mut folds = Vec::new();

        for block in blocks.as_slice() {
            match block {
                Block::Category(category) => {
                    if let Some(fold) = Fold::parse(&category) {
                        folds.push(fold);
                    }
                }
                _ => (),
            }
        }

        let mut matchers: Vec<&CategoryMatcher> = Vec::new();

        for fold in folds.as_slice() {
            for matcher in fold.from.matchers.as_slice() {
                matchers.extend(get_flat_matchers(matcher));
            }
        }

        for block in blocks.as_mut_slice() {
            match block {
                Block::Category(category) => {
                    annotate(category, &matchers);
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
