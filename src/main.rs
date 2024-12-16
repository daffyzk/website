use axum::{body::Body, extract::Path, http::{header, StatusCode}, response::{IntoResponse, Response}, routing::get, Router};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{services::ServeFile, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tokio::join!(
        serve(page_router().merge(css_router()), 12443),
    );
}


async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}


fn page_router() -> Router {
    Router::new()
        .nest_service("/", ServeFile::new("static/index.html"))
        .nest_service("/contact", ServeFile::new("static/contact.html"))
        .merge(blog_router())
}


fn css_router() -> Router {
    Router::new()
        .nest_service("/css/index", ServeFile::new("static/styles/index.css"))
        .nest_service("/css/contact", ServeFile::new("static/styles/contact.css"))
        .nest_service("/css/blog", ServeFile::new("static/styles/blog.css"))
        .nest_service("/css/blog_post", ServeFile::new("static/styles/blog_post.css"))
}


fn blog_router() -> Router {
    Router::new()
        .route_service("/blog", ServeFile::new("static/blog.html"))
        .route_service("/blog/:year", get(handle_yearly_blogs))
        // .route("/blog/:year/:month", get(handle_post))
        // .route("/blog/:year/:month/:post_name", get(handle_post))
}

#[axum::debug_handler]
async fn handle_yearly_blogs(Path(year): Path<u16>) -> Response {
    info!("year recieved: {year}");
    let base_dir = PathBuf::from("static/blog/posts/");
    let year_dir = base_dir.join(year.to_string());

    let year_string = year_dir.clone().into_os_string().into_string().ok().unwrap();
    // if only year is some, and year exists in dir, show page with all blog posts


    if year_dir.exists() {
        
        info!("file {year_string} exists");

        // generate an actual html body for the response
        let body = "<html><body><dir>yeah</dir></body></html>";

        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(body)).unwrap()
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
