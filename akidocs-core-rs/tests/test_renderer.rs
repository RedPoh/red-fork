use akidocs_core_rs::renderer::render_pdf;
use akidocs_core_rs::tokens::{Paragraph, Token};

fn assert_valid_pdf_bytes(result: &[u8]) {
    assert!(!result.is_empty());
}

#[test]
fn test_render_returns_bytes() {
    let tokens = vec![Token::Paragraph(Paragraph {
        content: String::from("Hello"),
    })];
    let result = render_pdf(&tokens);
    assert_valid_pdf_bytes(&result);
}
