use crate::data::{UserCreationPayload, UserPublic};
use surf::{self, http::mime::JSON, Response, StatusCode};

pub const USER_ROUTE: &str = "user";

pub async fn do_create(server_addr: &str, payload: &UserCreationPayload) -> Response {
    surf::post(format!("{}/{}", server_addr, USER_ROUTE))
        .body(serde_json::json!(payload))
        .content_type(JSON)
        .await
        .unwrap()
}

pub async fn create(server_addr: &str, payload: &UserCreationPayload) -> (String, UserPublic) {
    let mut response = do_create(server_addr, payload).await;
    assert!(response.status() == StatusCode::Created);
    let str_body = response.body_string().await.unwrap();
    let value = serde_json::from_str::<UserPublic>(&str_body).unwrap();
    (str_body, value)
}
