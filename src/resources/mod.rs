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

#[macro_export]
macro_rules! actor_response {
    ($channel: ident, $t:ty $(,)?) => {
        crossbeam_channel::select! {
          recv($channel) -> msg => {
            match msg {
              Ok(item) => crate::resources::respond_with;::<$t>(Some(item), tide::StatusCode::Ok),
              _ => crate::resources::respond_with;::<$t>(None, tide::StatusCode::InternalServerError)
            }
          },
          default(std::time::Duration::from_secs(3)) => crate::resources::respond_with;::<$t>(None, tide::StatusCode::RequestTimeout),
        }
    }
}

#[macro_export]
macro_rules! actor_lookup_response {
    ($channel: ident, $t:ty $(,)?) => {
        crossbeam_channel::select! {
          recv($channel) -> msg => {
            match msg {
              Ok(item) => match item {
                Some(item) => crate::resources::respond_with::<$t>(Some(item), tide::StatusCode::Ok),
                None => crate::resources::respond_with::<$t>(None, tide::StatusCode::NotFound),
              },
              _ => crate::resources::respond_with::<$t>(None, tide::StatusCode::InternalServerError)
            }
          },
          default(std::time::Duration::from_secs(3)) => crate::resources::respond_with::<$t>(None, tide::StatusCode::RequestTimeout),
        }
    }
}

#[macro_export]
macro_rules! actor_send {
    ($name: ident, $payload: expr $(,)?) => {
        $name
            .get()
            .unwrap()
            .read()
            .as_ref()
            .unwrap()
            .send($payload)
            .unwrap();
    };
}
