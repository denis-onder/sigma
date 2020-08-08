use std::fs::read_dir;

// Post structure
struct Post {
  pub template: String,
  pub author: String,
  pub date: String,
  pub content: String,
}

pub fn read_markdown_files(path: &str) {
  let mut path = path.to_owned();
  path.push_str("/posts");

  let files = read_dir(&path).unwrap();

  for f in files {
    println!("{:?}", f.unwrap().file_name());
  }
}
