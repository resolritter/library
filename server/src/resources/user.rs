use crate::messages::{UserCreationMsg, UserLoginMsg};
use crate::resources::ResponseData;
use crate::state::ServerState;
use entities::{access_mask, User, UserCreationPayload, UserLoginPayload};
use sqlx::postgres::{PgDone, PgPool, PgRow};
use sqlx::Row;
use tide::{Request, StatusCode};

pub fn from_row(row: &PgRow) -> Result<User, sqlx::Error> {
    Ok(User {
        email: row.try_get("email")?,
        access_mask: row.try_get("access_mask")?,
        access_token: row.try_get("access_token")?,
    })
}

pub async fn check_access_token(access_token: &str, db_pool: &PgPool) -> Result<bool, sqlx::Error> {
    Ok(
        sqlx::query(r#"SELECT EXISTS(SELECT 1 FROM "user" WHERE access_token=$1)"#)
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
pub async fn create_session(msg: &UserLoginMsg) -> Result<ResponseData<User>, sqlx::Error> {
    let row = sqlx::query(r#"SELECT * FROM "user" WHERE email=$1"#)
        .bind(&msg.payload.email)
        .fetch_optional(msg.db_pool)
        .await?;
    if let Some(row) = row {
        Ok(ResponseData(StatusCode::Created, Some(from_row(&row)?)))
    } else {
        Ok(ResponseData(StatusCode::NotFound, None))
    }
}
#[inline(always)]
async fn extract_login(req: &mut Request<ServerState>) -> tide::Result<UserLoginPayload> {
    Ok(req.body_json::<UserLoginPayload>().await?)
}
actor_response_handler::generate!({
    name: login,
    actor: User,
    response_type: User,
    tag: Login
});

#[inline(always)]
pub async fn create_user(msg: &UserCreationMsg) -> Result<ResponseData<User>, sqlx::Error> {
    let is_authorized = match &msg.payload.requester_access_token {
        Some(token) => check_access_mask(token, access_mask::ADMIN, msg.db_pool).await?,
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
#[inline(always)]
async fn extract_post(req: &mut Request<ServerState>) -> tide::Result<UserCreationPayload> {
    Ok(req.body_json::<UserCreationPayload>().await?)
}
actor_response_handler::generate!({
    name: post,
    actor: User,
    response_type: User,
    tag: Creation
});

endpoint_actor::generate!({ actor: User }, {
    Creation: create_user,
    Login: create_session,
});
