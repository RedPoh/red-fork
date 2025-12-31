use crate::tokens::{Header, Paragraph, Token};

fn try_parse_header(block: &str) -> Option<Header> {
    if !block.starts_with('#') {
        return None;
    }

    let stripped = block.trim_start_matches('#');
    let level = (block.len() - stripped.len()) as u8;

    if level > 6 {
        return None;
    }

    if !stripped.is_empty() && !stripped.starts_with([' ', '\t']) {
        return None;
    }

    Some(Header {
        level,
        content: String::from(stripped.trim()),
    })
}

pub fn tokenize(text: &str) -> Vec<Token> {
    let text = text.replace("\r\n", "\n");
    let mut tokens = Vec::new();

    for block in text.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        if let Some(header) = try_parse_header(block) {
            tokens.push(Token::Header(header));
        } else {
            tokens.push(Token::Paragraph(Paragraph {
                content: String::from(block),
            }));
        }
    }

    tokens
}
