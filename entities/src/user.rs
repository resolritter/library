use crate::data::{User, UserCreatePayload, UserLoginPayload, UserOkResponse};
use crate::{server_root, session_route};
use surf::{self, http::mime::JSON, Response, StatusCode};

pub const USER_ROUTE: &str = "user";

pub async fn do_create(server_addr: &str, payload: &UserCreatePayload) -> Response {
    surf::post(format!("{}/{}", server_addr, USER_ROUTE))
        .body(serde_json::json!(payload))
        .content_type(JSON)
        .await
        .unwrap()
}

pub async fn create(server_addr: &str, payload: &UserCreatePayload) -> (String, User) {
    let mut response = do_create(server_addr, payload).await;
    assert!(response.status() == StatusCode::Created);
    let str_body = response.body_string().await.unwrap();
    let value = serde_json::from_str::<UserOkResponse>(&str_body).unwrap();
    (str_body, value.Ok)
}

pub async fn do_login(server_addr: &str, payload: &UserLoginPayload) -> Response {
    surf::post(format!(
        concat!(server_root!(), session_route!()),
        server_addr,
    ))
    .body(serde_json::json!(payload))
    .content_type(JSON)
    .await
    .unwrap()
}

pub async fn login(server_addr: &str, payload: &UserLoginPayload) -> (String, User) {
    let mut response = do_login(server_addr, payload).await;
    assert!(response.status() == StatusCode::Created);
    let str_body = response.body_string().await.unwrap();
    let value = serde_json::from_str::<User>(&str_body).unwrap();
    (str_body, value)
}
