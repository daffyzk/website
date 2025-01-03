use crate::templates::{BlogPostTemplate, BlogPostPreview, BlogPostsListTemplate};

use std::{fs::{DirEntry, File}, io::{Error, ErrorKind, Read}, path::PathBuf};

use axum::{extract::Path, http::StatusCode, response::{IntoResponse, Response}};
use tracing::{info, error};


pub async fn handle_404() -> Response {
    (StatusCode::NOT_FOUND, "Nothing to see here!").into_response()
}

pub async fn handle_blog_index() -> Response {

    handle_blog(None, None, None) 
}

pub async fn handle_blog_post(
    Path((year, month, post_name)):
    Path<(u16, u8, String)> 
) -> Response {

    handle_blog(Some(year), Some(month), Some(post_name))
}

pub async fn handle_monthly_blog_posts(Path((year, month)): Path<(u16, u8)>) -> Response {

    handle_blog(Some(year), Some(month), None)
}

pub async fn handle_yearly_blog_posts(Path(year): Path<u16>) -> Response {
    
    handle_blog(Some(year), None, None)
}


fn handle_blog(year: Option<u16>, month: Option<u8>, post_name: Option<String>) -> Response {
   
    let base_dir: PathBuf = PathBuf::from("static/blog_posts/");

    match year {
        Some(year) => {
            info!("year recieved: {year}");
            let year_dir: PathBuf = base_dir.join(year.to_string());
            let year_str: String = year_dir.clone().into_os_string().into_string().ok().unwrap();
            // if only year is some, and year exists in dir, show page with all blog posts

            if year_dir.exists() {
                info!("dir {year_str} exists");
        
                match month {
                    Some(month) => {
                        let month_dir = year_dir.join(month.to_string());
                        let month_str = month_dir.clone().into_os_string().into_string().ok().unwrap();
                        if month_dir.exists() {  // if there is some month & month exists
                            match post_name {
                                Some(post) => {
                                    let post_file = post.clone() + ".bpd";
                                    let post_dir = month_dir.join(post_file);
                                    if post_dir.exists() {
                                        blogpost_template(post_dir)  // if post exists, return blogpost
                                    } else {
                                        let post_str: String = post_dir.into_os_string().into_string().unwrap();
                                        not_found_error(post_str, post)
                                    }
                                }
                                None => {
                                    blogpost_list_template(month_dir) // list of this month's blogposts
                                }
                            }
                        } else {
                            not_found_error(month_str, month.to_string())
                        }
                    },
                    None => {
                        blogpost_list_template(year_dir)  // list of this year's blogposts
                    },
                }
            } else {
                not_found_error(year_str, year.to_string())
            }
        },
        None => { 
            blogpost_list_template(base_dir.clone())  // list of all blogposts
        }
    }
}

fn blogpost_list_template(dir: PathBuf) -> Response {
    // old code 
    info!("dir {:?} exists", dir.clone());

    let mut blog_posts: Vec<BlogPostPreview> = Vec::new();

    let files: Vec<DirEntry> = get_all_files_in_dir(dir).unwrap();
    for path in files {
        let p: PathBuf = path.path();
        println!("Name: {}", p.display());

        // read the file into a string and send it
        let mut filestr= String::new();
        if File::open(path.path()).unwrap().read_to_string(&mut filestr).is_err() {
            error!("Could not open file");
        } else {
            blog_posts.push(
                BlogPostTemplate::from_file(
                    href_formatter(p.into_os_string().into_string().unwrap()),
                    &filestr).unwrap().preview
                    // we get the preview value for each blog post, since this is the list of blog posts
            );
        }
    }
    info!("blog post {:?}", blog_posts);
    let body: BlogPostsListTemplate = BlogPostsListTemplate::from_params(blog_posts);

    body.into_response()
}


fn blogpost_template(dir: PathBuf) -> Response {
    // when this function is called, it is assumed that the file it's calling exists (based on the
    // caller)
    let dir_str: String = dir.clone().into_os_string().into_string().unwrap();
    info!("dir {} exists", dir_str);

    let file = File::open(dir);

    match file {
        Ok(mut file) => {        
                // read the file into a string and send it
                let mut file_buf= String::new();
                match file.read_to_string(&mut file_buf) {
                    Ok(_usize) => {
                        info!("blog post {:?} was found and read", dir_str);         // we get the full blog post
                        return BlogPostTemplate::from_file(
                            href_formatter(dir_str),
                                &file_buf).unwrap().into_response();
                    }
                    Err(_) => {return internal_server_error( format!("Could not read file {dir_str}"));},
                }
        },
        Err(_) => {
            return internal_server_error(format!("Could not open file {dir_str}"));
        }
    }
}


fn not_found_error(file: String, value: String) -> Response {
    info!("uhh {file} does not exist");
    (StatusCode::NOT_FOUND, format!("No blog posts found in: {}", value)).into_response()
}


fn internal_server_error(err: String) -> Response {
    info!("internal server error: {}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Internal server error: {}", err)).into_response()
} 


fn href_formatter(mut string: String) -> String {
    string = string.replace("static/blog_posts/", "/blog/");
    string.replace(".bpd", "")
}


// Returns all bpd file paths inside a directory or tree of directiories.
fn get_all_files_in_dir(directory: PathBuf) -> Result<Vec<DirEntry>, Error> {
    let mut entries: Vec<DirEntry> = Vec::new();

    match directory.read_dir() {
        Ok(dir_iter) => {
            for entry in dir_iter {
                match entry {
                    Ok(entry) => {
                        match entry.file_type() {
                            Ok(file_type) => {
                                if file_type.is_file() && entry.path().extension().unwrap() == "bpd" {
                                    entries.push(entry); // if it's a bpd file, add it to entries vec
                                } else if file_type.is_dir() {
                                    match get_all_files_in_dir(entry.path().to_path_buf()) {
                                        Ok(files) => {
                                            for file in files {
                                                entries.push(file);  // add entry files to list
                                            }
                                        },
                                        Err(_) => {
                                            error!("error getting files for dir {:?}", entry)
                                        },
                                    }
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
                    format_and_log("Directory not found", Some(e))
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
    let err: String = if e.is_some() {
        format!("{}: {}", msg, e.unwrap())
    } else {
        format!("{}", msg)
    };
    error!(err);
    return err
}
