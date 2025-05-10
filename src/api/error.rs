use anyhow::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub struct AppError(pub Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 打印调用栈和错误信息
        eprintln!("Internal error: {:#}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Error>,
{
    fn from(err: E) -> Self {
        AppError(err.into())
    }
}
