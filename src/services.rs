use actix_web::{
    get, post, put, delete,
    web::{Data, Json, Path, ReqData},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use crate::{
    messages::{
        FetchPosts, 
        FetchSinglePost, 
        CreatePost, 
        UpdatePost,
        DeletePost, 
        CreateUser, 
        FetchSingleUser
    },
    AppState, DbActor, TokenClaims
};
use actix::Addr;

use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::{Hasher, Verifier, input::Password};
use chrono::NaiveDateTime;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
    use sha2::Sha256;

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

#[derive(Deserialize)]
pub struct CreateUserBody {
    username: String,
    password: String
}

#[derive(Serialize)]
pub struct UserNoPassword {
    id: i32,
    username: String,
}

#[derive(Serialize)]
pub struct AuthUser {
    id: i32,
    username: String,
    password: String,
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


#[post("user")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let user: CreateUserBody = body.into_inner();
    
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set");
    let mut hasher = Hasher::default();
    let hash = hasher
    .with_password(user.password)
    .with_secret_key(hash_secret)
    .hash()
    .unwrap();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(CreateUser {
        username: user.username.to_string(),
        pwd: hash.to_string()
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}


#[get("/auth")]
pub async fn authenticate(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set")
        .as_bytes(),
    ).unwrap();

    let username =  credentials.user_id();
    let password = credentials.password();

    match password {
        None => HttpResponse::Unauthorized().json("You must provide username and password"),
        Some(pass) => {
            match db.send(FetchSingleUser {
                username: username.to_string(),
            }).await
            {
                Ok(Ok(info)) => {
                    let hash_secret = std::env::var("HASH_SECRET")
                    .expect("HASH_SECRET must be set");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
                        .with_hash(info.pwd)
                        .with_password(pass)
                        .with_secret_key(hash_secret)
                        .verify()
                        .unwrap();
                    if is_valid {
                        let claims = TokenClaims { id: info.id };
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(token_str)
                    } else {
                        HttpResponse::Unauthorized().json("Incorrect username or password")
                    }
                }
                _ => HttpResponse::InternalServerError().json("Failed to fetch user"),
            }
        }
    }
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


#[delete("/post/{id}")]
pub async fn delete_post(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let id: i32 = path.into_inner();
    println!("{}", id);
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(DeletePost { post_id: id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json(format!("No post has this ID: {id}")),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve the post"),
    }
}