use clap::{App, Arg, SubCommand};
use quick_js::{Context, JsValue};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(PartialEq)]
enum Token {
    Tag(String),
    Hash,
    OpenBracket,
    CloseBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Import {
        from: String,
        element: String,
        name: String,
    },
    Function(String),
    Attribute(String),
    Value(String),
    Content(String),
}

#[derive(Debug)]
enum JsExecutionError {
    ContextError(quick_js::ContextError),
    ExecutionError(quick_js::ExecutionError),
    UnsupportedReturnType,
}

impl From<quick_js::ContextError> for JsExecutionError {
    fn from(error: quick_js::ContextError) -> Self {
        JsExecutionError::ContextError(error)
    }
}

impl From<quick_js::ExecutionError> for JsExecutionError {
    fn from(error: quick_js::ExecutionError) -> Self {
        JsExecutionError::ExecutionError(error)
    }
}

fn execute_js(js_code: &str) -> Result<String, JsExecutionError> {
    let context = Context::new().map_err(JsExecutionError::from)?;
    let js_codes = format!("let kitten = function(){}; kitten();", js_code);
    let result = context.eval(&js_codes).map_err(JsExecutionError::from)?;
    let stringified_result = match result {
        JsValue::String(s) => s,
        JsValue::Int(i) => i.to_string(),
        JsValue::Float(f) => f.to_string(),
        _ => return Err(JsExecutionError::UnsupportedReturnType),
    };
    Ok(stringified_result)
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
            'i' if chars.clone().collect::<String>().starts_with("import[") => {
                let mut import_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c != ']' {
                        import_str.push(c);
                        chars.next();
                    } else {
                        chars.next(); // consume ']'
                        break;
                    }
                }
                let import_parts: Vec<&str> = import_str.split_whitespace().collect();
                let from_parts: Vec<&str> = import_parts[0].split(':').collect();
                let from = from_parts[1].to_string();
                let element_parts: Vec<&str> = import_parts[1].split(':').collect();
                let element = element_parts[1].to_string();
                let name_parts: Vec<&str> = import_parts[2].split(':').collect();
                let name = name_parts[1].to_string();
                tokens.push(Token::Import {
                    from,
                    element,
                    name,
                });
            }
            '(' if chars.clone().collect::<String>().starts_with("()") => {
                let mut function_str = String::from("()");
                while let Some(&c) = chars.peek() {
                    if c != '}' {
                        function_str.push(c);
                        chars.next();
                    } else {
                        function_str.push(c);
                        chars.next(); // consume '}'
                        break;
                    }
                }
                tokens.push(Token::Function(function_str));
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
                    if c != ';' {
                        value.push(c);
                        chars.next();
                    } else {
                        chars.next(); // consume ';'
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
    let mut imports: HashMap<String, String> = HashMap::new();
    let self_closing_tags = vec![
        "img", "br", "hr", "input", "meta", "area", "base", "col", "embed", "link", "meta",
        "param", "source", "track", "wbr",
    ];
    while let Some(token) = iter.next() {
        match token {
            Token::Tag(tag) => {
                if let Some(imported_html) = imports.get(&tag.clone()) {
                    output.push_str(imported_html);
                } else {
                    tag_stack.push(tag.clone());
                    output.push_str(&format!("<{}", tag));
                }
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
            Token::Function(function) => {
                let result = execute_js(&function.trim_start_matches("()"));
                match result {
                    Ok(value) => output.push_str(&format!("{:?}", value)),
                    Err(e) => eprintln!("Error executing JS code: {:?}", e),
                }
            }
            Token::Import {
                from,
                element: _,
                name,
            } => {
                let path = std::env::current_dir().unwrap();
                let filename = if from.starts_with("/") {
                    from.strip_prefix("/").unwrap()
                } else {
                    &from
                };
                let input = fs::read_to_string(path.join(format!("./{}.kitten", filename)))
                    .expect("Could not read file");
                let output = match generate_html(&input) {
                    Ok(html) => html,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        return Err(e);
                    }
                };
                imports.insert(name.clone(), output.clone()); // Insert the output into the HashMap
            }
            Token::CloseBracket => {
                if let Some(tag) = tag_stack.last() {
                    if iter.peek() == Some(&Token::OpenCurlyBracket)
                        || matches!(iter.peek(), Some(Token::Attribute(_)))
                    {
                        output.push('>');
                    } else if self_closing_tags.contains(&tag.as_str()) {
                        output.push_str(" />");
                        let _ = tag_stack.pop(); // Remove self-closing tag from stack
                    } else {
                        output.push('>');
                    }
                }
            }
            Token::OpenCurlyBracket => {
                if let Some(&Token::Content(_)) = iter.peek() {
                    if let Token::Content(content) = iter.next().unwrap() {
                        output.push_str(&content);
                    }
                }
            }
            Token::CloseCurlyBracket => {
                if let Some(tag) = tag_stack.pop() {
                    output.push_str(&format!("</{}>", tag));
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
    if args.len() < 2 {
        eprintln!("Usage: kitten <command> [<args>]");
        return;
    }
    let path = std::env::current_dir().unwrap();
    let matches = App::new("Kitten")
        .version("0.2.0")
        .author("418e github.com/418e")
        .about("Kitten is an HTML template compiler")
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a .kitten file")
                .arg(
                    Arg::with_name("filename")
                        .help("The .kitten file to run")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(SubCommand::with_name("version").about("Displays the version of Kitten"))
        .subcommand(SubCommand::with_name("update").about("Updates Kitten to the latest version"))
        .subcommand(
            SubCommand::with_name("new")
                .about("Initializes a new project")
                .arg(
                    Arg::with_name("name")
                        .help("The name of the new project")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(_filename) = matches.value_of("filename") {
            let filename = &args[2];
            let input = if filename.ends_with(".kitten") {
                fs::read_to_string(path.join(filename)).expect("Could not read file")
            } else {
                fs::read_to_string(path.join(format!("{}.kitten", filename)))
                    .expect("Could not read file")
            };
            let output = match generate_html(&input) {
                Ok(html) => html,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            fs::write(path.join(format!("{}.html", filename)), output)
                .expect("Could not write file");
        }
    } else if matches.subcommand_matches("version").is_some() {
        println!("\nv0.2.0")
    } else if matches.subcommand_matches("update").is_some() {
        println!("\nDownloading latest version...");
        std::process::Command::new("curl")
            .arg("-o")
            .arg("kitten")
            .arg("https://kitten.tronlang.org/v/latest")
            .output()
            .expect("Failed to execute command");
        std::process::Command::new("sudo")
            .arg("mv")
            .arg("kitten")
            .arg("/usr/local/bin/")
            .output()
            .expect("Failed to execute command");
        std::process::Command::new("sudo")
            .arg("chmod")
            .arg("+x")
            .arg("/usr/local/bin/kitten")
            .output()
            .expect("Failed to execute command");

        println!("\nLatest version installed successfully")
    } else if let Some(matches) = matches.subcommand_matches("new") {
        if let Some(_name) = matches.value_of("name") {
            let dir_name = &args[2];
            std::fs::create_dir(dir_name).expect("Could not create directory");
            let mut file = std::fs::File::create(format!("{}/index.kitten", dir_name))
                .expect("Could not create file");
            file.write_all(b"div[]{\"hello world\"}")
                .expect("Could not write to file");
        }
    }
}
