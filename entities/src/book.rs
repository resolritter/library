use crate::data::{BookEndLoanByTitlePayload, BookLeaseByTitlePayload};
use crate::{book_route, end_loan_route, lease_route, server_root};
use surf::{self, http::mime::JSON, RequestBuilder, Response, StatusCode};

pub fn do_borrow(
    server_addr: &str,
    access_token: &str,
    payload: &BookLeaseByTitlePayload,
) -> RequestBuilder {
    surf::post(format!(
        concat!(server_root!(), book_route!(), lease_route!()),
        server_addr, payload.title, payload.lease_id_req
    ))
    .body(serde_json::json!(payload).to_string())
    .header("X-Auth", access_token)
    .content_type(JSON)
}
pub async fn borrow(
    server_addr: &str,
    access_token: &str,
    payload: &BookLeaseByTitlePayload,
) -> Response {
    let r = do_borrow(server_addr, access_token, payload).await.unwrap();
    assert!(r.status() == StatusCode::Ok);
    r
}

pub fn do_end_loan(server_addr: &str, payload: &BookEndLoanByTitlePayload) -> RequestBuilder {
    surf::patch(format!(
        concat!(server_root!(), book_route!(), end_loan_route!()),
        server_addr, payload.title, payload.lease_id_req
    ))
    .body(serde_json::json!(payload).to_string())
    .header("X-Auth", payload.access_token.clone())
    .content_type(JSON)
}

pub async fn end_loan(server_addr: &str, payload: &BookEndLoanByTitlePayload) -> Response {
    let r = do_end_loan(server_addr, payload).await.unwrap();
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
