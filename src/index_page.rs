#![allow(dead_code)]
/**
 * This module is responsible for generating the index.html page.
 * The page should contain links to generated posts
 */

#[path = "markdown.rs"]
mod markdown;

use markdown::Post;
use std::path::PathBuf;

pub struct IndexPage {
  posts: Vec<Post>,
}

pub fn read_index_page(base_folder_path: &String) {
  let mut file_path = PathBuf::new();
  file_path.push(base_folder_path);
  file_path.push("index");
  file_path.set_extension("hbs");

  println!("{:?}", file_path);
}
