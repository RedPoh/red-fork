use akidocs_core_rs::tokenizer::tokenize;
use akidocs_core_rs::tokens::{Header, Paragraph, Token};

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

#[test]
fn test_header_level_one() {
    let result = tokenize("# Hello");
    assert_eq!(
        result,
        vec![Token::Header(Header {
            level: 1,
            content: String::from("Hello")
        }),]
    );
}

#[test]
fn test_header_levels() {
    assert_eq!(
        tokenize("# One")[0],
        Token::Header(Header {
            level: 1,
            content: String::from("One")
        })
    );
    assert_eq!(
        tokenize("## Two")[0],
        Token::Header(Header {
            level: 2,
            content: String::from("Two")
        })
    );
    assert_eq!(
        tokenize("### Three")[0],
        Token::Header(Header {
            level: 3,
            content: String::from("Three")
        })
    );
    assert_eq!(
        tokenize("#### Four")[0],
        Token::Header(Header {
            level: 4,
            content: String::from("Four")
        })
    );
    assert_eq!(
        tokenize("##### Five")[0],
        Token::Header(Header {
            level: 5,
            content: String::from("Five")
        })
    );
    assert_eq!(
        tokenize("###### Six")[0],
        Token::Header(Header {
            level: 6,
            content: String::from("Six")
        })
    );
}

#[test]
fn test_header_requires_space() {
    let result = tokenize("#NoSpace");
    assert_eq!(
        result,
        vec![Token::Paragraph(Paragraph {
            content: String::from("#NoSpace")
        }),]
    );
}

#[test]
fn test_header_empty_content() {
    let result = tokenize("##");
    assert_eq!(
        result,
        vec![Token::Header(Header {
            level: 2,
            content: String::from("")
        }),]
    );
}

#[test]
fn test_header_level_seven_becomes_paragraph() {
    let result = tokenize("####### Seven hashes");
    assert_eq!(
        result,
        vec![Token::Paragraph(Paragraph {
            content: String::from("####### Seven hashes")
        }),]
    );
}

#[test]
fn test_header_with_tab_separator() {
    let result = tokenize("#\tHello");
    assert_eq!(
        result,
        vec![Token::Header(Header {
            level: 1,
            content: String::from("Hello")
        }),]
    );
}

#[test]
fn test_header_with_mixed_whitespace_separator() {
    let result = tokenize("##   \t\t   \t\t   Mixed");
    assert_eq!(
        result,
        vec![Token::Header(Header {
            level: 2,
            content: String::from("Mixed")
        }),]
    );
}
