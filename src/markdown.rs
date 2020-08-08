extern crate handlebars;
extern crate serde;

use handlebars::Handlebars;
use serde::Serialize;
use std::ffi::OsString;
use std::fs::{read_dir, File};
use std::io::{Read, Write};
use std::path::PathBuf;

// Post structure
#[derive(Serialize)]
pub struct Post {
  pub file_name: OsString,
  pub template: String,
  pub author: String,
  pub date: String,
  pub title: String,
  pub content: String,
}

pub fn read_markdown_files(path: &String) -> Vec<PathBuf> {
  let mut path = path.clone();
  path.push_str("/posts");

  let files = read_dir(path).unwrap();

  let mut result: Vec<PathBuf> = Vec::new();

  for f in files {
    result.push(f.unwrap().path());
  }

  result
}

fn parse_header(string: &str, header_name: &str) -> String {
  string.split(header_name).last().unwrap().replace("\r", "")
}

pub fn parse_post(path_to_file: &PathBuf) -> Post {
  let mut post = File::open(path_to_file).unwrap();
  let mut content = String::new();

  let file_name = path_to_file.file_name().unwrap().to_owned();

  post.read_to_string(&mut content).unwrap();

  let partials: Vec<&str> = content.split("\n").collect();
  // Placeholder variables for headers
  let mut template: String = String::new();
  let mut author: String = String::new();
  let mut date: String = String::new();
  let mut title: String = String::new();

  for i in partials.iter() {
    if i.contains("template") {
      template = parse_header(i, "template: ");
    }

    if i.contains("author") {
      author = parse_header(i, "author: ");
    }

    if i.contains("date") {
      date = parse_header(i, "date: ");
    }

    if i.contains("title") {
      title = parse_header(i, "title: ");
    }
  }

  // Return post
  Post {
    file_name,
    template,
    author,
    date,
    title,
    content,
  }
}

pub fn generate_post_page(base_folder_name: &String, post: Post) {
  let reg = Handlebars::new();

  // Generate path to template
  let mut path = PathBuf::new();
  path.push(base_folder_name);
  path.push("templates");
  path.push(&post.template);
  path.set_extension("hbs");

  // TODO: Implement a check if the template file exists prior to attempting templating
  let mut template = File::open(path).unwrap();
  let mut template_string = String::new();

  let mut output_path = PathBuf::new();
  output_path.push(base_folder_name);
  output_path.push(&post.file_name);
  output_path.set_extension("html");

  template.read_to_string(&mut template_string).unwrap();
  let result = reg.render_template(&template_string, &post).unwrap();

  let mut output_file = File::create(output_path).unwrap();

  output_file.write(result.as_bytes());
}
