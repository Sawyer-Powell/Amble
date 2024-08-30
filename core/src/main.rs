mod air;
mod db;
mod parsing;

use std::fs;

use air::CategoryBlock;
use db::AmbleDB;
use parsing::Parser;

fn main() {
    let document =
        fs::read_to_string("../test/org-files/mapreduce.org").expect("Should be able to open file");

    let blocks = Parser::new(&document).parse();

    let mut db = AmbleDB::new("amble.sqlite").expect("Could not create db");

    let category = CategoryBlock {
        id: Some(1),
        name: "MapReduce".to_string(),
        children: blocks,
        level: 0,
    };

    db.write_top_level_category(&category)
        .expect("Should be able to save category to database");

    db.get_top_level_category(1)
        .expect("Should be able to get category from database");
}
