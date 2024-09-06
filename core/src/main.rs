mod air;
mod db;
mod parsing;

use std::fs;

pub use air::CategoryBlock;
pub use db::AmbleDB;
use db::DbBlockMatrix;
pub use parsing::Parser;

fn main() {
    let document =
        fs::read_to_string("test/org/mapreduce.org").expect("Should be able to open file");

    let parser = Parser::new(&document);
    let blocks = parser.parse();

    let mut db = AmbleDB::new("amble.sqlite").expect("Could not create db");

    let category = CategoryBlock {
        id: Some(1),
        name: "MapReduce",
        children: blocks,
        level: 0,
    };

    db.write_top_level_category(&category)
        .expect("Should be able to save category to database");

    let matrix = DbBlockMatrix::new(&db.connection, 1).expect("Could not create db block matrix");

    let flat_blocks = matrix
        .produce_flat_db_block_vec(&db.connection)
        .expect("Could not produce flat vec of db blocks");

    let category_block = matrix
        .get_category_block(&flat_blocks)
        .expect("Could not get category block");
}
