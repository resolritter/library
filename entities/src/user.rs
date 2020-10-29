use surf::{self, http::mime::JSON, Response, StatusCode};

pub const USER_ROUTE: &str = "user";

pub async fn create(server_addr: &str) -> Response {
    let r = surf::post(format!("{}/{}", server_addr, USER_ROUTE))
        .body(r#"{ "email": "user@user.com", "access_level": 0 }"#)
        .content_type(JSON)
        .await
        .unwrap();
    assert!(r.status() == StatusCode::Created);
    r
}
