use crate::schema::posts;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=posts)]
pub struct NewPost {
  pub title: String,
  pub body: String,
  pub published: bool,
}