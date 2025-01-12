use axum::{routing::get, Router}; 
use tracing::info;
use rust_embed::RustEmbed;
use axum_embed::ServeEmbed;
use tower_http::services::ServeFile;
use crate::handlers::{handle_blog_post, handle_monthly_blog_posts, handle_yearly_blog_posts, handle_blog_index, handle_404};


//const ASSET_DIR: &str = "static/assets/";

#[derive(RustEmbed, Clone)]
#[folder = "static/assets/"]
struct Assets;

pub fn route_assets() -> Router {

    info!("asset router");
    let router: Router = Router::new().nest_service("/assets", ServeEmbed::<Assets>::new());
    
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
        .merge(route_assets())
        .fallback(handle_404)
}


fn route_blog() -> Router {

    info!("blog router");
    Router::new()
        .route("/blog", get(handle_blog_index))
        .route("/blog/:year/:month/:post_name", get(handle_blog_post))
        .route("/blog/:year/:month", get(handle_monthly_blog_posts))
        .route("/blog/:year", get(handle_yearly_blog_posts))        
}

