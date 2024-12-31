use axum::{routing::get, Router}; 
use tracing::info;
use tower_http::services::ServeFile;
use crate::handlers::{handle_blog_post, handle_monthly_blog_posts, handle_yearly_blog_posts, handle_404};

pub fn route_images() -> Router {

    info!("img router");
    let router: Router = Router::new().route_service("/images/", ServeFile::new("static/images/"));
    
    return router;    
}


pub fn route_money() -> Router {
    Router::new().route_service("/", ServeFile::new("static/wallets.html"))
}


pub fn route_page() -> Router {
    Router::new()
        .nest_service("/", ServeFile::new("static/index.html"))
        .nest_service("/contact", ServeFile::new("static/contact.html"))
        .merge(route_blog())
        .merge(route_css())
        .merge(route_images())
        .fallback(handle_404)
}


fn route_blog() -> Router {

    info!("blog router");
    Router::new()
        .route_service("/blog", ServeFile::new("static/blog.html"))
        .route("/blog/:year/:month/:post_name", get(handle_blog_post))
        .route("/blog/:year/:month", get(handle_monthly_blog_posts))
        .route("/blog/:year", get(handle_yearly_blog_posts))        
}


fn route_css() -> Router {
    Router::new()
        .nest_service("/css/index", ServeFile::new("static/styles/index.css"))
        .nest_service("/css/contact", ServeFile::new("static/styles/contact.css"))
        .nest_service("/css/blog", ServeFile::new("static/styles/blog.css"))
        .nest_service("/css/blog_post", ServeFile::new("static/styles/blog_post.css"))
}

