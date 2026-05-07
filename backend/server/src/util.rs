use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub async fn obfuscate_client_failures(response: Response) -> Response {
    match response.status() {
        StatusCode::BAD_REQUEST => (response.status(), "Bad Request").into_response(),
        StatusCode::UNPROCESSABLE_ENTITY => {
            (response.status(), "Unprocessable Entity").into_response()
        }
        _ => response,
    }
}
