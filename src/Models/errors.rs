use serde_json::{Error};
use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum RecipeServerError {
    SqlxError(sqlx::Error),
    SerdeError(serde_json::Error)
}

impl From<sqlx::Error> for RecipeServerError {
    fn from(err: sqlx::Error) -> RecipeServerError {
        RecipeServerError::SqlxError(err)
    }
}

impl From<serde_json::Error> for RecipeServerError {
    fn from(err: serde_json::Error) -> RecipeServerError {
        RecipeServerError::SerdeError(err)
    }
}

impl fmt::Display for RecipeServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecipeServerError::SqlxError(e) => write!("Internal Server Error with Sqlx", "{}", e),
            RecipeServerError::SerdeError(e) => write!("Serialization Error {}", "{}", e)
        }
    }
}

impl ResponseError for RecipeServerError {
    fn error_response(&self) -> HttpResponse{
        match *self {
            RecipeServerError::SqlxError(_) => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            },
            RecipeServerError::SerdeError(ref err) => {
                HttpResponse::BadRequest().json(format!("Serialization Error: {}", err))
            }
        }
    }
}