use askama_axum::Template;

#[derive(Template)]
#[template(path = "blog_posts_page_template.html")]
pub struct BlogPostsListTemplate {
    pub blog_posts: Vec<BlogPost>, 
}

pub struct BlogPost {
    href: String,
    title: String,
    tags: String,
    date: String,
    image: String,
}

pub fn build_blog_post(href: String, title: String, tags: String , date: String, image: String) -> BlogPost {
    BlogPost {
        href,
        title,
        tags,
        date,
        image
    }
}

