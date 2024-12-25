use crate::templates::{BlogPostTemplate, BlogPostPreview, BlogPostsListTemplate};

use std::{fs::{read_dir, DirEntry, File, ReadDir}, io::{Error, ErrorKind, Read}};

use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Response}};
use std::path::PathBuf;
use tracing::{info, error};

pub async fn handle_monthly_blogs(Path((year, month)): Path<(u16, Option<u8>)>) -> Response {
    info!("year recieved: {year}");
    let base_dir: PathBuf = PathBuf::from("static/blog/posts/");
    let year_dir: PathBuf = base_dir.join(year.to_string());

    let year_string: String = year_dir.clone().into_os_string().into_string().ok().unwrap();
    // if only year is some, and year exists in dir, show page with all blog posts

    if year_dir.exists() {
        info!("dir {year_string} exists");
        
        let month_dir: PathBuf;
        let month_string: String;
        // if month.is_some() {
            month_dir = year_dir.join(month.unwrap().to_string());
            month_string = month_dir.clone().into_os_string().into_string().ok().unwrap();
        // } else {
        //     // return template for yearly blog posts
        // }
    
        // get some real blog posts?
        let mut blog_posts: Vec<BlogPostPreview> = vec![];
                

        if month_dir.exists() {
            info!("dir {month_string} exists");
            
            // prints all directories available, should turn this data into a vector of blog posts
            let files = get_all_files_in_dir(month_dir).unwrap();
            for path in files {
                let p = path.path();
                println!("Name: {}", p.display());

                // read the file into a string and send it
                let mut filestr= String::new();
                if File::open(path.path()).unwrap().read_to_string(&mut filestr).is_err() {
                    error!("Could not open file");
                } else {
                    blog_posts.push(
                        BlogPostTemplate::from_file(
                            p.into_os_string().into_string().unwrap(),
                            &filestr).unwrap().preview
                            // get the preview value for each blog post, since this is the monthly list of blog posts
                    );
                }              
            }
            info!("blog post {:?}", blog_posts);
            let body: BlogPostsListTemplate = BlogPostsListTemplate {
                blog_posts
            };

            body.into_response()

        } else {
            info!("uhh {month_string} does not exist");
            (StatusCode::NOT_FOUND, format!("No blog posts found for month: {}", month.unwrap())).into_response()
        }
    } else {
        info!("uhh {year_string} does not exist");
        (StatusCode::NOT_FOUND, format!("No blog posts found for year: {year}")).into_response()
    }
}

pub async fn handle_blog_post(Path((year, month, post)): Path<(u16, u8, String)>) -> Response {
    info!("year recieved: {year}");
    let base_dir: PathBuf = PathBuf::from("static/blog/posts/");
    let year_dir: PathBuf = base_dir.join(year.to_string());

    let year_string: String = year_dir.clone().into_os_string().into_string().ok().unwrap();
    // if only year is some, and year exists in dir, show page with all blog posts

    if year_dir.exists() {
        info!("dir {year_string} exists");

        let month_dir: PathBuf = year_dir.join(month.to_string());
        let month_string: String = month_dir.clone().into_os_string().into_string().ok().unwrap();
        if month_dir.exists() {
            info!("dir {month_string} exists");
            
            // prints all directories available, should turn this data into a vector of blog posts
            for path in month_dir.read_dir().unwrap() {
                println!("Name: {}", path.unwrap().path().display())
            }

            // get some real blogposts?
            let blog_posts: Vec<BlogPostPreview> = vec![
                BlogPostPreview::from_params(
                    "posts/2023/july/handwired-corne_en.html".to_string(),
                    "handwired corne".to_string(),
                    "storytime | build".to_string(),
                    "15-Jul-2023".to_string(),
                    "yeah".to_string()),
                ];
        
            let body = BlogPostsListTemplate {
                blog_posts
            };

            body.into_response()
        } else {
            info!("uhh {month_string} does not exist");
            (StatusCode::NOT_FOUND, format!("No blog posts found for month: {month}")).into_response()
        }
    } else {
        info!("uhh {year_string} does not exist");
        (StatusCode::NOT_FOUND, format!("No blog posts found for year: {year}")).into_response()
    }
}

// fn handle_post(Path((year, month, post_name)): Path<(u16, Option<u8>, Option<String>)>) -> 
//     impl IntoResponse {
//         let base_dir = PathBuf::from("static/blog/posts/");
//         let year_dir = base_dir.join(year.to_string());

//         // if only year is some, and year exists in dir, show page with all 

//         match (year, month, post_name) {
//             (year, Some(month), Some(post_name)) => get_blog_post(year, month, post_name),

//             // Case 2: Year and month are present, but no post name
//             (year, Some(month), None) => get_month_posts(),

//             (year, None, None) => get_year_posts(),
//         }

//         // add month if provided
//         if let Some(month) = month {
//             let file_path = year_dir.join(format!("{:02}", month)).join(format!("{}", (post_name.as_str())));

//             if month_dir.exists() && month_dir.is_file() {
//                 // Read the HTML file
//                 match fs::read_to_string(file_path) {
//                     Ok(contents) => Html(contents), // Automatically sets "Content-Type: text/html"
//                     Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file".to_string()),
//                 }
//             }
//         } else {
//             (axum::http::StatusCode::NOT_FOUND, "File not found".to_string())
//         }
// }

pub async fn handle_yearly_blogs(Path(year): Path<u16>) -> Response {
    info!("year recieved: {year}");
    let base_dir = PathBuf::from("static/blog/posts/");
    let year_dir = base_dir.join(year.to_string());

    let year_string = year_dir.clone().into_os_string().into_string().ok().unwrap();
    // if only year is some, and year exists in dir, show page with all blog posts

    if year_dir.exists() {
        let month_dirs = read_dir(year_dir).unwrap(); // only shows the directory itself. not the files it contains
        info!("file {year_string} exists");
        
        // get some real blogposts?
        let blog_posts: Vec<BlogPostPreview> = vec![
            // todo remove this sample
            BlogPostPreview::from_params(
                "posts/2023/july/handwired-corne_en.html".to_string(),
                "handwired corne".to_string(),
                "storytime | build".to_string(),
                "15-Jul-2023".to_string(),
                "yeah".to_string()),
        ];

        for value in month_dirs {
            let contains = read_dir(value.unwrap().path());

            if contains.is_ok() {
                info!("dirs {}", contains.unwrap().next().unwrap().unwrap().path().display());
            }
            else {
                info!("no files inside");
            }
        }

        let body = BlogPostsListTemplate {
            blog_posts
        };
        body.into_response()

    } else {
        info!("uhh {year_string} does not exist");
        (StatusCode::NOT_FOUND, format!("No blog posts found for year: {year}")).into_response()
    }
}

fn get_all_files_in_dir(directory: PathBuf) -> Result<Vec<DirEntry>, Error> {
    let mut entries = Vec::new();

    match directory.read_dir() {
        Ok(dir_iter) => {
            for entry in dir_iter {
                match entry {
                    Ok(entry) => {
                        match entry.file_type() {
                            Ok(file_type) => {
                                if file_type.is_file() {
                                    entries.push(entry);
                                }
                            }
                            Err(e) => 
                                return Err(
                                    Error::new(
                                        ErrorKind::Other, 
                                        format_and_log("Failed to check file type", Some(e)
                                    ))
                                ),
                        }
                    }
                    Err(e) => match e.kind() {
                        ErrorKind::NotFound => return Err(
                            Error::new(ErrorKind::NotFound, 
                                format_and_log("Directory not found", Some(e))
                            )),
                        ErrorKind::PermissionDenied => return Err(
                            Error::new(ErrorKind::PermissionDenied, 
                                format_and_log("Permission denied", None))),
                        _ => return Err(
                            Error::new(ErrorKind::Other, 
                                format_and_log("Failed to read directory entry", Some(e)))),
                    },
                }
            }
            Ok(entries)
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(
                Error::new(ErrorKind::NotFound, 
                    format_and_log("Directory not found", None)
                    )),
            ErrorKind::PermissionDenied => Err(
                Error::new(ErrorKind::PermissionDenied, 
                    format_and_log("Permission denied", None)
                    )),
            _ => Err(
                Error::new(ErrorKind::Other, 
                    format_and_log("Failed to read directory", Some(e))
                )),
        },
    }
}

fn format_and_log(msg: &str, e: Option<Error>) -> String {
    let err = if e.is_some() {
        format!("{}: {}", msg, e.unwrap())
    } else {
        format!("{}", msg)
    };
    error!(err);
    return err
}