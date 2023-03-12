            AstNode::Lists(text) => {
                if !is_in_list {
                    result.push_str("<ul>");
                    is_in_list = true;
                }
                result.push_str(&format!("<li>{}</li>", text));
            }
