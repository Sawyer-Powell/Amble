use std::ffi::{c_char, CStr, CString};

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

    CString::new(rust_category.content).unwrap().into_raw()
}
