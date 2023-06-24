use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("unknown error `{0}`")]
    Unknown(String),
}
