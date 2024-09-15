use crate::Block;

pub fn render_to_org(block: Block) -> String {
    let mut out: String = "".to_string();

    match block {
        Block::Category(category_block) => {
            if category_block.level > 0 {
                out += &"*".repeat(category_block.level);
                out += &" ";
                out += category_block.name;
            }

            for block in category_block.children {
                out += &format!("{}", render_to_org(block));
            }
        },
        Block::RichText(rich_text_block) => {
            for block in rich_text_block.children {
                out += &format!("{}", render_to_org(block));
            }
        },
        Block::Text(text_block) => {
            out += text_block.content;
        }
    }

    return out;
}
