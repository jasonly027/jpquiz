#[utoipa::path(get, path = "/health_check", responses((status = OK, body = str)))]
pub async fn health_check() -> &'static str {
    "OK"
}
