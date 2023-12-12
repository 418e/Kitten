use std::fs;

#[derive(PartialEq)]
enum Token {
    Tag(String),
    Hash,
    OpenBracket,
    CloseBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Attribute(String),
    Value(String),
    Content(String),
}

fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '#' => {
                while let Some(&c) = chars.peek() {
                    if c != '\n' {
                        tokens.push(Token::Hash);
                        chars.next();
                    } else {
                        break;
                    }
                }
            }
            '[' => {
                tokens.push(Token::OpenBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::CloseBracket);
                chars.next();
            }
            '{' => {
                tokens.push(Token::OpenCurlyBracket);
                chars.next();
                while let Some(&c) = chars.peek() {
                    if c == ' ' || c == '\n' || c == '\t' {
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        let mut content = String::new();
                        while let Some(&c) = chars.peek() {
                            if c != '"' {
                                content.push(c);
                                chars.next();
                            } else {
                                chars.next(); // consume closing "
                                break;
                            }
                        }
                        tokens.push(Token::Content(content.trim().to_string()));
                        // trim the content
                    }
                }
            }
            '}' => {
                tokens.push(Token::CloseCurlyBracket);
                chars.next();
            }
            ':' => {
                chars.next();
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    if c != ' ' && c != ']' {
                        value.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Value(value));
            }
            ' ' | '\n' | '\t' => {
                chars.next();
            }
            _ => {
                let mut text = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '-' {
                        text.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !text.is_empty() {
                    if chars.peek() == Some(&':') {
                        tokens.push(Token::Attribute(text));
                    } else {
                        tokens.push(Token::Tag(text));
                    }
                }
            }
        }
    }

    tokens
}

fn parser(tokens: Vec<Token>) -> Result<String, String> {
    let mut output = String::new();
    let mut iter = tokens.into_iter().peekable();
    let mut tag_stack = Vec::new();

    while let Some(token) = iter.next() {
        match token {
            Token::Hash => {
                continue;
            }
            Token::OpenCurlyBracket => {
                output.push('>');
                if let Some(&Token::Content(_)) = iter.peek() {
                    if let Token::Content(content) = iter.next().unwrap() {
                        output.push_str(&content);
                    }
                }
            }
            Token::Tag(tag) => {
                tag_stack.push(tag.clone());
                output.push_str(&format!("<{}", tag));
            }
            Token::OpenBracket => {
                let mut attributes = String::new();
                while let Some(&Token::Attribute(_)) = iter.peek() {
                    if let Token::Attribute(attr) = iter.next().unwrap() {
                        attributes.push_str(&format!(" {}=\"", attr));
                    }
                    if let Token::Value(val) = iter.next().unwrap() {
                        attributes.push_str(&format!("{}\"", val));
                    }
                }
                output.push_str(&attributes);
            }
            Token::CloseBracket => {
                if let Some(tag) = tag_stack.pop() {
                    if iter.peek() == Some(&Token::OpenCurlyBracket) || matches!(iter.peek(), Some(Token::Attribute(_))) {
                        output.push_str(&format!("</{}>", tag));
                    } else {
                        output.push_str(" />");
                    }
                }
            }
            _ => {}
        }
    }

    if !tag_stack.is_empty() {
        return Err("Mismatched tags".to_string());
    }

    Ok(output)
}

fn generate_html(input: &str) -> Result<String, String> {
    let tokens = lexer(input);
    parser(tokens)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: kitten run <filename>");
        return;
    }
    let command = &args[1];
    let filename = &args[2];
    if command != "run" {
        eprintln!("Unknown command: {}", command);
        return;
    }
    let path = std::env::current_dir().unwrap();
    let input =
        fs::read_to_string(path.join(format!("{}.kitten", filename))).expect("Could not read file");
    let output = match generate_html(&input) {
        Ok(html) => html,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    fs::write(path.join(format!("{}.html", filename)), output).expect("Could not write file");
}
