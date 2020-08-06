use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

// 1. Read a template HTML file
fn read_html_template() -> Result<File, Error> {
    let file = File::open("index.html")?;
    return Ok(file);
}

// 2. Parse the file
fn parse_html(stringified_file: String) {
    let parsing_regex = Regex::new(r"#output").unwrap();

    if parsing_regex.is_match(&stringified_file) {
        println!("found");
    } else {
        println!("not found");
    }
}

fn main() -> Result<(), Error> {
    let mut file = read_html_template()?;
    let mut output = String::new();

    file.read_to_string(&mut output)?;

    parse_html(output);

    Ok(())
}
