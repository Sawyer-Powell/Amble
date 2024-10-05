use std::collections::HashSet;

use anyhow::{anyhow, Context};
use rusqlite::Transaction;
use crate::matching::fold::CategoryMatcher;

use super::db_io::{DbCategoryBlock, DbIO, DbRichTextBlock, DbTextBlock};

/**
* This file contains the specification of the Amble Intermediate Representation,
* abbreviated 'AIR'. AIR is produced from amble parsers and serves as the standard
* language that amble uses to reason about information it is tracking.
*/

#[derive(Debug)]
pub enum Block<'a> {
    Category(CategoryBlock<'a>),
    RichText(RichTextBlock<'a>),
    Text(TextBlock<'a>),
}

// -----------------------------------------------------------


#[derive(Debug)]
pub struct CategoryBlock<'a> {
    pub id: Option<i64>,
    pub name: &'a str,
    pub level: usize,
    pub children: Vec<Block<'a>>,
    pub matches: Vec<&'a CategoryMatcher>
}

impl<'a> Clone for CategoryBlock<'a> {
    fn clone(&self) -> Self {
        CategoryBlock {
            id: self.id,
            name: self.name,
            level: self.level,
            children: Vec::new(),
            matches: Vec::new()
        }
    }
}

impl<'a> CategoryBlock<'a> {
    fn as_db_type(&self, parent_category_id: Option<i64>) -> DbCategoryBlock {
        DbCategoryBlock {
            id: self.id,
            name: self.name.to_string(),
            parent_category_id,
        }
    }

    pub fn from_db_type(db_block: &'a DbCategoryBlock, level: usize) -> Self {
        CategoryBlock {
            id: db_block.id.clone(),
            name: &db_block.name,
            level,
            children: Vec::new(),
            matches: Vec::new()
        }
    }

    pub fn write_to_db(
        &self,
        tx: &Transaction,
        parent_category_id: Option<i64>,
    ) -> Result<i64, anyhow::Error> {
        let db_id = self
            .as_db_type(parent_category_id)
            .write(tx)
            .context("Could not perform write of category block")?;

        for child in &self.children {
            match child {
                Block::Category(cat) => cat
                    .write_to_db(tx, Some(db_id))
                    .context("Could not write category")?,
                Block::RichText(rt) => rt
                    .write_to_db(tx, None, Some(db_id))
                    .context("Could not write rich text block")?,
                Block::Text(t) => t
                    .write_to_db(tx, None, Some(db_id), None)
                    .context("Could not write text block")?,
            };
        }

        Ok(db_id)
    }
}

// -----------------------------------------------------------

#[derive(Debug)]
pub struct RichTextBlock<'a> {
    pub children: Vec<Block<'a>>,
}

impl<'a> Clone for RichTextBlock<'a> {
    fn clone(&self) -> Self {
        RichTextBlock {
            children: Vec::new(),
        }
    }
}

impl<'a> RichTextBlock<'a> {
    fn as_db_type(&self, id: Option<i64>, parent_category_id: Option<i64>) -> DbRichTextBlock {
        DbRichTextBlock {
            id,
            parent_category_id,
        }
    }

    pub fn from_db_type(_db_block: &DbRichTextBlock) -> Self {
        RichTextBlock {
            children: Vec::new(),
        }
    }

    pub fn write_to_db(
        &self,
        tx: &Transaction,
        id: Option<i64>,
        parent_category_id: Option<i64>,
    ) -> Result<i64, anyhow::Error> {
        let db_id = self.as_db_type(id, parent_category_id).write(tx)?;

        for child in &self.children {
            match child {
                Block::Category(_) => {
                    return Err(anyhow!("Cannot have a category child of a rich text block",))
                }
                Block::RichText(_) => {
                    return Err(anyhow!(
                        "Cannot have a rich text block child of a rich text block",
                    ))
                }
                Block::Text(t) => {
                    t.write_to_db(tx, None, None, Some(db_id))
                        .context(
                            format!(
                                "Could not write text block to database as child of rich text block with id {}",
                                db_id))?;
                    ()
                }
            }
        }

        Ok(db_id)
    }
}

// -----------------------------------------------------------

#[derive(Debug)]
pub struct TextBlock<'a> {
    pub content: &'a str ,
}

impl<'a> Clone for TextBlock<'a> {
    fn clone(&self) -> Self {
        TextBlock {
            content: self.content,
        }
    }
}

impl<'a> TextBlock<'a> {
    fn as_db_type(
        &self,
        id: Option<i64>,
        parent_category_id: Option<i64>,
        parent_rich_text_block_id: Option<i64>,
    ) -> DbTextBlock {
        DbTextBlock {
            id,
            content: self.content.to_string(),
            parent_category_id,
            parent_rich_text_block_id,
        }
    }

    pub fn from_db_type(db_block: &'a DbTextBlock) -> Self {
        TextBlock {
            content: &db_block.content,
        }
    }

    pub fn write_to_db(
        &self,
        tx: &Transaction,
        id: Option<i64>,
        parent_category_id: Option<i64>,
        parent_rich_text_block_id: Option<i64>,
    ) -> Result<i64, anyhow::Error> {
        let db_id = self
            .as_db_type(id, parent_category_id, parent_rich_text_block_id)
            .write(tx)
            .context("Could not write text block into database")?;

        Ok(db_id)
    }
}

// -----------------------------------------------------------
