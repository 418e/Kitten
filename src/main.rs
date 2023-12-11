use std::fs;

enum Token {
    Tag(String),
    OpenBracket,
    CloseBracket,
    Attribute(String),
    Value(String),
    Content(String),
}

fn lexer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '[' => {
                tokens.push(Token::OpenBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::CloseBracket);
                chars.next();
            }
            '{' => {
                chars.next();
                let mut content = String::new();
                while let Some(&c) = chars.peek() {
                    if c != '}' {
                        content.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Content(content));
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

fn parser(tokens: Vec<Token>) -> String {
    let mut output = String::new();
    let mut iter = tokens.into_iter().peekable();
    let mut current_tag = String::new();

    while let Some(token) = iter.next() {
        match token {
            Token::Tag(tag) => {
                current_tag = tag.clone();
                output.push_str(&format!("<{}", tag));
            }
            Token::OpenBracket => {
                while let Some(&Token::Attribute(_)) = iter.peek() {
                    if let Token::Attribute(attr) = iter.next().unwrap() {
                        output.push_str(&format!(" {}=\"", attr));
                    }
                    if let Token::Value(val) = iter.next().unwrap() {
                        output.push_str(&format!("{}\"", val));
                    }
                }
                output.push('>');
            }
            Token::Content(content) => {
                output.push_str(&content);
                output.push_str(&format!("</{}>", current_tag));
                current_tag.clear();
            }
            Token::CloseBracket => {
                if current_tag.is_empty() {
                    if let Some(&Token::Tag(ref tag)) = iter.peek() {
                        output.push_str(&format!("<{}>", tag));
                        iter.next();
                    }
                }
            }
            _ => {}
        }
    }

    output
}

fn generate_html(input: &str) -> String {
    let tokens = lexer(input);
    parser(tokens)
}

fn main() {
    let input = fs::read_to_string("test/index.kitten").expect("Could not read file");
    let output = generate_html(&input);
    fs::write("test/index.html", output).expect("Could not write file");
}