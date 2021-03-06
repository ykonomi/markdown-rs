use block::Block;
use block::BlockType;
use block_parser;
use htmlescape::encode_minimal;
use inline_parser;
use std::collections::HashMap;

fn print(tree: Block, mut env: &mut HashMap<String, String>) -> String {
    match tree {
        Block {
            block_type: BlockType::Document,
            children,
            ..
        } => {
            let mut result_str = String::new();
            for v in children {
                result_str.push_str(&print(v, &mut env))
            }
            result_str
        }
        Block {
            block_type: BlockType::ThematicBreaks,
            ..
        } => {
            let mut result_str = String::with_capacity(6);
            result_str.push_str("<hr />");
            result_str
        }
        Block {
            block_type: BlockType::BreakLine,
            ..
        } => "".to_string(),
        Block {
            block_type: BlockType::Paragraph,
            raw_text,
            ..
        } => format!("<p>{}</p>", raw_text),
        Block {
            block_type: BlockType::AtxHeading1,
            raw_text,
            ..
        } => format!("<h1>{}</h1>", raw_text),
        Block {
            block_type: BlockType::AtxHeading2,
            raw_text,
            ..
        } => format!("<h2>{}</h2>", raw_text),
        Block {
            block_type: BlockType::AtxHeading3,
            raw_text,
            ..
        } => format!("<h3>{}</h3>", raw_text),
        Block {
            block_type: BlockType::AtxHeading4,
            raw_text,
            ..
        } => format!("<h4>{}</h4>", raw_text),
        Block {
            block_type: BlockType::AtxHeading5,
            raw_text,
            ..
        } => format!("<h5>{}</h5>", raw_text),
        Block {
            block_type: BlockType::AtxHeading6,
            raw_text,
            ..
        } => format!("<h6>{}</h6>", raw_text),
        Block {
            block_type: BlockType::SetextHeadingUnderline1,
            raw_text,
            ..
        } => format!("<h1>{}</h1>", raw_text),
        Block {
            block_type: BlockType::SetextHeadingUnderline2,
            raw_text,
            ..
        } => format!("<h2>{}</h2>", raw_text),
        Block {
            block_type: BlockType::IndentedCodeBlock,
            raw_text,
            ..
        } => format!("<pre><code>{}</code></pre>", encode_minimal(&raw_text)),
        Block {
            block_type: BlockType::FencedCodeBlock,
            raw_text,
            ..
        } => format!("<pre><code>{}</code></pre>", encode_minimal(&raw_text)),
        Block {
            block_type: BlockType::BlockQuote,
            children,
            ..
        } => {
            let mut result_str = String::new();
            for v in children {
                result_str.push_str(&print(v, &mut env))
            }
            format!("<blockquote>{}</blockquote>", result_str)
        }
        Block {
            block_type: BlockType::BulletListItem,
            children,
            ..
        } => {
            if children.len() <= 2 {
                //  Paragraph + BreakLine | Paragraph
                return format!(
                    "<ul><li>{}</li></ul>",
                    children.iter().next().unwrap().get_text()
                );
            }
            let mut result_str = String::new();
            // ad_hoc
            for v in children {
                result_str.push_str(&print(v, &mut env))
            }
            format!("<ul><li>{}</li></ul>", result_str)
        }
        Block {
            block_type: BlockType::OrderedListItem,
            children,
            ..
        } => {
            let mut result_str = String::new();
            for v in children {
                result_str.push_str(&print(v, &mut env))
            }
            format!("<ol><li>{}</li></ol>", result_str)
        }
        Block {
            block_type: BlockType::LinkDefinition,
            raw_text,
            children,
            ..
        } => {
            let mut result_str = String::new();
            for v in children {
                result_str.push_str(&print(v, &mut env))
            }
            env.insert(raw_text.to_string(), result_str);
            "".to_string()
        }
        Block {
            block_type: BlockType::ReferenceLink,
            raw_text,
            ..
        } => match env.get(&raw_text) {
            Some(html) => html.to_string(),
            None => "".to_string(),
        },
    }
}

pub fn top(input_str: &str) -> String {
    // Add line feed.
    let mut input = String::new();
    input.push_str(input_str);
    input.push_str("\n");

    //let mut input = convert_tabs(&input);
    let mut block_tree = block_parser::top(&input);
    inline_parser::top(&mut block_tree);
    let mut env = HashMap::new();
    print(block_tree, &mut env)
}
