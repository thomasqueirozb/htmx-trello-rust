use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use sqlx::{error::Error as SqlxError, QueryBuilder}; // Import the Display derive

#[derive(Debug, Display)]
pub enum CustomError {
    #[display(fmt = "Data type error: {}", _0)]
    DataTypeError(SqlxError),
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        println!("Database error during request: {:?}", self);
        HttpResponse::InternalServerError().body("Internal Server Error")
    }
}

pub trait InIndexVector {
    fn in_index_vector(self, vec: &Vec<i64>) -> Self;
}

impl<T: sqlx::Database> InIndexVector for QueryBuilder<'_, T> {
    fn in_index_vector(mut self, vec: &Vec<i64>) -> Self {
        if vec.is_empty() {
            return self;
        }

        let values: Vec<String> = vec.iter().map(|n| format!("'{n}'")).collect();
        self.push(format!(" IN ({}) ", values.join(",")));
        self
    }
}

pub trait Helper {
    type Output;

    fn ensure_data_type(self) -> Result<Self::Output, CustomError>;
}

impl<T> Helper for Result<T, sqlx::Error> {
    type Output = T;

    fn ensure_data_type(self) -> Result<T, CustomError> {
        self.map_err(CustomError::DataTypeError)
    }
}
