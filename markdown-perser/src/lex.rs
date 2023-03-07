use crate::{HeadingLevel, Token};

pub fn lex(input: &str) -> Vec<Token> {
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
                    // Skip whitespace
                    while let Some(' ') = chars.peek() {
                        chars.next();
                    }

                    tokens.push(Token::Heading(
                        match level {
                            1 => HeadingLevel::H1,
                            2 => HeadingLevel::H2,
                            3 => HeadingLevel::H3,
                            4 => HeadingLevel::H4,
                            5 => HeadingLevel::H5,
                            _ => HeadingLevel::H6,
                        },
                        chars.collect(),
                    ));

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

        if let Some(Token::Text(last)) = tokens.last() {
            if last == "\n" {
                tokens.pop();
            }
        }
    }

    tokens
}
