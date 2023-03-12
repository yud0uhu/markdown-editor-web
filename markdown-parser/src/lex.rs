                    let mut list_buffer = String::new();
                    loop {
                        match chars.next() {
                            Some('\n') => {
                                tokens.push(Token::Lists(list_buffer.clone()));
                                list_buffer.clear();
                            }
                            Some(c) => {
                                list_buffer.push(c);
                            }
                            None => {
                                tokens.push(Token::Lists(list_buffer.clone()));
                                break;
                            }
                        }
                    }
- List1\n
- List2\n
        Token::Text(" text.".to_string()),
        Token::Lists("List1".to_string()),
        Token::Lists("List2".to_string()),
