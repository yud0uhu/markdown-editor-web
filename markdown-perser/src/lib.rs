extern crate cfg_if;
extern crate wasm_bindgen;

// use pulldown_cmark::{html, Options, Parser};

mod html;
mod lex;
mod parse;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

use std::iter::Peekable;
use std::vec::IntoIter;

pub type PeekableIter<T> = Peekable<IntoIter<T>>;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello,{}!", name));
}

// #[wasm_bindgen]
// pub fn pulldown_cmark(source_text: &str) -> String {
//     let markdown_input = source_text;

//     let mut options = Options::empty();
//     options.insert(Options::ENABLE_STRIKETHROUGH);
//     let parser = Parser::new_ext(markdown_input, options);

//     let mut html_output = String::new();
//     html::push_html(&mut html_output, parser);
//     html_output
// }

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Token {
    Heading(HeadingLevel, String),
    Bold(String),
    Italic(String),
    Text(String),
    BlockQuotes(String),
    Lists(String),
}

#[wasm_bindgen]
pub fn text_to_token(input_text: &str) -> String {
    let tokens = lex::lex(input_text);
    let ast = parse::parse(&tokens);
    html::generate_html(&ast)
}

#[test]
fn test_lex() {
    let input = "## Heading 2\n\n> This is a blockquote.\n\nMore **bold** and __italic__ text.";
    let expected_output = vec![
        Token::Heading(HeadingLevel::H2, "Heading 2".to_string()),
        Token::BlockQuotes("This is a blockquote.".to_string()),
        Token::Text("More ".to_string()),
        Token::Bold("bold".to_string()),
        Token::Text(" and ".to_string()),
        Token::Italic("italic".to_string()),
        Token::Text(" text.".to_string()),
    ];

    assert_eq!(lex::lex(input), expected_output);
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Heading(HeadingLevel, String),
    Bold(String),
    Italic(String),
    Text(String),
    BlockQuotes(String),
    Lists(String),
    Paragraph(Vec<AstNode>),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[test]
fn test_lex_and_parse() {
    let input = "\
# Hello, world!\n
> This is a blockquote\n
This is a **markdown** __parser__.";
    let expected_output = vec![
        AstNode::Heading(HeadingLevel::H1, "Hello, world!".to_string()),
        AstNode::BlockQuotes("This is a blockquote".to_string()),
        AstNode::Paragraph(vec![
            AstNode::Text("This is a ".to_string()),
            AstNode::Bold("markdown".to_string()),
            AstNode::Italic("parser".to_string()),
        ]),
    ];
    let tokens = lex::lex(input);
    let output = parse::parse(&tokens);
    assert_eq!(output, expected_output);
}
