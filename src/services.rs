use actix_web::{
    get, post, put,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use crate::{
    messages::{FetchPosts, FetchSinglePost, CreatePost, UpdatePost},
    AppState, DbActor
};
use actix::Addr;

#[derive(Deserialize)]
pub struct CreatePostBody {
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}


#[get("/healthChecker")]
pub async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Tutto funziona";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}


#[get("/posts")]
pub async fn fetch_posts(state: Data<AppState>) -> impl Responder {
    // "GET /users".to_string()
    println!("Fetch all");
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchPosts).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No posts found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve posts"),
    }
}


#[get("/post/{id}")]
pub async fn fetch_single_post(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    println!("Fetch by id {}", id);
    // format!("GET /users/{id}/articles")

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchSinglePost { post_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No post has this ID: {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve the post"),
    }
}


#[post("/createPost")]
pub async fn create_post(state: Data<AppState>, body: Json<CreatePostBody>) -> impl Responder {
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


#[put("post/{id}")]
pub async fn update_post(state: Data<AppState>, body: Json<CreatePostBody>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();   
    println!("Update");
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(UpdatePost { 
        post_id: id,
        title: body.title.to_string(),
        body: body.body.to_string(),
        published: body.published 
    }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No post has this ID"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve the post"),
    } 
}