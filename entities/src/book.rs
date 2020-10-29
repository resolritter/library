use surf::{self, http::mime::JSON, RequestBuilder, Response, StatusCode};

pub const BOOK_ROUTE: &str = "book";
const SECS_IN_DAY: u32 = 86400;

pub fn do_borrow(server_addr: &str, id: &str, seconds: u32) -> RequestBuilder {
    surf::patch(format!("{}/{}/{}", server_addr, BOOK_ROUTE, id))
        .body(format!("{{ \"lease_length\": {} }}", seconds))
        .content_type(JSON)
}
pub async fn bad_borrow(server_addr: &str, id: &str) -> Response {
    let r = do_borrow(server_addr, id, SECS_IN_DAY).await.unwrap();
    assert!(r.status() == StatusCode::Forbidden);
    r
}
pub async fn borrow(server_addr: &str, id: &str) -> Response {
    let r = do_borrow(server_addr, id, SECS_IN_DAY).await.unwrap();
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
