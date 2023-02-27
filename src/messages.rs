use crate::db_models::{Post};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Post>>")]
pub struct FetchPosts;

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct FetchSinglePost {
  pub post_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct CreatePost {
  pub title: String,
  pub body: String,
  pub published: bool,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct UpdatePost {
  pub post_id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
}