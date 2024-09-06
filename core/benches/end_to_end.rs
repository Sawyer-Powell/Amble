use amble::Parser;
use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs, hint::black_box};

fn parse_file(file: &str) {
    let parser = Parser::new(&file);
    parser.parse();
}

fn parsing(c: &mut Criterion) {
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

        let file = fs::read_to_string(&path).expect("Should be able to read file");

        c.bench_function(&format!("{} - parsing", test_name), |b| {
            b.iter(|| parse_file(black_box(&file)))
        });
    }
}

criterion_group!(benches, parsing);
criterion_main!(benches);
