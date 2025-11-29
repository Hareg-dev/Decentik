use askama::Template;
use dtt_core::domain::Post;

#[derive(Template)]
#[template(path = "feed.html")]
pub struct FeedTemplate {
    pub posts: Vec<Post>,
    pub user_id: String,
}

#[derive(Template)]
#[template(path = "upload.html")]
pub struct UploadTemplate {
    pub user_id: String,
}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTemplate {
    pub user_id: String,
    pub posts: Vec<Post>,
}