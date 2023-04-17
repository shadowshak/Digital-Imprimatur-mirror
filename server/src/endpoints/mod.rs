use axum::http::StatusCode;

pub mod user;
pub mod sub;
pub mod doc;
pub mod feedback;


fn check_length(
    value: &str,
    max_length: usize,
) -> Result<(), StatusCode> {
    if value.len() > max_length {
        return Err(StatusCode::PAYLOAD_TOO_LARGE)
    }

    Ok(())
}