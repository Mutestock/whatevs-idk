use thiserror::Error;

#[derive(Error, Debug)]
pub enum InvalidSqlError {
    #[error("Underlying Sql did not contain 'CREATE TABLE': \n `{0}`")]
    MissingCreateTable(String),
    #[error("Unknown sql error occurred")]
    Unknown,
}