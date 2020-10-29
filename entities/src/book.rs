use crate::data::BookLeaseByTitlePayload;
use surf::{self, http::mime::JSON, RequestBuilder, Response, StatusCode};

pub const BOOK_ROUTE: &str = "book";

pub fn do_borrow(server_addr: &str, payload: &BookLeaseByTitlePayload) -> RequestBuilder {
    surf::patch(format!(
        "{}/{}/{}/lease/{}",
        server_addr, BOOK_ROUTE, payload.title, payload.lease_id_req
    ))
    .body(serde_json::json!(payload).to_string())
    .content_type(JSON)
}
pub async fn borrow(server_addr: &str, payload: &BookLeaseByTitlePayload) -> Response {
    let r = do_borrow(server_addr, payload).await.unwrap();
    println!("{}", r.status());
    assert!(r.status() == StatusCode::Ok);
    r
}

pub async fn get(server_addr: &str, id: &str) -> Response {
    let r = surf::get(format!("{}/{}/{}", server_addr, BOOK_ROUTE, id))
        .await
        .unwrap();
    assert!(r.status() == StatusCode::Ok);
    r
}
