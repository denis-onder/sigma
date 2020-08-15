use comrak::{markdown_to_html, ComrakOptions};
use handlebars::Handlebars;
use serde::Serialize;
use std::ffi::OsString;
use std::fs::{create_dir, metadata, read_dir, File};
use std::io::Read;
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

pub struct FolderPaths {
  pub src: String,
  pub build: PathBuf,
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
  let mut post_content = String::new();

  let file_name = path_to_file.file_name().unwrap().to_owned();

  post.read_to_string(&mut post_content).unwrap();

  let partials: Vec<&str> = post_content.split("\n").collect();
  // Placeholder variables for headers
  let mut template: String = String::new();
  let mut author: String = String::new();
  let mut date: String = String::new();
  let mut title: String = String::new();

  let mut headers_end_index: usize = 0;

  for (i, partial) in partials.iter().enumerate() {
    // Check for headers
    if partial.contains("template: ") {
      template = parse_header(partial, "template: ");
    }

    if partial.contains("author: ") {
      author = parse_header(partial, "author: ");
    }

    if partial.contains("date: ") {
      date = parse_header(partial, "date: ");
    }

    if partial.contains("title: ") {
      title = parse_header(partial, "title: ");
    }

    // Check if headers have ended
    if partial.trim() == "---" {
      headers_end_index = i;
    }
  }

  let (_, content) = partials.split_at(headers_end_index + 1);
  let content = markdown_to_html(&content.join("\n"), &ComrakOptions::default());

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

fn path_exists(path: &PathBuf) -> bool {
  metadata(path).is_ok()
}

pub fn generate_post_page(paths: &FolderPaths, post: &Post) {
  let reg = Handlebars::new();

  // Generate path to template
  let mut path = PathBuf::new();
  path.push(&paths.src);
  path.push("templates");
  path.push(&post.template);
  path.set_extension("hbs");

  // TODO: Implement a check if the template file exists prior to attempting templating
  let mut template = File::open(path).unwrap();
  let mut template_string = String::new();

  let mut output_path = PathBuf::new();
  output_path.push(&paths.build);
  output_path.push("posts");

  if !path_exists(&output_path) {
    create_dir(&output_path).unwrap();
  }

  output_path.push(&post.file_name);
  output_path.set_extension("html");

  template.read_to_string(&mut template_string).unwrap();
  reg
    .render_template_to_write(&template_string, &post, File::create(output_path).unwrap())
    .unwrap();
}
