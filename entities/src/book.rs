use crate::data::{BookLeaseByTitlePayload, UserPublic};
use crate::{book_route, lease_route, server_root};
use surf::{self, http::mime::JSON, RequestBuilder, Response, StatusCode};

pub fn do_borrow(
    server_addr: &str,
    user: Option<&UserPublic>,
    access_token: &str,
    payload: &BookLeaseByTitlePayload,
) -> RequestBuilder {
    let mut req = surf::patch(format!(
        concat!(server_root!(), book_route!(), lease_route!()),
        server_addr, payload.title, payload.lease_id_req
    ))
    .body(serde_json::json!(payload).to_string())
    .header("X-Auth", access_token)
    .content_type(JSON);
    if let Some(user) = user {
        req = req.header("From", &*user.email)
    }
    req
}
pub async fn borrow(
    server_addr: &str,
    user: Option<&UserPublic>,
    access_token: &str,
    payload: &BookLeaseByTitlePayload,
) -> Response {
    let r = do_borrow(server_addr, user, access_token, payload)
        .await
        .unwrap();
    println!("{}", r.status());
    assert!(r.status() == StatusCode::Ok);
    r
}

pub async fn get(server_addr: &str, id: &str) -> Response {
    let r = surf::get(format!(
        concat!(server_root!(), book_route!()),
        server_addr, id
    ))
    .await
    .unwrap();
    assert!(r.status() == StatusCode::Ok);
    r
}
