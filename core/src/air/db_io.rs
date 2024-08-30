use rusqlite::{Transaction, Connection};
use anyhow::{anyhow, Context};

#[derive(Debug)]
pub enum DbBlock {
    Category(DbCategoryBlock),
    RichText(DbRichTextBlock),
    Text(DbTextBlock),
}

pub trait DbIO {
    /// Initializes the structure on the stack with default values
    fn zero() -> Self;
    /// Returns the id of the newly created entity
    fn write(&self, tx: &Transaction) -> Result<i64, anyhow::Error>;
    /// Returns the number of items deleted in the database
    fn delete(&self, tx: &Transaction) -> Result<usize, anyhow::Error>;
    /// Creates the struct using a select statement from the database
    fn select(&mut self, connection: &Connection, id: i64) -> Result<(), anyhow::Error>;
}

#[derive(Debug)]
pub struct DbCategoryBlock {
    pub id: Option<i64>,
    pub name: String,
    pub parent_category_id: Option<i64>,
}

impl Clone for DbCategoryBlock {
    fn clone(&self) -> Self {
        DbCategoryBlock {
            id: self.id,
            name: self.name.clone(),
            parent_category_id: self.parent_category_id.clone()
        }
    }
}

impl DbIO for DbCategoryBlock {
    fn zero() -> Self {
        DbCategoryBlock {
            id: Some(0),
            name: "".to_string(),
            parent_category_id: Some(0),
        }
    }

    fn select(&mut self, connection: &Connection, id: i64) -> Result<(), anyhow::Error> {
        let mut stmt = connection
            .prepare(
                "
            SELECT id, name, parent_category_id
            FROM category_blocks
            WHERE id = ?1",
            )
            .context("Could not prepare select statement")?;

        let mut row_iter = stmt
            .query_map([id], |row| {
                Ok(DbCategoryBlock {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    parent_category_id: row.get(2)?,
                })
            })
            .context("Could not prepare select statement query map")?;

        match row_iter.nth(0) {
            Some(v) => {
                let block = v.context("Failed to get category from query")?;

                self.id = block.id;
                self.name = block.name;
                self.parent_category_id = block.parent_category_id;

                Ok(())
            }
            None => Ok(()),
        }
    }

    fn write(&self, tx: &Transaction) -> Result<i64, anyhow::Error> {
        if self.id != None {
            self.delete(tx)
                .context("Could not perform deletion operation on category block")?;
        }

        tx.execute(
            "
            INSERT INTO category_blocks (id, name, parent_category_id)
            VALUES (?1, ?2, ?3)",
            (&self.id, &self.name, &self.parent_category_id),
        )
        .context("Could not perform insert of category block")?;

        let id = match self.id {
            None => tx.last_insert_rowid(),
            Some(id) => id,
        };

        Ok(id)
    }

    fn delete(&self, tx: &Transaction) -> Result<usize, anyhow::Error> {
        if self.id == None {
            return Err(anyhow!(
                "Attempting to delete a category block that does not have id"
            ));
        }

        let db_id = self.id.unwrap();

        let count = tx
            .execute("DELETE from category_blocks WHERE id = ?1", [db_id])
            .context(format!("Could not delete category block with id {}", db_id))?;

        Ok(count)
    }
}

#[derive(Debug)]
pub struct DbRichTextBlock {
    pub id: Option<i64>,
    pub parent_category_id: Option<i64>,
}

impl Clone for DbRichTextBlock {
    fn clone(&self) -> Self {
        DbRichTextBlock {
            id: self.id,
            parent_category_id: self.parent_category_id
        }
    }
}

impl DbIO for DbRichTextBlock {
    fn zero() -> Self {
        DbRichTextBlock {
            id: Some(0),
            parent_category_id: Some(0),
        }
    }

    fn select(&mut self, connection: &Connection, id: i64) -> Result<(), anyhow::Error> {
        let mut stmt = connection
            .prepare(
                "
            SELECT id, parent_category_id
            FROM rich_text_blocks
            WHERE id = ?1",
            )
            .context("Could not prepare select statement")?;

        let mut row_iter = stmt
            .query_map([id], |row| {
                Ok(DbRichTextBlock {
                    id: row.get(0)?,
                    parent_category_id: row.get(1)?,
                })
            })
            .context("Could not prepare select statement query map")?;

        match row_iter.nth(0) {
            Some(v) => {
                let block = v.context("Failed to get category from query")?;

                self.id = block.id;
                self.parent_category_id = block.parent_category_id;

                Ok(())
            }
            None => Ok(()),
        }
    }

    fn write(&self, tx: &Transaction) -> Result<i64, anyhow::Error> {
        if self.id != None {
            self.delete(tx)
                .context("Could not perform deletion operation on rich text block")?;
        }

        tx.execute(
            "
            INSERT INTO rich_text_blocks (id, parent_category_id)
            VALUES (?1, ?2)",
            (&self.id, &self.parent_category_id),
        )
        .context("Could not insert rich text block")?;

        let id = match self.id {
            None => tx.last_insert_rowid(),
            Some(id) => id,
        };

        Ok(id)
    }

    fn delete(&self, tx: &Transaction) -> Result<usize, anyhow::Error> {
        if self.id == None {
            return Err(anyhow!(
                "Attempting to delete a rich text block that does not have id"
            ));
        }

        let db_id = self.id.unwrap();

        let count = tx
            .execute("DELETE from rich_text_blocks WHERE id = ?1", [db_id])
            .context(format!(
                "Could not delete rich text block with id {}",
                db_id
            ))?;

        Ok(count)
    }
}

#[derive(Debug)]
pub struct DbTextBlock {
    pub id: Option<i64>,
    pub content: String,
    pub parent_category_id: Option<i64>,
    pub parent_rich_text_block_id: Option<i64>,
}

impl Clone for DbTextBlock {
    fn clone(&self) -> Self {
        DbTextBlock {
            id: self.id,
            content: self.content.clone(),
            parent_category_id: self.parent_category_id,
            parent_rich_text_block_id: self.parent_rich_text_block_id
        }
    }
}

impl DbIO for DbTextBlock {
    fn zero() -> Self {
        DbTextBlock {
            id: Some(0),
            content: "".to_string(),
            parent_category_id: Some(0),
            parent_rich_text_block_id: Some(0),
        }
    }

    fn select(&mut self, connection: &Connection, id: i64) -> Result<(), anyhow::Error> {
        let mut stmt = connection
            .prepare(
                "
            SELECT id, content, parent_category_id, parent_rich_text_block_id
            FROM text_blocks
            WHERE id = ?1",
            )
            .context("Could not prepare select statement")?;

        let mut row_iter = stmt
            .query_map([id], |row| {
                Ok(DbTextBlock {
                    id: row.get(0)?,
                    content: row.get(1)?,
                    parent_category_id: row.get(2)?,
                    parent_rich_text_block_id: row.get(3)?,
                })
            })
            .context("Could not prepare select statement query map")?;

        match row_iter.nth(0) {
            Some(v) => {
                let block = v.context("Failed to get category from query")?;

                self.id = block.id;
                self.content = block.content;
                self.parent_category_id = block.parent_category_id;
                self.parent_rich_text_block_id = block.parent_rich_text_block_id;

                Ok(())
            }
            None => Ok(()),
        }
    }

    fn write(&self, tx: &Transaction) -> Result<i64, anyhow::Error> {
        tx.execute(
            "
            INSERT INTO text_blocks (id, content, parent_category_id, parent_rich_text_block_id)
            VALUES (?1, ?2, ?3, ?4)",
            (
                &self.id,
                &self.content,
                &self.parent_category_id,
                &self.parent_rich_text_block_id,
            ),
        )
        .context("Could not insert text block into database")?;

        let id = match self.id {
            None => tx.last_insert_rowid(),
            Some(id) => id,
        };

        Ok(id)
    }

    fn delete(&self, tx: &Transaction) -> Result<usize, anyhow::Error> {
        if self.id == None {
            return Err(anyhow!(
                "Attempting to delete a text block that does not have id"
            ));
        }

        let db_id = self.id.unwrap();

        let count = tx
            .execute("DELETE from text_blocks WHERE id = ?1", [db_id])
            .context(format!("Could not delete text block with id {}", db_id))?;

        Ok(count)
    }
}
