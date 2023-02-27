use actix::SyncArbiter;
use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection
};
use actix_web::middleware::Logger;
use std::env;

mod services;
mod db_utils;
mod messages;
mod actors;
mod db_models;
mod schema;
mod insertables;

use db_utils::{get_pool, AppState, DbActor};
use services::{create_user_article, fetch_user_articles, fetch_users, update_post};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<ConnectionManager<PgConnection>> = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));
    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: db_addr.clone() }))
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
            .service(update_post)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}