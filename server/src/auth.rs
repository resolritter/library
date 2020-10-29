use crate::resources::user::check_access_token;
use crate::state::ServerState;
use tide::{Request, StatusCode};

pub async fn require_auth_token(req: &Request<ServerState>) -> tide::Result<String> {
    match req.header("X-Auth") {
        Some(token) => {
            let t = token.as_str();
            match check_access_token(t, req.state().global.db_pool).await {
                Ok(true) => Ok(t.to_string()),
                Ok(false) => Err(tide::Error::from_str(
                    StatusCode::Forbidden,
                    "Invalid token",
                )),
                _ => Err(tide::Error::from_str(
                    StatusCode::InternalServerError,
                    "Unknown error",
                )),
            }
        }
        _ => Err(tide::Error::from_str(
            StatusCode::Forbidden,
            "Missing token",
        )),
    }
}
