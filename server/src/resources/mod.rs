pub mod book;
pub mod user;

use serde::ser::Serialize;
use tide::{Body, Response, StatusCode};

#[derive(Debug)]
pub struct ResponseData<T>(StatusCode, Option<T>);

pub fn respond_with<P>(status: StatusCode, content: Option<P>) -> tide::Result<Response>
where
    P: Serialize,
{
    let mut resp = Response::new(status);
    if let Some(item) = content {
        resp.set_body(Body::from_json(&serde_json::json!(item))?);
    }
    Ok(resp)
}
