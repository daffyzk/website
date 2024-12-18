use crate::templates::{build_blog_post, BlogPost, BlogPostsListTemplate};

use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Response}};
use std::path::PathBuf;
use tracing::info;

pub async fn handle_yearly_blogs(Path(year): Path<u16>) -> Response {
    info!("year recieved: {year}");
    let base_dir = PathBuf::from("static/blog/posts/");
    let year_dir = base_dir.join(year.to_string());

    let year_string = year_dir.clone().into_os_string().into_string().ok().unwrap();
    // if only year is some, and year exists in dir, show page with all blog posts

    if year_dir.exists() {
        info!("file {year_string} exists");

        // get some real blogposts?
        let blog_posts: Vec<BlogPost> = vec![
            // todo remove this sample
            build_blog_post(
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
            
            // prints all directories available, should turn this data into a vector of blogposts
            for path in month_dir.read_dir().unwrap() {
                println!("Name: {}", path.unwrap().path().display())
            }

            // get some real blogposts?
            let blog_posts: Vec<BlogPost> = vec![
                build_blog_post(
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
