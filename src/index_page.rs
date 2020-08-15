/**
 * This module is responsible for generating the index.html page.
 * The page should contain links to generated posts
 */

mod markdown;

use markdown::Post;

pub struct IndexPage {
  posts: Vec<Post>;
}