use crate::{ast::AstNode, token::HeadingLevel};

pub fn generate_html(ast: &[AstNode]) -> String {
    let mut result = String::new();
    for node in ast {
        match node {
            AstNode::Heading(HeadingLevel::H1, text) => {
                result.push_str(&format!("<h1>{}</h1>", text));
            }
            AstNode::Heading(HeadingLevel::H2, text) => {
                result.push_str(&format!("<h2>{}</h2>", text));
            }
            AstNode::Heading(HeadingLevel::H3, text) => {
                result.push_str(&format!("<h3>{}</h3>", text));
            }
            AstNode::Heading(HeadingLevel::H4, text) => {
                result.push_str(&format!("<h4>{}</h4>", text));
            }
            AstNode::Heading(HeadingLevel::H5, text) => {
                result.push_str(&format!("<h5>{}</h5>", text));
            }
            AstNode::Heading(HeadingLevel::H6, text) => {
                result.push_str(&format!("<h6>{}</h6>", text));
            }
            AstNode::BlockQuotes(text) => {
                result.push_str(&format!("<blockquote>{}</blockquote>", text));
            }
            AstNode::Lists(text) => {
                result.push_str(&format!("<ul><li>{}</li></ul>", text));
            }
            AstNode::Bold(text) => {
                result.push_str(&format!("<b>{}</b>", text));
            }
            AstNode::Italic(text) => {
                result.push_str(&format!("<i>{}</i>", text));
            }
            AstNode::Text(text) => {
                result.push_str(&text);
            }
            AstNode::Paragraph(nodes) => {
                result.push_str("<p>");
                result.push_str(&generate_html(nodes));
                result.push_str("</p>");
            }
        }
    }
    result
}
