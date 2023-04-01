use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    UserNotFound(String),
    UncaughtError(anyhow::Error),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let tuple = match self {
            AppError::UncaughtError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::UserNotFound(user_id) => {
                (StatusCode::NOT_FOUND, format!("user {} not found", user_id))
            }
        };
        tuple.into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::UncaughtError(err.into())
    }
}
