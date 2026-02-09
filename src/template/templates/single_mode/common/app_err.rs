use std::borrow::Cow;
use crate::common::app_err::AppError::{ParamError, RuntimeError, ServerError};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::{json, Value};
use std::convert::Into;
use tracing::error;

pub enum AppError {
    RuntimeError(Cow<'static, str>),
    ParamError(Cow<'static, str>),
    ServerError(Cow<'static, str>),
}

impl AppError {
    fn transform_json(msg: &str) -> axum::Json<Value> {
        axum::Json(json!({
            "msg": msg,
        }))
    }

    pub fn transform_response(&self, code: StatusCode, msg: &str) -> Response {
        (code, AppError::transform_json(msg)).into_response()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            ParamError(ref msg) => self.transform_response(StatusCode::BAD_REQUEST, &format!("param error: {}.", msg)),
            ServerError(ref msg) => self.transform_response(StatusCode::INTERNAL_SERVER_ERROR, &format!("server error: {}.", msg)),
            RuntimeError(ref msg) => self.transform_response(StatusCode::UNPROCESSABLE_ENTITY, &format!("runtime error: {}.", msg)),
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error> + std::fmt::Debug,
{
    fn from(value: E) -> Self {
        error!("server error: {:?}", &value);
        ServerError(Cow::Owned(format!("{:?}", value.into())))
    }
}
