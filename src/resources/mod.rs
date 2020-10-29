pub mod book;

use serde::ser::Serialize;

use tide::{Body, Response, StatusCode};

pub fn respond_with<P>(payload: P, status: StatusCode) -> tide::Result<Response>
where
  P: Serialize,
{
  let mut resp = Response::new(status);
  let body = Body::from_json(&serde_json::json!({ "data": payload }))?;
  resp.set_body(body);
  Ok(resp)
}
