use crate::tokens::{Paragraph, Token};

pub fn tokenize(text: &str) -> Vec<Token> {
    if text.is_empty() {
        return vec![];
    }

    vec![Token::Paragraph(Paragraph {
        content: String::from(text),
    })]
}
