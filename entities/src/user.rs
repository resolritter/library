use crate::data::{UserCreationPayload, UserPublic};
use surf::{self, http::mime::JSON, StatusCode};

pub const USER_ROUTE: &str = "user";

pub async fn create(server_addr: &str, payload: &UserCreationPayload) -> (String, UserPublic) {
    let mut response = surf::post(format!("{}/{}", server_addr, USER_ROUTE))
        .body(serde_json::json!(payload))
        .content_type(JSON)
        .await
        .unwrap();
    assert!(response.status() == StatusCode::Created);
    let str_body = response.body_string().await.unwrap();
    let value = serde_json::from_str::<UserPublic>(&str_body).unwrap();
    (str_body, value)
}
