use amble::{AmbleDB, CategoryBlock, Parser};
use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs, hint::black_box, path::PathBuf};

fn file_load_then_return(file_path: &PathBuf) {
    let file = fs::read_to_string(file_path).expect("Should be able to read file");

    let blocks = Parser::new(&file).parse();

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

fn end_to_end(c: &mut Criterion) {
    for entry in fs::read_dir("test/org").expect("There should be a test/org folder") {
        let entry = entry.expect("There should exist at least one entry in the test/org folder");
        let path = entry.path();
        let file_name = path.file_name()
            .expect("There should be a file name");
        let file_name = file_name.to_string_lossy();
        let test_name = file_name.as_ref();

        if let Some(char) = test_name.chars().next() {
            if char == '=' {
                continue;
            }
        }

        c.bench_function(test_name, |b| {
            b.iter(|| file_load_then_return(black_box(&path)))
        });
    }
}

criterion_group!(benches, end_to_end);
criterion_main!(benches);
