use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse,
};
use serde::Deserialize;
use crate::{
    messages::{FetchPosts, FetchSinglePost, CreatePost},
    AppState, DbActor
};
use actix::Addr;

#[derive(Deserialize)]
pub struct CreatePostBody {
    pub title: String,
    pub body: String,
    pub published: bool
}

#[get("/posts")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchPosts).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No posts found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve posts"),
    }
}

#[get("/post/{id}")]
pub async fn fetch_user_articles(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    // format!("GET /users/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchSinglePost { post_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No post has this ID: {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve the post"),
    }
}

#[post("/createPost")]
pub async fn create_user_article(state: Data<AppState>, body: Json<CreatePostBody>) -> impl Responder {
    // format!("POST /users/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreatePost {
        title: body.title.to_string(),
        body: body.body.to_string(),
        published: body.published
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create post"),
    }
}