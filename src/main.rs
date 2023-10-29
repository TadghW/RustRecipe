mod Handlers;
mod Models;
use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;
use Handlers::recipe_handler::*;
use Models::errors;
use Models::recipe;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/recipes", web::get().to(get_recipes))
            .route("/recipes", web::post().to(add_recipe))
            .route("/recipes", web::put().to(update_recipe))
            .route("/recipes", web::delete().to(remove_recipe))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}