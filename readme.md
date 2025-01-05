## what
just a little website, that collects some of my thoughts.

i wanted this site to run completely without js, only good old html and css.

there's blog posts and contact info. 


## how
Webserver in rust, routed with axum.
Blog handler is the only 'complicated' feature, it's a function that returns an http response, based on an Askama template (like jinja but rusty).
It reads values from '.bpd' files, an acronym for 'Blog Post Data', which is pretty much an ENV file, but the last field should be 'content', and every character after that will be considered part of the blog post's content. 

### templates
There's two templates, blog_post and blog_posts_list, they are filled by a recursive function that takes all the values from each directory, checks if they are '.bpd' files, and appends them to a vector of 'DirEntry' values.

this vec, can then be used to read each file, and pass their bpd values to a struct, which is then used to generate the html templates on compile time.
(yes, this means you have to re-build the server if there's html changes on the templates).

#### blog_post
holds the actual blog post, consists of a title, subtitle, an index list, and the post content.

#### blog_posts_list
holds a list of blog post previews, kind of like thumbnails. These are comprised of an image, href, title, date, and tags.
the href value is inferred form the actual blog post path, the rest of the values are defined on a bpd file.


To handle the blog thumbnail and 

