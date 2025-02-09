use axum::{
    extract::rejection::JsonRejection, extract::FromRequest, http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::json;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

// We implement `IntoResponse` for our extractor so it can be used as a response
impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> axum::response::Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

// We create our own rejection type
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

// We implement `From<JsonRejection> for ApiError`
impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

// We implement `IntoResponse` so `ApiError` can be used as a response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "message": self.message,
        });

        (self.status, axum::Json(payload)).into_response()
    }
}
