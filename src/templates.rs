use askama_axum::Template;

#[derive(Template)]
#[template(path = "blog_posts_list_template.html")]
pub struct BlogPostsListTemplate<'a> {
    pub blog_posts: Vec<BlogPost<'a>>, 
}

pub struct BlogPost<'a> {
    pub title: &'a str,
    pub tags: &'a str,
    pub date: &'a str,
}

