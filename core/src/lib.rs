use std::ffi::{c_char, CStr, CString};
mod air;
mod db;
mod parsing;
mod rendering;
mod matching;

pub use air::*;
pub use db::AmbleDB;
use db::DbBlockMatrix;
use matching::expand_folds;
pub use parsing::Parser;

use anyhow::{anyhow, Context};
use rendering::render_to_org;

#[repr(C)]
pub struct TopLevelCategory {
    id: i64,
    name: *const c_char,
    content: *const c_char,
}

pub struct RTopLevelCategory {
    id: i64,
    name: String,
    content: String,
}

impl TopLevelCategory {
    fn to_rust(&self) -> Result<RTopLevelCategory, anyhow::Error> {
        if self.name.is_null() {
            return Err(anyhow!("Name was null"));
        }
        if self.content.is_null() {
            return Err(anyhow!("Content was null"));
        }

        let name = unsafe { CStr::from_ptr(self.name) };
        let content = unsafe { CStr::from_ptr(self.content) };

        Ok(RTopLevelCategory {
            id: self.id,
            name: name.to_str().context("Invalid UTF-8 for name")?.to_owned(),
            content: content
                .to_str()
                .context("Invalid UTF-8 for name")?
                .to_owned(),
        })
    }
}

#[no_mangle]
pub extern "C" fn write_category(category: *const TopLevelCategory) -> TopLevelCategory {
    let rust_category = unsafe {
        category
            .as_ref()
            .expect("Could not convert category to non reference type")
            .to_rust()
            .expect("Could not convert category pointer to rust category")
    };

    let parser = Parser::new(&rust_category.content);
    let mut folds = Vec::new();
    let mut captures = Vec::new();

    let blocks = expand_folds(&parser, &mut folds, &mut captures);

    let mut db = AmbleDB::new("amble.sqlite").expect("Could not create db");

    let category = CategoryBlock {
        id: if rust_category.id > 0 { Some(rust_category.id) } else { None },
        name: &rust_category.name,
        children: blocks,
        level: 1,
        matches: Vec::new(),
        captures: Vec::new()
    };

    let last_id = db.write_top_level_category(&category)
        .expect("Should be able to save category to database");

    let cat_id = if rust_category.id > 0 { rust_category.id } else { last_id };

    let matrix = DbBlockMatrix::new(&db.connection, cat_id).expect("Could not create db block matrix");

    let flat_blocks = matrix
        .produce_flat_db_block_vec(&db.connection)
        .expect("Could not produce flat vec of db blocks");

    let category_block = matrix
        .form_category_block_tree(&flat_blocks)
        .expect("Could not get category block");

    let out_string = render_to_org(Block::Category(category_block));

    dbg!(&out_string);

    let out_cat = TopLevelCategory {
        id: cat_id,
        name: CString::new(rust_category.name).unwrap().into_raw(),
        content: CString::new(out_string).unwrap().into_raw(),
    };

    out_cat
}

#[repr(C)]
pub struct TopLevelCategoryResult {
    id: i64,
    name: *const c_char,
}

#[repr(C)]
pub struct TopLevelCategoryResults {
    categories: *const TopLevelCategoryResult,
    length: usize,
}

#[no_mangle]
pub extern "C" fn get_top_level_categories() -> TopLevelCategoryResults {
    let mut db = AmbleDB::new("amble.sqlite").expect("Could not create db");

    let db_categories = db
        .get_top_level_categories()
        .expect("Should be able to get categories");

    let mut tl_categories: Vec<TopLevelCategoryResult> = db_categories
        .into_iter()
        .map(|cat| TopLevelCategoryResult {
            id: cat.id.expect("Id was not present"),
            name: CString::new(cat.name).unwrap().into_raw(),
        })
        .collect();

    let tl_results = TopLevelCategoryResults {
        categories: tl_categories.as_mut_ptr(),
        length: tl_categories.len(),
    };

    std::mem::forget(tl_categories);

    return tl_results;
}

#[no_mangle]
pub extern "C" fn get_category_content(id: i64) -> *mut c_char {
    let db = AmbleDB::new("amble.sqlite").expect("Could not create db");

    let matrix = DbBlockMatrix::new(&db.connection, id).expect("Could not create db block matrix");

    let flat_blocks = matrix
        .produce_flat_db_block_vec(&db.connection)
        .expect("Could not produce flat vec of db blocks");

    let category_block = matrix
        .form_category_block_tree(&flat_blocks)
        .expect("Could not get category block");

    let content = render_to_org(Block::Category(category_block));

    CString::new(content).unwrap().into_raw()
}
