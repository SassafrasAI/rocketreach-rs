

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error ({status}): {message}")]
    Api { status: u16, message: String, request_id: Option<String> },

    #[error("Rate limited")]
    RateLimited { request_id: Option<String> },

    #[error("Unauthorized")]
    Unauthorized { request_id: Option<String> },

    #[error("Not found")]
    NotFound { request_id: Option<String> },

    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),
}

impl Error {
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Error::Api { request_id, .. } => request_id.as_deref(),
            Error::RateLimited { request_id } => request_id.as_deref(),
            Error::Unauthorized { request_id } => request_id.as_deref(),
            Error::NotFound { request_id } => request_id.as_deref(),
            _ => None,
        }
    }
}

pub(crate) fn map_response_error(status: reqwest::StatusCode, headers: &reqwest::header::HeaderMap, body: &str) -> Error {
    let request_id = headers
        .get("RR-Request-ID")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let message = body.to_string();

    match status.as_u16() {
        401 => Error::Unauthorized { request_id },
        404 => Error::NotFound { request_id },
        429 => Error::RateLimited { request_id },
        _ => Error::Api {
            status: status.as_u16(),
            message,
            request_id,
        },
    }
}

pub type Result<T> = std::result::Result<T, Error>;