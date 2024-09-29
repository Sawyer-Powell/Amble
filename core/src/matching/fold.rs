use crate::{air::Block, parsing::Tokenizer, CategoryBlock};
use crate::Parser;

use super::value_matcher::ValueMatcher;

#[derive(Debug)]
struct CategoryMatcher {
    title: ValueMatcher,
    body: Vec<CategoryMatcher>,
}

impl CategoryMatcher {
    fn parse(category: &CategoryBlock) -> CategoryMatcher {
        let source = category.name;
        let mut tokenizer = Tokenizer::new(source);
        let tokens = tokenizer.get_tokens();
        let title_value_matcher = ValueMatcher::parse(source, &tokens);

        let mut child_category_matchers: Vec<CategoryMatcher> = Vec::new();

        for child in category.children.as_slice() {
            match child {
                Block::Category(child_cat) => child_category_matchers.push(Self::parse(child_cat)),
                _ => (),
            }
        }

        CategoryMatcher {
            title: title_value_matcher,
            body: child_category_matchers,
        }
    }
}

#[derive(Debug)]
struct FoldFrom {
    matchers: Vec<CategoryMatcher>,
}

impl FoldFrom {
    fn parse(category: &CategoryBlock) -> Option<Self> {
        let mut matchers: Vec<CategoryMatcher> = Vec::new();

        if category.name.contains("FROM") {
            for child in category.children.as_slice() {
                match child {
                    Block::Category(child_cat) => matchers.push(CategoryMatcher::parse(child_cat)),
                    _ => (),
                }
            }
            return Some(Self { matchers });
        }

        None
    }
}

#[derive(Debug)]
struct FoldInto {
    matchers: Vec<CategoryMatcher>,
}

impl FoldInto {
    fn parse(category: &CategoryBlock) -> Option<Self> {
        let mut matchers: Vec<CategoryMatcher> = Vec::new();

        if category.name.contains("INTO") {
            for child in category.children.as_slice() {
                match child {
                    Block::Category(child_cat) => matchers.push(CategoryMatcher::parse(child_cat)),
                    _ => (),
                }
            }
            return Some(Self { matchers });
        }

        None
    }
}

#[derive(Debug)]
pub struct Fold {
    from: FoldFrom,
    into: FoldInto,
}

impl Fold {
    pub fn parse(category: &CategoryBlock) -> Option<Self> {
        if category.name.contains("FOLD") {
            if category.children.len() < 2 {
                dbg!(category.children.len());
                return None;
            }

            if let Block::Category(child_cat) = &category.children[0] {
                if let Block::Category(child_cat_2) = &category.children[1] {
                    let from_component = FoldFrom::parse(child_cat)?;
                    let into_component = FoldInto::parse(child_cat_2)?;

                    return Some(Self {
                        from: from_component,
                        into: into_component,
                    });
                }
            } else {
                return None;
            }
        }

        None
    }
}
