use handlebars::Handlebars;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

extern crate handlebars;
#[macro_use]
extern crate serde_json;

/**
 * CONCEPT:
 * 1. User sends a zipped project with the following structure:
 *    assets
 */

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
fn parse_json(json_string: String) -> Result<Value, String> {}

fn main() -> Result<(), Error> {
    let mut html_file = read_file("index.html")?;
    let mut html_stringified = String::new();
    html_file.read_to_string(&mut html_stringified)?;

    let res = parse_html(html_stringified);

    match res {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{}", e),
    }

    let mut json_data = read_file("mock_data.json")?;
    let mut json_stringified = String::new();
    json_data.read_to_string(&mut json_stringified)?;

    println!("{}", json_stringified);

    // let res = parse_json(json_stringified);

    Ok(())
}
