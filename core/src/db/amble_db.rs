use std::env;

use anyhow::Context;
use rusqlite::Connection;

use crate::{air::CategoryBlock, air::DbCategoryBlock};

use super::schema::AMBLE_DB_SCHEMA;

pub struct AmbleDB {
    pub connection: Connection,
}

impl<'a> AmbleDB {
    pub fn new(filename: &str) -> Result<Self, anyhow::Error> {
        let current_dir = match env::current_dir() {
            Ok(path) => path,
            Err(error) => panic!("Could not get current directory {error:?}"),
        };
        let db_path = current_dir.join(filename);
        let db_existed = db_path.exists();

        let connection = Connection::open(db_path)?;

        if !db_existed {
            connection.execute_batch(AMBLE_DB_SCHEMA)?;
        }

        return Ok(AmbleDB { connection });
    }

    pub fn get_top_level_categories(&mut self) -> Result<Vec<DbCategoryBlock>, anyhow::Error> {
        let mut stmt = self
            .connection
            .prepare(
                "
            SELECT id, name
            FROM category_blocks
            WHERE parent_category_id = null",
            )
            .context("Could not prepare select statement")?;

        let mut categories: Vec<DbCategoryBlock> = Vec::new();

        let row_iter = stmt
            .query_map([], |row| {
                Ok(DbCategoryBlock {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    parent_category_id: row.get(2)?,
                })
            })
            .context("Could not prepare select statement query map")?;

        for row in row_iter {
            let category = row.context("Error getting row")?;
            categories.push(category);
        }

        return Ok(categories);
    }

    pub fn write_top_level_category(
        &mut self,
        category: &CategoryBlock,
    ) -> Result<(), anyhow::Error> {
        // NOTE: Synchronous off is used to significantly increase our write
        // speeds as far as this program is concerned. Has some safety/data
        // integrity implications

        self.connection.execute("PRAGMA synchronous = OFF", ())?;

        let tx = self
            .connection
            .transaction()
            .context("Could not create a new transaction")?;

        category
            .write_to_db(&tx, None)
            .context("Should be able to write categroy to db")?;

        tx.commit().context("Could not commit transaction")?;

        Ok(())
    }
}
