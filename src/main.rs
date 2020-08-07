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
fn parse_html(stringified_file: String) {
    let parsing_regex = Regex::new(r"#output").unwrap();

    if parsing_regex.is_match(&stringified_file) {
        let split = parsing_regex.split(&stringified_file);

        if split.count() >= 2 {
            // Return split
        } else {
            // Handle error
            println!("Not a valid template.");
        }
    } else {
        println!("not found");
    }
}

// 3. Read JSON file data

fn main() -> Result<(), Error> {
    let mut html_file = read_file("index.html")?;
    let mut html_stringified = String::new();

    html_file.read_to_string(&mut html_stringified)?;

    parse_html(html_stringified);

    Ok(())
}
