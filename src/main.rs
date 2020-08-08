extern crate handlebars;
extern crate unzip;
extern crate uuid;

mod markdown;

use std::borrow::Cow;
use std::fs::{remove_dir_all, File};
use unzip::Unzipper;
use uuid::Uuid;

/**
 * CONCEPT:
 * 1. User sends a zipped project with:
 *    1.1. Assets in assets folder (CSS, JS, etc),
 *    1.2. Posts in posts folder (Markdown),
 *    1.3. Templates in the templates folder (Handlebars)
 *
 * 2. Unzip the project
 * 3. Parse Markdown posts
 * 4. Generate HTML from those posts using the Handlebars engine
 * 5. Remove the project directory and the zip file
 */

// 1. Unzip project
fn unzip() -> String {
    // Generate UUID to be used as a temporary folder name
    let folder_name = Uuid::new_v4().to_string();
    let file = File::open("archive.zip");

    match file {
        Ok(f) => {
            // Build a string pointing to the new folder
            let mut path = String::new();
            path.push_str(&folder_name);

            let unzip_result = Unzipper::new(f, path).unzip();

            match unzip_result {
                Ok(_) => println!("Unzipped"),
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    }

    folder_name
}

fn main() {
    // Unzip the archive, returning the output folder name
    let folder_name = unzip();

    let posts = markdown::read_markdown_files(&folder_name);

    for p in posts {
        let post = markdown::parse_post(p);

        println!("{:?}", post.headers.author);
    }

    // Finally, remove the temp directory
    remove_dir_all(folder_name);
}
