mod markdown;

// use std::fs::remove_dir_all;
use handlebars::Handlebars;
use markdown::{generate_post_page, parse_post, read_markdown_files, FolderPaths, Post};
use sass_rs::{compile_file, Options};
use serde::Serialize;
use std::env;
use std::fs::{copy, create_dir, metadata, read_dir, File};
use std::io::prelude::*;
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
 * 4. Register all partials
 * 5. Generate the HTML from the templates and the posts using the Handlebars engine
 * 6. Compile the SCSS if there is any, and build out the projectž
 * 7. Zip the build and serve it to the user
 * 8. Remove the project directory and the source zip file
 */

#[derive(Serialize)]
struct IndexPage {
    posts: Vec<Post>,
}

fn generate_index_page(posts: Vec<Post>, base_folder_path: &String) {
    let reg = Handlebars::new();

    let mut file_path = PathBuf::new();
    file_path.push(base_folder_path);

    let mut output_file_path = PathBuf::from(&file_path);
    file_path.push("index");
    file_path.set_extension("hbs");

    output_file_path.push("build");
    output_file_path.push("index");
    output_file_path.set_extension("html");

    let output_file = File::create(output_file_path).unwrap();

    let mut template = File::open(file_path).unwrap();
    let mut template_string = String::new();
    template.read_to_string(&mut template_string).unwrap();

    let data = IndexPage { posts: posts };

    reg.render_template_to_write(&template_string, &data, output_file)
        .unwrap();
}

fn generate_extension(string: &String) -> String {
    let image_extensions = vec!["jpg", "jpeg", "png", "svg", "bmp", "gif"];
    let css_extensions = vec!["sass", "scss"];

    // If the file is an image, create a folder called `img`
    for x in image_extensions {
        if x.to_string() == string.to_owned() {
            return "img".to_string();
        }
    }

    // If the file is an stylesheet, create a folder called `css`
    for y in css_extensions {
        if y.to_string() == string.to_owned() {
            return "css".to_string();
        }
    }

    string.to_owned()
}

fn build_assets(path: &PathBuf, output_path: &PathBuf) {
    // Copy the contents of the assets folder to the build directory if the folder exists
    if metadata(PathBuf::from(&path)).is_ok() {
        let asset_folders = read_dir(&path).unwrap();

        for asset_folder in asset_folders {
            let path = &asset_folder.unwrap().path();

            // Read the directory
            let folder = read_dir(path).unwrap();

            for file in folder {
                let file = file.unwrap();
                let file_name = file.file_name().to_str().unwrap().to_string();
                let extension =
                    generate_extension(&file_name.split(".").last().unwrap().to_string());

                // Create an output folder for the files respective to their type
                let mut copy_path = PathBuf::from(&output_path);
                copy_path.push(extension);

                if !metadata(&copy_path).is_ok() {
                    create_dir(&copy_path).unwrap();
                }
                // Check if the file is a Sass/SCSS file
                if !file_name.contains("__")
                    && (file_name.contains(".scss") || file_name.contains(".sass"))
                {
                    let compiled = compile_file(file.path(), Options::default()).unwrap();

                    // Write the compiled Sass to a CSS file
                    let mut css_path = PathBuf::from(&copy_path);
                    css_path.push(&file_name);
                    css_path.set_extension("css");

                    let mut css_file = File::create(css_path).unwrap();
                    css_file.write(compiled.as_bytes()).unwrap();
                } else {
                    // Copy the file over to it's new directory
                    copy_path.push(&file_name);
                    copy(file.path(), copy_path).unwrap();
                }
            }
        }
    }
}

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

fn register_partials(base_path: &String) {
    // Create a path buffer to the partials folder path
    let mut partials_path = PathBuf::new();
    partials_path.push(base_path);
    partials_path.push("partials");

    println!("Partials path: {:?}", partials_path)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut zip_archive_name: String = args.last().unwrap().to_string();

    if !zip_archive_name.contains(".zip") {
        zip_archive_name = "example.zip".to_owned();
    }
    // Unzip the archive, returning the output folder name
    let folder_names = unzip(&zip_archive_name);

    let mut assets_path = PathBuf::new();
    assets_path.push(&folder_names.src);
    assets_path.push("assets");

    build_assets(&assets_path, &folder_names.build);

    // Register partials
    register_partials(&folder_names.src);

    let post_paths = read_markdown_files(&folder_names.src);
    let mut posts: Vec<Post> = vec![];

    for p in post_paths {
        println!(
            "Parsing and generating page for: {:?}",
            p.file_name().unwrap()
        );
        let post = parse_post(&p);
        generate_post_page(&folder_names, &post);

        posts.push(post);
    }

    println!("{}", posts.len());

    generate_index_page(posts, &folder_names.src);

    // Finally, remove the temp directory
    // remove_dir_all(folder_name); // Re-enable this for production after the user has been served
}
