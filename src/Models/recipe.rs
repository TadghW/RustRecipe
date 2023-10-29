use serde::{Deserialize, Serialize};
use sqlx::{FromRow};
use sqlx::sqlite::{SqlitePool};
use actix_web::{web, HttpResponse};
use crate::Models::errors::RecipeServerError;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Recipe {
    title: String,
    ingredients: Vec<Ingredient>,
    instructions: Vec<Instruction>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    quant: String,
    item: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instruction {
    order: u8,
    step: String
}

pub async fn get_recipes(pool: &SqlitePool) -> Result<HttpResponse, RecipeServerError>{
    let recipes = sqlx::query_as!(Recipe, "SELECT * FROM recipes").fetch_all(pool).await?;
    Ok(HttpResponse::Ok().json(recipes))
}

pub async fn post_recipe(pool: &SqlitePool, data: web::Json<Recipe>) -> Result<HttpResponse, RecipeServerError>{
    let ingredients = serde_json::to_string(&data.ingredients)?;
    let instructions = serde_json::to_string(&data.instructions)?;
    sqlx::query_as!(Recipe, "INSERT INTO recipes (title, ingredients, instructions) VALUES (?, ?, ?)", data.title, ingredients, instructions).execute(pool).await?;
    let inserted_recipe = sqlx::query_as!(Recipe, "SELECT * FROM recipes WHERE title = ?", data.0.title).fetch_one(pool).await?;
    Ok(HttpResponse::Ok().json(inserted_recipe))
}

pub async fn put_recipe(pool: &SqlitePool, data: web::Json<Recipe>) -> Result<HttpResponse, RecipeServerError>{
    let ingredients = serde_json::to_string(&data.ingredients)?;
    let instructions = serde_json::to_string(&data.instructions)?;
    sqlx::query!("UPDATE recipes SET title = ?, ingredients = ?, instructions = ? WHERE title = ?", data.title, ingredients, instructions, data.title).execute(pool).await?;
    let updated_recipe = sqlx::query_as!(Recipe, "SELECT * FROM recipes WHERE title = ?", data.0.title).fetch_one(pool).await?;
    Ok(HttpResponse::Ok().json(updated_recipe))
}

pub async fn del_recipe(pool: &SqlitePool, title: String) -> Result<HttpResponse, RecipeServerError>{
    sqlx::query!("DELETE FROM recipes WHERE title = ?", title).execute(pool).await?;
    Ok(HttpResponse::NoContent().finish())
}