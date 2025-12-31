use crate::tokens::{Paragraph, Token};

pub fn tokenize(text: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for block in text.split("\n\n") {
        let block = block.trim();
        if block.is_empty() {
            continue;
        }

        tokens.push(Token::Paragraph(Paragraph {
            content: String::from(block),
        }));
    }

    tokens
}
