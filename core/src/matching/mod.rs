pub mod fold;
mod value_matcher;

use std::collections::HashMap;

use crate::{Block, CategoryBlock, Parser};
use fold::*;

fn get_flat_matchers<'a>(matcher: &'a CategoryMatcher) -> Vec<&'a CategoryMatcher> {
    let mut children: Vec<&'a CategoryMatcher> = Vec::new();

    for child in matcher.body.as_slice() {
        children.extend(get_flat_matchers(child));
    }

    children.push(&matcher);
    children
}

fn annotate<'a>(
    category: &mut CategoryBlock<'a>,
    matchers: &Vec<&'a CategoryMatcher>,
) -> Vec<HashMap<String, String>> {
    // Recursively annotate all the children
    let mut top_level_matches = Vec::new();

    // Ignore folds
    if let Some(_) = Fold::parse(category) {
        return top_level_matches;
    }

    for child in category.children.as_mut_slice() {
        match child {
            Block::Category(child) => {
                top_level_matches.extend(annotate(child, matchers));
            }
            _ => (),
        }
    }

    for (index, matcher) in matchers.iter().enumerate() {
        if let Some(captures) = matcher.do_match(category) {
            category.matches.push(*matcher);
            category.captures.push(captures.clone());

            if index == matchers.len() - 1 {
                top_level_matches.push(captures);
            }
        }
    }

    return top_level_matches;
}

pub fn expand_folds<'a>(
    parser: &'a Parser<'a>,
    folds: &'a mut Vec<Fold>,
    generated_captures: &'a mut Vec<String>,
) -> Vec<Block<'a>> {
    let mut blocks = parser.parse();

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

    let mut into_input = Vec::new();

    for block in blocks.as_mut_slice() {
        match block {
            Block::Category(category) => {
                for captures in annotate(category, &matchers) {
                    into_input.push(captures);
                }
            }
            _ => (),
        }
    }

    for fold in folds.as_slice() {
        for matcher in fold.into.matchers.as_slice() {
            for capture in into_input.as_slice() {
                generated_captures.push(format!(
                    "{}\n",
                    matcher.generate_block_from_captures(capture)
                ));
            }
        }
    }

    for cap in generated_captures.as_slice() {
        blocks.push(Block::Category(CategoryBlock {
            id: None,
            name: cap,
            level: 1,
            children: Vec::new(),
            matches: Vec::new(),
            captures: Vec::new(),
        }));
    }

    blocks
}

#[cfg(test)]
mod tests {
    use crate::{
        db::DbBlockMatrix,
        matching::value_matcher::{get_position_of_substring, ValueMatcher},
        parsing::Tokenizer,
        rendering::render_to_org,
        AmbleDB, Block, CategoryBlock, Parser,
    };

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

    #[test]
    fn expand_folds() {
        let content = r#"* TODO Need to talk to Sarah
* FOLD
** FROM
*** "TODO" title
** INTO
*** title
"#;

        let parser = Parser::new(&content);
        let mut folds = Vec::new();
        let mut captures = Vec::new();

        let blocks = super::expand_folds(&parser, &mut folds, &mut captures);

        let mut db = AmbleDB::new("amble.sqlite").expect("Could not create db");

        let category = CategoryBlock {
            id: Some(1),
            name: "Test",
            children: blocks,
            level: 1,
            matches: Vec::new(),
            captures: Vec::new(),
        };

        let last_id = db
            .write_top_level_category(&category)
            .expect("Should be able to save category to database");

        let cat_id = 1;

        let matrix =
            DbBlockMatrix::new(&db.connection, cat_id).expect("Could not create db block matrix");

        let flat_blocks = matrix
            .produce_flat_db_block_vec(&db.connection)
            .expect("Could not produce flat vec of db blocks");

        let category_block = matrix
            .form_category_block_tree(&flat_blocks)
            .expect("Could not get category block");

        let out_string = render_to_org(Block::Category(category_block));

        println!("{}", out_string);
    }
}
