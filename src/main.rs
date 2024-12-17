mod handlers;
mod templates;

use axum::{body::Body, extract::Path, http::{header, StatusCode}, response::{IntoResponse, Response}, routing::get, Router};
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{services::ServeFile, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use handlers::handle_yearly_blogs;

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