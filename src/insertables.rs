use crate::schema::posts;
use crate::schema::users;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=posts)]
pub struct NewPost {
  pub title: String,
  pub body: String,
  pub published: bool,
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct NewUser {
  pub username: String,
  pub pwd: String,
}