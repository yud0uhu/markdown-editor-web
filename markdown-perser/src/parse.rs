use crate::{ast::AstNode, token::Token};

pub fn parse(tokens: &[Token]) -> Vec<AstNode> {
    let mut result = Vec::new();
    let mut current_paragraph = Vec::new();
    let mut in_bold = false;
    let mut in_italic = false;

    for token in tokens {
        match token {
            Token::Heading(level, text) => {
                if !current_paragraph.is_empty() {
                    result.push(AstNode::Paragraph(current_paragraph.clone()));
                    current_paragraph.clear();
                }
                result.push(AstNode::Heading(level.clone(), text.clone()));
            }
            Token::BlockQuotes(text) => {
                if !current_paragraph.is_empty() {
                    result.push(AstNode::Paragraph(current_paragraph.clone()));
                    current_paragraph.clear();
                }
                result.push(AstNode::BlockQuotes(text.clone()));
            }
            Token::Lists(text) => {
                if !current_paragraph.is_empty() {
                    result.push(AstNode::Paragraph(current_paragraph.clone()));
                    current_paragraph.clear();
                }
                result.push(AstNode::Lists(text.clone()));
            }
            Token::Bold(text) => {
                if !in_bold {
                    current_paragraph.push(AstNode::Bold(text.clone()));
                    in_bold = true;
                } else {
                    current_paragraph.push(AstNode::Text(text.clone()));
                    in_bold = false;
                }
            }
            Token::Italic(text) => {
                if !in_italic {
                    current_paragraph.push(AstNode::Italic(text.clone()));
                    in_italic = true;
                } else {
                    current_paragraph.push(AstNode::Text(text.clone()));
                    in_italic = false;
                }
            }
            Token::Text(text) => {
                if !in_bold && !in_italic {
                    current_paragraph.push(AstNode::Text(text.clone()));
                } else {
                    let mut inner_paragraph = Vec::new();
                    inner_paragraph.push(AstNode::Text(text.clone()));
                    if in_bold {
                        inner_paragraph.push(AstNode::Bold(text.clone()));
                    }
                    if in_italic {
                        inner_paragraph.push(AstNode::Italic(text.clone()));
                    }
                    in_bold = false;
                    in_italic = false;
                }
            }
        }
    }

    if !current_paragraph.is_empty() {
        result.push(AstNode::Paragraph(current_paragraph.clone()));
        current_paragraph.clear();
    }

    result
}
