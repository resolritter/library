use crate::messages::UserCreationMsg;
use crate::resources::ResponseData;
use crate::state::ServerState;
use entities::{access_mask, UserCreationPayload, UserPublic};
use sqlx::postgres::{PgDone, PgPool, PgRow};
use sqlx::Row;
use tide::{Request, StatusCode};

pub fn from_row(row: &PgRow) -> Result<UserPublic, sqlx::Error> {
    Ok(UserPublic {
        email: row.try_get("email")?,
        access_mask: row.try_get("access_mask")?,
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

pub async fn create_super_user(
    email: &str,
    access_token: &str,
    db_pool: &PgPool,
) -> Result<PgDone, sqlx::Error> {
    Ok(
        sqlx::query(r#"INSERT INTO "user" (email, access_token, access_mask) VALUES ($1, $2, $3)"#)
            .bind(email)
            .bind(access_token)
            .bind(access_mask::ADMIN)
            .execute(db_pool)
            .await?,
    )
}

pub async fn check_access_mask(
    token: &str,
    target_mask: i32,
    db_pool: &PgPool,
) -> Result<bool, sqlx::Error> {
    if let Some(row) = sqlx::query(r#"SELECT access_mask FROM "user" WHERE access_token=$1"#)
        .bind(token)
        .fetch_optional(db_pool)
        .await?
    {
        let requester_access_mask = row.try_get::<i32, usize>(0)?;
        Ok((target_mask & requester_access_mask) == target_mask)
    } else {
        Ok(false)
    }
}

#[inline(always)]
pub async fn create_user(msg: &UserCreationMsg) -> Result<ResponseData<UserPublic>, sqlx::Error> {
    let is_authorized = match &msg.payload.requester_access_token {
        Some(token) => check_access_mask(token, msg.payload.access_mask, msg.db_pool).await?,
        None => msg.payload.access_mask == access_mask::USER,
    };
    if !is_authorized {
        return Ok(ResponseData(StatusCode::Forbidden, None));
    }

    let row = sqlx::query(
        r#"INSERT INTO "user" (email, access_mask, access_token) VALUES ($1, $2, $3) RETURNING *"#,
    )
    .bind(&msg.payload.email)
    .bind(&msg.payload.access_mask)
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
