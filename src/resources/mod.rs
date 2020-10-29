#[macro_use]
pub mod book;

use serde::ser::Serialize;
use tide::{Body, Response, StatusCode};

pub fn respond_with<P>(content: Option<P>, status: StatusCode) -> tide::Result<Response>
where
    P: Serialize,
{
    let mut resp = Response::new(status);
    if let Some(item) = content {
        resp.set_body(Body::from_json(&serde_json::json!({ "data": item }))?);
    }
    Ok(resp)
}
