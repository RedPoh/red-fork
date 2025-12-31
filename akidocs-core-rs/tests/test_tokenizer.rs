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

#[test]
fn test_whitespace_only_returns_empty_list() {
    assert_eq!(tokenize("   "), vec![]);
    assert_eq!(tokenize("\n\n"), vec![]);
    assert_eq!(tokenize("  \n\n  "), vec![]);
}

#[test]
fn test_multiple_paragraphs() {
    let result = tokenize("First paragraph\n\nSecond paragraph");
    assert_eq!(
        result,
        vec![
            Token::Paragraph(Paragraph {
                content: String::from("First paragraph")
            }),
            Token::Paragraph(Paragraph {
                content: String::from("Second paragraph")
            }),
        ]
    );
}

#[test]
fn test_single_newline_stays_in_paragraph() {
    let result = tokenize("Line one\nLine two");
    assert_eq!(
        result,
        vec![Token::Paragraph(Paragraph {
            content: String::from("Line one\nLine two")
        }),]
    );
}

#[test]
fn test_multiple_blank_lines() {
    let result = tokenize("First\n\n\n\nSecond");
    assert_eq!(
        result,
        vec![
            Token::Paragraph(Paragraph {
                content: String::from("First")
            }),
            Token::Paragraph(Paragraph {
                content: String::from("Second")
            }),
        ]
    );
}

#[test]
fn test_leading_trailing_whitespace_ignored() {
    let result = tokenize("\n\nContent\n\n");
    assert_eq!(
        result,
        vec![Token::Paragraph(Paragraph {
            content: String::from("Content")
        }),]
    );
}

#[test]
fn test_windows_crlf_line_endings() {
    let result = tokenize("First paragraph\r\n\r\nSecond paragraph");
    assert_eq!(
        result,
        vec![
            Token::Paragraph(Paragraph {
                content: String::from("First paragraph")
            }),
            Token::Paragraph(Paragraph {
                content: String::from("Second paragraph")
            }),
        ]
    );
}
