use crate::entities::{ServerState, UserCreationPayload, UserPublic};
use sqlx::postgres::PgRow;
use sqlx::Row;
use tide::Request;

pub fn from_row(row: &PgRow) -> Result<UserPublic, sqlx::Error> {
    Ok(UserPublic {
        id: row.try_get("id")?,
        email: row.try_get("email")?,
        access_level: row.try_get("access_level")?,
    })
}

#[inline(always)]
async fn extract_post(req: &mut Request<ServerState>) -> tide::Result<UserCreationPayload> {
    Ok(req.body_json::<UserCreationPayload>().await?)
}
actor_response_handler::generate!(Config {
    name: post,
    actor: User,
    response_type: String,
    tag: Creation
});
