use regex::Regex;
use std::fs;
use std::io::prelude::*;

fn compile(contents: &str) -> String {
    let re = Regex::new(r"(\w+)\[([^\]]+)\]").unwrap();
    let re2 = Regex::new(r"(\w+)\[]").unwrap();
    let mut result = String::new();
    let mut level = 0;
    let mut tag = String::new();
    let mut attrs = String::new();
    let mut content = String::new();
    let mut is_tag = false;

    for c in contents.chars() {
        match c {
            '{' => {
                level += 1;
                is_tag = true;
            }
            '}' => {
                level -= 1;
                if level == 0 {
                    let nested_content = compile(&content);
                    result.push_str(&format!("<{}{}>{}</{}>", tag, attrs, nested_content, tag));
                    tag.clear();
                    attrs.clear();
                    content.clear();
                    is_tag = false;
                }
            }
            _ => {
                if level == 0 {
                    if let Some(caps) = re.captures(&format!("{}{}", tag, c)) {
                        tag = caps[1].to_string();
                        attrs = caps[2].to_string();
                        let attr_parts: Vec<&str> = attrs.split(" ").collect();
                        let mut attr_str = String::new();
                        for attr in attr_parts {
                            let parts: Vec<&str> = attr.split(":").collect();
                            let attr_name = parts[0];
                            let attr_value = parts[1];
                            attr_str.push_str(&format!(" {}=\"{}\"", attr_name, attr_value));
                        }
                        attrs = attr_str;
                    } else if let Some(caps) = re2.captures(&format!("{}", tag)) {
                        tag = caps[1].to_string();
                        attrs = "".to_string();
                    } else {
                        tag.push(c);
                    }
                } else if level == 1 && is_tag {
                    content.push(c);
                }
            }
        }
    }

    if level == 0 && !tag.is_empty() {
        if tag.ends_with(".") {
            tag.pop();
            result.push_str(&format!("{}", tag));
        } else if tag.trim().len() < 1 {
            result.push_str(" ");
        } else {
            result.push_str(&format!("<{}{}>{}</{}>", tag, attrs, content, tag));
        }
    }

    result
}

fn generate(content: &str) -> std::io::Result<()> {
    let mut file = fs::File::create("test/index.html")?;
    let contents = content.to_string();
    writeln!(&mut file, "{}", &contents.to_string())?;
    Ok(())
}

fn run_string(contents: &str) -> Result<(), String> {
    let out = compile(contents);
    let _gen = generate(&out);
    return Ok(());
}
fn main() {
    let _ = match fs::read_to_string("test/index.kitten") {
        Err(msg) => Err(msg.to_string()),
        Ok(contents) => run_string(&contents),
    };
}
