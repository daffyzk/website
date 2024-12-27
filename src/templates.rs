use askama_axum::Template;
use std::error::Error;
use tracing::debug;

#[derive(Template)]
#[template(path = "blog_posts_page_template.html")]
pub struct BlogPostsListTemplate {
    pub blog_posts: Vec<BlogPostPreview>, 
}

#[derive(Debug)]
pub struct BlogPostPreview {
    href: String,
    title: String,
    tags: String,
    date: String,
    image: String,
}

impl BlogPostPreview{
    pub fn from_params(href: String, title: String, tags: String, date: String, image: String) -> BlogPostPreview {

        BlogPostPreview {
            href,
            title,
            tags,
            date,
            image
        }
    }
}

#[derive(Template, Debug)]
#[template(path = "blog_post_template.html", escape = "none")]
pub struct BlogPostTemplate {
    pub preview: BlogPostPreview,
    title: String,
    subtitle: String,
    header_title: String,
    header_subtitle: String,
    header_date: String,
    indexes: Vec<String>,
    content: String,
}

impl BlogPostTemplate {
    fn from_params(preview: BlogPostPreview, title: String, subtitle: String, header_title: String, header_subtitle: String, header_date: String, 
        indexes: Vec<String>, content: String) -> BlogPostTemplate {

        BlogPostTemplate{
            preview, 
            title,
            subtitle,
            header_title, 
            header_subtitle,
            header_date, 
            indexes, 
            content
        }
    }

    pub fn from_file(href: String, file_content: &str) -> Result<Self, Box<dyn Error>> {

        let mut title: String = String::new();
        let mut subtitle: String = String::new();
        let mut tags: String = String::new();
        let mut date: String = String::new();
        let mut image: String = String::new();
    
        let mut header_subtitle: String = String::new();
        let mut header_title: String = String::new();
        let mut header_date: String = String::new();
    
        let mut indexes: Vec<String> = Vec::new();
    
        let mut post_content: String = String::new();
    
        let mut current_field = "";
    
        for line in file_content.lines() {
            if line.starts_with("_*") {
                current_field = &line[2..];
                debug!("current-field: {}", current_field);
                if let Some((field, value)) = current_field.split_once("=") {
                    debug!("VALUE in field: {value}");
                    match field {
                        "title" => title = value.to_string(),
                        "subtitle" => subtitle = value.to_string(),
                        "tags" => tags = value.to_string(),
                        "date" => date = value.to_string(),
                        "image" => image = value.to_string(),
                        "header_title" => header_title = value.to_string(),
                        "header_subtitle" => header_subtitle = value.to_string(),
                        "header_date" => header_date = value.to_string(),
                        "indexes" => indexes = value.split(',').map(|s| s.to_string()).collect(),
                        _ => {}
                    }
                }
            } else if current_field == "content" {
                post_content.push_str(line);
                post_content.push('\n'); // adding newline to preserve md format
            }
        }

        debug!("title:{title}, subtitle:{subtitle}, tags:{tags}, date:{date}, image:{image}, header_title:{header_title}, header_subtitle:{header_subtitle}, header_date:{header_date}, indexes:{:?}", indexes);

        let blogpost = BlogPostTemplate::from_params(
            BlogPostPreview::from_params(href.to_string(), title.clone().to_string(), tags.to_string(), date.to_string(), image.to_string()),
            title.to_string(), subtitle.to_string(), header_title.to_string(), header_subtitle.to_string(), header_date.to_string(), indexes, post_content.to_string());

        debug!("finally: {:?}", blogpost);

    
        Ok(blogpost)
    }
}
