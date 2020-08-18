#![allow(dead_code)]
/**
 * This module is responsible for generating the index.html page.
 * The page should contain links to generated posts
 */

#[path = "markdown.rs"]
mod markdown;

use handlebars::Handlebars;
use markdown::Post;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct IndexPage {
  posts: Vec<Post>,
}

pub fn generate_index_page(posts: Vec<Post>, base_folder_path: &String) {
  let reg = Handlebars::new();

  let mut file_path = PathBuf::new();
  file_path.push(base_folder_path);
  file_path.push("index");

  let mut output_file_path = PathBuf::from(&file_path);

  file_path.set_extension("hbs");
  output_file_path.set_extension("html");

  let output_file = File::create(output_file_path).unwrap();

  let mut template = File::open(file_path).unwrap();
  let mut template_string = String::new();
  template.read_to_string(&mut template_string).unwrap();

  let data = IndexPage { posts: posts };

  reg
    .render_template_to_write(&template_string, &data, output_file)
    .unwrap();
}
