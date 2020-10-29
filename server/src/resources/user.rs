use crate::messages::UserCreationMsg;
use crate::resources::ResponseData;
use crate::state::ServerState;
use entities::{UserCreationPayload, UserPublic};
use sqlx::postgres::PgRow;
use sqlx::Row;
use tide::{Request, StatusCode};

pub fn from_row(row: &PgRow) -> Result<UserPublic, sqlx::Error> {
    Ok(UserPublic {
        email: row.try_get("email")?,
        access_level: row.try_get("access_level")?,
    })
}

#[inline(always)]
pub async fn create_user(msg: &UserCreationMsg) -> Result<ResponseData<UserPublic>, sqlx::Error> {
    let row =
        sqlx::query(r#"INSERT INTO "user" (email, access_level) VALUES ($1, $2) RETURNING *"#)
            .bind(&msg.payload.email)
            .bind(&msg.payload.access_level)
            .fetch_one(msg.db_pool)
            .await?;

    Ok(ResponseData(StatusCode::Created, Some(from_row(&row)?)))
}
endpoint_actor::generate!({ actor: User }, {
    Creation: create_user,
});

#[inline(always)]
async fn extract_post(req: &mut Request<ServerState>) -> tide::Result<UserCreationPayload> {
    Ok(req.body_json::<UserCreationPayload>().await?)
}
actor_response_handler::generate!(Config {
    name: post,
    actor: User,
    response_type: UserPublic,
    tag: Creation
});
