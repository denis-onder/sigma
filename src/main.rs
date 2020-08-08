extern crate handlebars;
extern crate unzip;

mod markdown;

// use handlebars::Handlebars;
use std::fs::File;
use unzip::Unzipper;

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
fn unzip() {
    let file = File::open("archive.zip");

    match file {
        Ok(f) => {
            let unzip_result = Unzipper::new(f, "./test").unzip();

            match unzip_result {
                Ok(_) => println!("Unzipped"),
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    }
}

fn main() {
    println!("Sigma");

    unzip();

    let posts = markdown::read_markdown_files("./test");

    for post in posts {
        let headers = markdown::parse_post_headers(post);

        println!("{:?}", headers);
    }
}
