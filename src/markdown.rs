use handlebars::Handlebars;
use std::fs::{read_dir, File};
use std::io::Read;
use std::path::PathBuf;

// Headers structure
#[derive(Debug)]
pub struct Headers {
  pub template: String,
  pub author: String,
  pub date: String,
}

// Post structure
#[derive(Debug)]
pub struct Post {
  pub headers: Headers,
  pub content: String,
}

pub fn read_markdown_files(path: &String) -> Vec<PathBuf> {
  let mut path = path.clone();
  path.push_str("/posts");

  let files = read_dir(&path).unwrap();

  let mut result: Vec<PathBuf> = Vec::new();

  for f in files {
    result.push(f.unwrap().path());
  }

  result
}

fn parse_header(string: &str, header_name: &str) -> String {
  string.split(header_name).last().unwrap().replace("\r", "")
}

pub fn parse_post(path_to_file: PathBuf) -> Post {
  let mut post = File::open(path_to_file).unwrap();
  let mut content = String::new();

  post.read_to_string(&mut content).unwrap();

  let partials: Vec<&str> = content.split("\n").collect();
  // Placeholder variables for headers
  let mut template: String = String::new();
  let mut author: String = String::new();
  let mut date: String = String::new();

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
  }

  // Return post
  Post {
    headers: Headers {
      template,
      author,
      date,
    },
    content,
  }
}

pub fn generate_post_page(post: Post) {
  let reg = Handlebars::new();

  // Read template file
}
