use akidocs_core_rs::tokenizer::tokenize;
use akidocs_core_rs::tokens::{Paragraph, Token};

#[test]
fn test_empty_string_returns_empty_list() {
    assert_eq!(tokenize(""), vec![]);
}

#[test]
fn test_plain_paragraph() {
    let result = tokenize("Hello world");
    assert_eq!(
        result,
        vec![Token::Paragraph(Paragraph {
            content: String::from("Hello world"),
        })]
    );
}
