use std::collections::HashSet;

use anyhow::{anyhow, Context};
use rusqlite::Connection;

use crate::air::{
    Block, CategoryBlock, DbBlock, DbCategoryBlock, DbIO, DbRichTextBlock, DbTextBlock,
    RichTextBlock, TextBlock,
};

#[derive(Debug)]
struct DbBlockMatrixRow {
    category_block_id: i64,
    rich_text_block_id: Option<i64>,
    text_block_id: Option<i64>,
}

#[derive(Debug)]
pub struct DbBlockMatrix {
    rows: Vec<DbBlockMatrixRow>,
}

// TODO: This approach to querying the database for a category
// and its children is very scary to look at. Should spend some
// making all this logic clearer

impl DbBlockMatrix {
    pub fn new(connection: &Connection, category_id: i64) -> Result<DbBlockMatrix, anyhow::Error> {
        let mut stmt = connection.prepare(
            "
            WITH RECURSIVE child_categories(id) AS (
                SELECT id FROM category_blocks 
                WHERE id = ?1
                UNION ALL
                SELECT cb.id 
                FROM category_blocks cb
                INNER JOIN child_categories cc ON cb.parent_category_id = cc.id
            )
            SELECT 
                cb.id AS category_block_id, 
                rtb.id AS rich_text_block_id, 
                tb.id AS text_block_id 
            FROM category_blocks cb
            LEFT JOIN rich_text_blocks rtb ON rtb.parent_category_id = cb.id
            LEFT JOIN text_blocks tb 
                ON rtb.id = tb.parent_rich_text_block_id 
                OR tb.parent_category_id = cb.id
            INNER JOIN child_categories cc ON cb.id = cc.id
            ",
        )?;

        let row_iter = stmt.query_map([category_id], |row| {
            Ok(DbBlockMatrixRow {
                category_block_id: row.get(0)?,
                rich_text_block_id: row.get(1)?,
                text_block_id: row.get(2)?,
            })
        })?;

        let mut block_matrix = DbBlockMatrix { rows: Vec::new() };

        for row in row_iter {
            block_matrix.rows.push(row?)
        }

        Ok(block_matrix)
    }

    pub fn produce_flat_db_block_vec(
        &self,
        connection: &Connection,
    ) -> Result<Vec<DbBlock>, anyhow::Error> {
        let mut db_blocks: Vec<DbBlock> = Vec::new();

        let mut category_block_ids_so_far: HashSet<i64> = HashSet::new();
        let mut rich_text_block_ids_so_far: HashSet<i64> = HashSet::new();
        let mut text_block_ids_so_far: HashSet<i64> = HashSet::new();

        for row in &self.rows {
            let mut db_category = DbCategoryBlock::zero();

            if !category_block_ids_so_far.contains(&row.category_block_id) {
                db_category
                    .select(connection, row.category_block_id)
                    .context(format!(
                        "Could not select db category block with id {}",
                        row.category_block_id
                    ))?;
                db_blocks.push(DbBlock::Category(db_category));
                category_block_ids_so_far.insert(row.category_block_id);
            }

            if let Some(rt_id) = row.rich_text_block_id {
                if !rich_text_block_ids_so_far.contains(&rt_id) {
                    let mut db_rich_text = DbRichTextBlock::zero();
                    db_rich_text.select(connection, rt_id).context(format!(
                        "Could not select db rich text block with id {}",
                        rt_id
                    ))?;
                    db_blocks.push(DbBlock::RichText(db_rich_text));
                    rich_text_block_ids_so_far.insert(rt_id);
                }
            }

            if let Some(t_id) = row.text_block_id {
                if !text_block_ids_so_far.contains(&t_id) {
                    let mut db_text = DbTextBlock::zero();
                    db_text
                        .select(connection, t_id)
                        .context(format!("Could not select db text block with id {}", t_id))?;
                    db_blocks.push(DbBlock::Text(db_text));
                    text_block_ids_so_far.insert(t_id);
                }
            }
        }

        Ok(db_blocks)
    }

    fn get_child_blocks<'a>(
        &'a self,
        db_blocks: &'a Vec<DbBlock>,
        parent_block: &DbBlock,
        start_index: usize,
        level: usize,
    ) -> Result<(Vec<Block>, usize), anyhow::Error> {
        let mut children: Vec<Block> = Vec::new();
        let mut index = start_index;

        while index < db_blocks.len() {
            let db_block = &db_blocks[index];
            index += 1;
            match parent_block {
                DbBlock::Category(db_category_parent) => match db_block {
                    DbBlock::Category(db_cat) => {
                        if let Some(parent_category_id) = db_cat.parent_category_id {
                            if parent_category_id
                                == db_category_parent
                                    .id
                                    .context("Id was not present on db category")?
                            {
                                let mut child_cat = CategoryBlock::from_db_type(db_cat, level);

                                let (new_children, new_index) = self
                                    .get_child_blocks(
                                        db_blocks,
                                        &DbBlock::Category(db_cat.clone()),
                                        index,
                                        level + 1,
                                    )
                                    .context("Failed to get child blocks of category")?;

                                index = new_index;
                                child_cat.children = new_children;

                                children.push(Block::Category(child_cat));
                            } else {
                                return Ok((children, index - 1))
                            }
                        }
                    }
                    DbBlock::RichText(db_rt) => {
                        if let Some(parent_category_id) = db_rt.parent_category_id {
                            if parent_category_id
                                == db_category_parent
                                    .id
                                    .context("Id was not present on db category")?
                            {
                                let mut child_rt = RichTextBlock::from_db_type(db_rt);

                                let (new_children, new_index) = self
                                    .get_child_blocks(
                                        db_blocks,
                                        &DbBlock::RichText(db_rt.clone()),
                                        index,
                                        level,
                                    )
                                    .context("Failed to get child blocks of category")?;

                                index = new_index;
                                child_rt.children = new_children;

                                children.push(Block::RichText(child_rt));
                            } else {
                                return Ok((children, index - 1))
                            }
                        }
                    }
                    DbBlock::Text(db_t) => {
                        if let Some(parent_category_id) = db_t.parent_category_id {
                            if parent_category_id
                                == db_category_parent
                                    .id
                                    .context("Id was not present on db category")?
                            {
                                let child_t = TextBlock::from_db_type(db_t);
                                children.push(Block::Text(child_t));
                            } else {
                                return Ok((children, index - 1))
                            }
                        }
                    }
                },
                DbBlock::RichText(db_rich_text_parent) => match db_block {
                    DbBlock::Text(db_t) => {
                        if let Some(parent_category_id) = db_t.parent_rich_text_block_id {
                            if parent_category_id
                                == db_rich_text_parent
                                    .id
                                    .context("Could not get id of rich text parent")?
                            {
                                let child_t = TextBlock::from_db_type(db_t);
                                children.push(Block::Text(child_t));
                            } else {
                                return Ok((children, index - 1))
                            }
                        }
                    }
                    _ => return Ok((children, index - 1)),
                },
                _ => return Ok((children, index - 1)),
            }
        }

        Ok((children, index))
    }

    pub fn form_category_block_tree<'a>(
        &'a self,
        db_blocks: &'a Vec<DbBlock>
    ) -> Result<CategoryBlock, anyhow::Error> {
        let db_block = &db_blocks[0];

        if let DbBlock::Category(db_cat_block) = db_block {
            let mut cat_block = CategoryBlock::from_db_type(&db_cat_block, 0);

            let (new_children, _) = self
                .get_child_blocks(&db_blocks, &db_block, 1, 1)
                .context("Could not get child blocks in get_category_blocks")?;

            cat_block.children = new_children;

            Ok(cat_block)
        } else {
            Err(anyhow!("First block in db_blocks was not a category"))
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use rusqlite::Connection;

    use crate::{air::*, db::AmbleDB, parsing::Parser};

    use core::panic;
    use std::{collections::HashSet, fs};

    use super::DbBlockMatrix;

    fn create_test_matrix() -> Result<(DbBlockMatrix, Connection), anyhow::Error> {
        let document = fs::read_to_string("../test/org-files/mapreduce.org")
            .context("Should be able to open file")?;

        let parser = Parser::new(&document);
        let blocks = parser.parse();


        let mut db = AmbleDB::new("amble-test.sqlite").context("Could not create db")?;

        let test_category = CategoryBlock {
            id: Some(1),
            name: "Test Category",
            level: 0,
            children: blocks,
            matches: Vec::new(),
            captures: Vec::new()
        };

        let tx = db
            .connection
            .transaction()
            .context("Should be able to create transaction")?;

        test_category
            .write_to_db(&tx, None)
            .context("Should be able to write test category to db")?;

        tx.commit().unwrap();

        let matrix = DbBlockMatrix::new(&db.connection, 1)
            .context("Should be able to create block matrix")?;

        Ok((matrix, db.connection))
    }

    #[test]
    fn ensure_ordering_of_block_matrix() {
        let (matrix, _) = create_test_matrix().expect("Should be able to create test matrix");

        let mut last_category_id: i64 = -1;
        let mut last_rtb_id: i64 = -1;
        let mut last_tb_id: i64 = -1;

        for row in matrix.rows {
            if row.category_block_id >= last_category_id {
                last_category_id = row.category_block_id;
            } else {
                panic!(
                    "Categories are not ordered, {} is not greater than {}",
                    row.category_block_id, last_category_id
                );
            }

            if let Some(rtb_id) = row.rich_text_block_id {
                if rtb_id >= last_rtb_id {
                    last_rtb_id = rtb_id;
                } else {
                    panic!(
                        "Rich text blocks are not ordered, {} is not greater than {}",
                        rtb_id, last_rtb_id
                    );
                }
            }

            if let Some(tb_id) = row.text_block_id {
                if tb_id >= last_tb_id {
                    last_tb_id = tb_id;
                } else {
                    panic!(
                        "Text blocks are not ordered, {} is not greater than {}",
                        tb_id, last_tb_id
                    );
                }
            }
        }
    }

    #[test]
    fn ensure_ordering_of_db_blocks() {
        let (matrix, connection) =
            create_test_matrix().expect("Should be able to create test matrix");
        let blocks = matrix
            .produce_flat_db_block_vec(&connection)
            .expect("Should be able to get db blocks");

        let mut category_block_ids_so_far: HashSet<i64> = HashSet::new();
        let mut rich_text_block_ids_so_far: HashSet<i64> = HashSet::new();
        let mut text_block_ids_so_far: HashSet<i64> = HashSet::new();

        for block in blocks {
            dbg!(&block);
            match block {
                DbBlock::Category(cat) => match cat.id {
                    Some(id) => {
                        if !category_block_ids_so_far.insert(id) {
                            panic!("Duplicate db category block {}", id)
                        }
                        if let Some(parent_cat) = cat.parent_category_id {
                            match category_block_ids_so_far.get(&parent_cat) {
                                None => panic!("Parent category {} of category {} not in category blocks so far", parent_cat, id),
                                _ => ()
                            }
                        }
                    }
                    _ => panic!("Category does not have an id"),
                },
                DbBlock::RichText(rt) => match rt.id {
                    Some(id) => {
                        if !rich_text_block_ids_so_far.insert(id) {
                            panic!("Duplicate db rich text block {}", id)
                        }
                        if let Some(parent_cat) = rt.parent_category_id {
                            match category_block_ids_so_far.get(&parent_cat) {
                                None => panic!("Parent category {} of rich text block {} not in category blocks so far", parent_cat, id),
                                _ => ()
                            }
                        }
                    }
                    None => panic!("Rich text block does not have an id"),
                },
                DbBlock::Text(t) => match t.id {
                    Some(id) => {
                        if !text_block_ids_so_far.insert(id) {
                            panic!("Duplicate db rich text block {}", id)
                        }
                        if let Some(parent_cat) = t.parent_category_id {
                            match category_block_ids_so_far.get(&parent_cat) {
                                None => panic!("Parent category {} of text block {} not in category blocks so far", parent_cat, id),
                                _ => ()
                            }
                        }
                        if let Some(parent_rt) = t.parent_rich_text_block_id {
                            match rich_text_block_ids_so_far.get(&parent_rt) {
                                None => panic!("Parent rich text_block {} of text block {} not in category blocks so far", parent_rt, id),
                                _ => ()
                            }
                        }
                    }
                    None => panic!("Text block does not have an id"),
                },
            }
        }
    }
}
