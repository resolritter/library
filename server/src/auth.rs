use crate::resources::user::check_access_token;
use crate::state::ServerState;
use tide::{Request, StatusCode};

pub async fn require_auth_token(req: &Request<ServerState>) -> (StatusCode, Option<String>) {
    match req.header("X-Auth") {
        Some(token) => {
            let t = token.as_str();
            match check_access_token(t, req.state().global.db_pool).await {
                Ok(true) => (StatusCode::Ok, Some(t.to_string())),
                Ok(false) => (StatusCode::Forbidden, Some(t.to_string())),
                _ => (StatusCode::InternalServerError, Some(t.to_string())),
            }
        }
        _ => (StatusCode::Forbidden, None),
    }
}
