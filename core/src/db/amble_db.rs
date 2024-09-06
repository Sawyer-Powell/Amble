use std::env;

use anyhow::Context;
use rusqlite::Connection;

use crate::air::CategoryBlock;

use super::{schema::AMBLE_DB_SCHEMA, DbBlockMatrix};

pub struct AmbleDB {
    pub connection: Connection,
}

impl AmbleDB {
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
