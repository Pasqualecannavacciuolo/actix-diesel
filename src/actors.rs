use crate::db_models::Post;
use crate::db_utils::DbActor;
use crate::schema::posts::dsl::*;
use crate::messages::{FetchPosts, FetchSinglePost, CreatePost, UpdatePost, DeletePost};
use crate::insertables::NewPost;
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchPosts> for DbActor {
  type Result = QueryResult<Vec<Post>>;

  fn handle(&mut self, _msg: FetchPosts, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch Post: Unable to establish connection");

    posts.get_results::<Post>(&mut conn)
  }
}


impl Handler<FetchSinglePost> for DbActor {
  type Result = QueryResult<Post>;

  fn handle(&mut self, msg: FetchSinglePost, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Fetch Single Post: Unable to establish connection");

    posts.filter(id.eq(msg.post_id)).get_result::<Post>(&mut conn)
  }
}


impl Handler<CreatePost> for DbActor {
  type Result = QueryResult<Post>;

  fn handle(&mut self, msg: CreatePost, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Create Post: Unable to establish connection");

    let new_post = NewPost {
      title: msg.title,
      body: msg.body,
      published: false,
    };

    diesel::insert_into(posts)
      .values(new_post)
      .returning((
        id,
        title,
        body,
        published,
      ))
      .get_result::<Post>(&mut conn)
  }
}


impl Handler<UpdatePost> for DbActor {
  type Result = QueryResult<Post>;

  fn handle(&mut self, msg: UpdatePost, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Update Post: Unable to establish connection");

    diesel::update(posts.filter(id.eq(msg.post_id as i32)))
    .set((title.eq(msg.title), body.eq(msg.body), published.eq(msg.published)))
    .get_result::<Post>(&mut conn)

  }
}


impl Handler<DeletePost> for DbActor {
  type Result = QueryResult<Post>;

  fn handle(&mut self, msg: DeletePost, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Delete Post: Unable to establish connection");

    diesel::delete(posts.filter(id.eq(msg.post_id))).get_result::<Post>(&mut conn)
  }
}