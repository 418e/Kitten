use regex::Regex;
use std::fs;
use std::io::prelude::*;

fn compile(contents: &str) -> String {
    let re = Regex::new(r"(\w+)\[([^\]]+)\]\{([^}]+)}").unwrap();
    let caps = re.captures(contents).unwrap();
    let tag_name = &caps[1];
    let attrs = &caps[2];
    let content = &caps[3];
    let attrs: Vec<&str> = attrs.split(" ").collect();
    let mut attr_str = String::new();
    for attr in attrs {
        let parts: Vec<&str> = attr.split(":").collect();
        let attr_name = parts[0];
        let attr_value = parts[1];
        attr_str.push_str(&format!(" {}=\"{}\"", attr_name, attr_value));
    }
    format!("<{}{}>{}</{}>", tag_name, attr_str, content, tag_name)
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
