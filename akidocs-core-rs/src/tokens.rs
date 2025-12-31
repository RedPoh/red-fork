pub struct Header {
    pub level: u8,
    pub content: String, // Will become Vec<InlineText> later
}

#[derive(Debug, PartialEq)]
pub struct Paragraph {
    pub content: String, // Will become Vec<InlineText> later
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Paragraph(Paragraph),
    Header(Header),
}
