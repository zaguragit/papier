
#[derive(Debug, Default)]
pub struct TextContent {
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Debug)]
pub enum Paragraph {
    Text(String),
    H2(String),
    H3(String),
    H4(String),
}