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
pub struct Post {
  pub headers: Headers,
  pub content: String,
}

pub fn read_markdown_files(path: &str) -> Vec<PathBuf> {
  let mut path = path.to_owned();
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

pub fn parse_post_headers(path_to_file: PathBuf) -> Headers {
  let mut post = File::open(path_to_file).unwrap();
  let mut post_string = String::new();

  post.read_to_string(&mut post_string).unwrap();

  let partials: Vec<&str> = post_string.split("\n").collect();
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

  // Return headers
  return Headers {
    template,
    author,
    date,
  };
}
