use std::ffi::{c_char, CStr, CString};
mod air;
mod db;
mod parsing;

pub use air::*;
pub use db::AmbleDB;
pub use parsing::Parser;

use anyhow::{anyhow, Context};

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
pub extern "C" fn write_category(category: *const TopLevelCategory) -> *mut c_char {
    let rust_category = unsafe {
        category
            .as_ref()
            .expect("Could not convert category to non reference type")
            .to_rust()
            .expect("Could not convert category pointer to rust category")
    };

    let blocks = Parser::new(&rust_category.content).parse();

    let mut db = AmbleDB::new("amble.sqlite").expect("Could not create db");

    let category = CategoryBlock {
        id: Some(1),
        name: rust_category.name,
        children: blocks,
        level: 0,
    };

    db.write_top_level_category(&category)
        .expect("Should be able to save category to database");

    db.get_top_level_category(1)
        .expect("Should be able to get category from database");

    CString::new(rust_category.content).unwrap().into_raw()
}
