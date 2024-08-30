pub const AMBLE_DB_SCHEMA: &str = "
    CREATE TABLE category_blocks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT,
        parent_category_id INTEGER NULL REFERENCES category_blocks(id) ON DELETE CASCADE
    );
    CREATE TABLE rich_text_blocks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        parent_category_id INTEGER NULL REFERENCES category_blocks(id) ON DELETE CASCADE
    );
    CREATE TABLE text_blocks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        content TEXT,
        parent_category_id INTEGER NULL REFERENCES category_blocks(id) ON DELETE CASCADE,
        parent_rich_text_block_id INTEGER NULL REFERENCES rich_text_blocks(id) ON DELETE CASCADE
    );";
