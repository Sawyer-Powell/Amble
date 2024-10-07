mod air;
mod db;
mod parsing;
mod rendering;
mod matching;

use std::fs;

pub use air::CategoryBlock;
pub use db::AmbleDB;
pub use parsing::Parser;

use air::Block;
use db::DbBlockMatrix;
use rendering::render_to_org;

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
        level: 1,
        matches: Vec::new(),
        captures: Vec::new()
    };

    db.write_top_level_category(&category)
        .expect("Should be able to save category to database");

    let matrix = DbBlockMatrix::new(&db.connection, 1).expect("Could not create db block matrix");

    let flat_blocks = matrix
        .produce_flat_db_block_vec(&db.connection)
        .expect("Could not produce flat vec of db blocks");

    let category_block = matrix
        .form_category_block_tree(&flat_blocks)
        .expect("Could not get category block");

    dbg!(&category_block);

    println!("{}", render_to_org(Block::Category(category_block)));
}
