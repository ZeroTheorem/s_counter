use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    BadRequest,
    NotFound,
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::Internal => (StatusCode::BAD_REQUEST, "Internal server error"),
        };
        (status, Json(serde_json::json!({"message": message}))).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(_: anyhow::Error) -> AppError {
        Self::Internal
    }
}
