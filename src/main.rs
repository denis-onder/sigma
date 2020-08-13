extern crate unzip;
extern crate uuid;

mod markdown;

// use std::fs::remove_dir_all;
use markdown::{generate_post_page, parse_post, read_markdown_files, FolderPaths};
use std::env;
use std::fs::{create_dir, File};
use std::path::PathBuf;
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
 * 5. Zip the generated project and serve it to the user
 * 6. Remove the project directory and the source zip file
 */

// 1. Unzip project
fn unzip(zip_archive_name: &String) -> FolderPaths {
    // Generate UUID to be used as a temporary folder name
    let folder_name = Uuid::new_v4().to_string();
    let file = File::open(zip_archive_name);

    // Build a string pointing to the new folder
    let mut path = PathBuf::new();

    match file {
        Ok(f) => {
            path.push(&folder_name);

            let unzip_result = Unzipper::new(f, &path).unzip();

            match unzip_result {
                Ok(_) => {
                    println!("Unzipped {}", zip_archive_name);
                    path.push("build");
                    create_dir(&path).unwrap();
                }
                Err(e) => panic!("{}", e),
            }
        }
        Err(e) => panic!("{}", e),
    }

    FolderPaths {
        src: folder_name,
        build: path,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut zip_archive_name: String = args.last().unwrap().to_string();

    if !zip_archive_name.contains(".zip") {
        zip_archive_name = "example.zip".to_owned();
    }
    // Unzip the archive, returning the output folder name
    let folder_names = unzip(&zip_archive_name);

    let posts = read_markdown_files(&folder_names.src);

    for p in posts {
        println!(
            "Parsing and generating page for: {:?}",
            p.file_name().unwrap()
        );
        let post = parse_post(&p);

        generate_post_page(&folder_names, post);
    }

    // Finally, remove the temp directory
    // remove_dir_all(folder_name); // Re-enable this for production after the user has been served
}
