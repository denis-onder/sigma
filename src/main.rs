use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

// 1. Read a template HTML file
fn read_file(path: &str) -> Result<File, Error> {
    let file = File::open(path)?;
    return Ok(file);
}

// 2. Parse the file
fn parse_html(stringified_file: String) -> Result<Vec<String>, String> {
    let parsing_regex = Regex::new(r"#output").unwrap();

    if parsing_regex.is_match(&stringified_file) {
        let split = parsing_regex.split(&stringified_file);
        let res: Vec<String> = split.map(|x| x.to_string()).collect();

        if res.len() >= 2 {
            return Ok(res);
        } else {
            return Err("Invalid template.".to_string());
        }
    }

    Err("Error".to_string())
}

// 3. Read JSON file data

fn main() -> Result<(), Error> {
    let mut html_file = read_file("index.html")?;
    let mut html_stringified = String::new();

    html_file.read_to_string(&mut html_stringified)?;

    let res = parse_html(html_stringified);

    match res {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    Ok(())
}
