extern crate cfg_if;
extern crate wasm_bindgen;

use pulldown_cmark::{html, Options, Parser};

mod utils;

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

#[wasm_bindgen]
pub fn pulldown_cmark(source_text: &str) -> String {
    let markdown_input = source_text;

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum Token {
    // Heading(HeadingLevel, Option<&'a str>, Vec<&'a str>),
    Heading(String),
    Bold(String),
    Italic(String),
    Text(String),
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut in_bold = false;
    let mut in_italic = false;

    for line in input.lines() {
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match (c, in_bold, in_italic) {
                ('#', false, false) => {
                    // Heading
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    let mut level = 1;
                    while chars.peek() == Some(&'#') {
                        chars.next();
                        level += 1;
                    }
                    tokens.push(Token::Heading(chars.collect()));
                    break;
                }
                ('*', false, false) => {
                    // Bold
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    in_bold = true;
                }
                ('*', true, false) => {
                    // End of Bold
                    tokens.push(Token::Bold(buffer.clone()));
                    buffer.clear();
                    in_bold = false;
                }
                ('_', false, false) => {
                    // Italic
                    if !buffer.is_empty() {
                        tokens.push(Token::Text(buffer.clone()));
                        buffer.clear();
                    }
                    in_italic = true;
                }
                ('_', false, true) => {
                    // End of Italic
                    tokens.push(Token::Italic(buffer.clone()));
                    buffer.clear();
                    in_italic = false;
                }
                _ => {
                    // Text
                    buffer.push(c);
                }
            }
        }
        if !buffer.is_empty() {
            tokens.push(Token::Text(buffer.clone()));
            buffer.clear();
        }
        tokens.push(Token::Text("\n".to_string()));
    }

    tokens.pop(); // Remove last newline

    tokens
}

// #[wasm_bindgen]
// pub fn text_to_token(source_text: &str) -> String {}
#[test]
fn test_lex() {
    let input = "##Heading 2\n\nMore *bold* and _italic_ text.";
    let expected_output = vec![
        Token::Heading("Heading 2".to_string()),
        Token::Text("\n".to_string()),
        Token::Text("\n".to_string()),
        Token::Text("More ".to_string()),
        Token::Bold("bold".to_string()),
        Token::Text(" and ".to_string()),
        Token::Italic("italic".to_string()),
        Token::Text(" text.".to_string()),
    ];

    assert_eq!(lex(input), expected_output);
}
