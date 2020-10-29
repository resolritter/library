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
        let status = match (req.header("From"), req.header("X-Auth")) {
            (Some(email), Some(access_token)) => match (email.get(0), access_token.get(0)) {
                (Some(email), Some(access_token)) => {
                    let result = check_access_token(
                        email.as_str(),
                        access_token.as_str(),
                        req.state().global.db_pool,
                    )
                    .await;
                    match result {
                        Ok(true) => StatusCode::Ok,
                        Ok(false) => StatusCode::Forbidden,
                        _ => StatusCode::InternalServerError,
                    }
                }
                _ => StatusCode::Forbidden,
            },
            _ => StatusCode::Forbidden,
        };

        if status == StatusCode::Ok {
            Ok(next.run(req).await)
        } else {
            Ok(tide::Response::new(status))
        }
    })
}
