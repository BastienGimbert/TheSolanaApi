use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),
    #[error("validator selection failed: {0}")]
    Selection(String),
    #[error("upstream request failed: {0}")]
    Upstream(String),
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_) | AppError::Selection(_) => StatusCode::BAD_REQUEST,
            AppError::Upstream(_) => StatusCode::BAD_GATEWAY,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let payload = ErrorResponse {
            error: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(payload)
    }
}

impl From<awc::error::SendRequestError> for AppError {
    fn from(value: awc::error::SendRequestError) -> Self {
        AppError::Upstream(value.to_string())
    }
}

impl From<awc::error::PayloadError> for AppError {
    fn from(value: awc::error::PayloadError) -> Self {
        AppError::Upstream(value.to_string())
    }
}

impl From<crate::validators::SelectionError> for AppError {
    fn from(value: crate::validators::SelectionError) -> Self {
        AppError::Selection(value.to_string())
    }
}
