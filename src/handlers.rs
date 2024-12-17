use crate::templates::{BlogPostsListTemplate, BlogPost};

use axum::{body::Body, extract::Path, http::{header, StatusCode}, response::{IntoResponse, Response}};
use std::path::PathBuf;
use tracing::info;

#[axum::debug_handler]
pub async fn handle_yearly_blogs(Path(year): Path<u16>) -> Response {
    info!("year recieved: {year}");
    let base_dir = PathBuf::from("static/blog/posts/");
    let year_dir = base_dir.join(year.to_string());

    let year_string = year_dir.clone().into_os_string().into_string().ok().unwrap();
    // if only year is some, and year exists in dir, show page with all blog posts


    if year_dir.exists() {
        
        info!("file {year_string} exists");

        // generate an actual html body for the response
        let body = BlogPostsListTemplate { 
            blog_posts: vec![ BlogPost{title: "yeah", tags: "yeah", date: "yeah"},  BlogPost{title: "yeah1", tags: "yeah1", date: "yeah1"} ]
        };

        // let body = "<html><body><dir>yeah</dir></body></html>";
        body.into_response()
        // Response::builder()
        //     .status(StatusCode::OK)
        //     .header(header::CONTENT_TYPE, "text/html")
        //     .body(Body::from(body)).unwrap()
    } else {
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
