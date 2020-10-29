use crate::resources::user::check_access_token;
use crate::state::ServerState;
use std::future::Future;
use std::pin::Pin;
use tide::StatusCode;

pub fn auth_middleware<'a>(
    req: tide::Request<ServerState>,
    next: tide::Next<'a, ServerState>,
) -> Pin<Box<dyn Future<Output = tide::Result> + 'a + Send>> {
    Box::pin(async move {
        match req.header("X-Auth") {
            Some(token) => {
                let result = check_access_token(token.as_str(), req.state().global.db_pool).await;
                match result {
                    Ok(true) => Ok(next.run(req).await),
                    Ok(false) => Ok(tide::Response::new(StatusCode::Forbidden)),
                    _ => Ok(tide::Response::new(StatusCode::InternalServerError)),
                }
            }
            _ => Ok(tide::Response::new(StatusCode::Forbidden)),
        }
    })
}
