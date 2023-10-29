use actix_web::{web, HttpResponse, get, put, post, delete};
use crate::SqlitePool;
use crate::Models::recipe::*;
use crate::Models::errors::RecipeServerError;

#[get("/recipes")]
async fn get_recipes(pool: web::Data<SqlitePool>) -> Result<HttpResponse, RecipeServerError>{
    let con = pool.get_ref();
    get_recipes(*con).await?;
}

#[post("/recipes")]
async fn add_recipe(pool: web::Data<SqlitePool>, body: web::Json<Recipe>) -> Result<HttpResponse, RecipeServerError> {
    let con = pool.get_ref();
    post_recipe(*con, body).await?;
}

#[put("/recipes")]
async fn update_recipe(pool: web::Data<SqlitePool>, body: web::Json<Recipe>) -> Result<HttpResponse, RecipeServerError> {
    let con = pool.get_ref();
    put_recipe(*con, body).await?;
}

#[delete("/recipes")]
async fn remove_recipe(pool: web::Data<SqlitePool>, body: web::Json<Recipe>) -> Result<HttpResponse, RecipeServerError> {
    let con = pool.get_ref();
    put_recipe(*con, body).await?
}
