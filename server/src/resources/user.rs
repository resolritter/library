use crate::messages::UserCreationMsg;
use crate::resources::ResponseData;
use crate::state::ServerState;
use entities::{UserCreationPayload, UserPublic};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use tide::{Request, StatusCode};

pub fn from_row(row: &PgRow) -> Result<UserPublic, sqlx::Error> {
    Ok(UserPublic {
        email: row.try_get("email")?,
        access_level: row.try_get("access_level")?,
        access_token: row.try_get("access_token")?,
    })
}

pub async fn check_access_token(
    email: &str,
    access_token: &str,
    db_pool: &PgPool,
) -> Result<bool, sqlx::Error> {
    Ok(
        sqlx::query(r#"SELECT EXISTS(SELECT 1 FROM "user" WHERE email=$1 AND access_token=$2)"#)
            .bind(email)
            .bind(access_token)
            .fetch_one(db_pool)
            .await?
            .try_get::<bool, usize>(0)?,
    )
}

#[inline(always)]
pub async fn create_user(msg: &UserCreationMsg) -> Result<ResponseData<UserPublic>, sqlx::Error> {
    let row = sqlx::query(
        r#"INSERT INTO "user" (email, access_level, access_token) VALUES ($1, $2, $3) RETURNING *"#,
    )
    .bind(&msg.payload.email)
    .bind(&msg.payload.access_level)
    // TODO have an actual access token generation strategy
    .bind(&msg.payload.email)
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
