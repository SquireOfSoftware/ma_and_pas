use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use std::error::Error;
use tokio_postgres::error::Error as PGError;

#[derive(Display, Debug, From)]
pub enum CustomError {
    NotFound,
    PGError(PGError),
    PGMError,
    PoolError(PoolError),
}

impl Error for CustomError {}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            CustomError::NotFound => HttpResponse::NotFound().finish(),
            CustomError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
