use crate::auth::require_auth_token;
use crate::messages::{
    BookCreationMsg, BookEndLoanByTitleMsg, BookGetByTitleMsg, BookLeaseByTitleMsg,
    BookPublicListMsg,
};
use crate::resources::user::check_access_mask;
use crate::resources::ResponseData;
use crate::state::ServerState;
use entities::{
    access_mask, Book, BookCreationPayload, BookEndLoanByTitlePayload, BookGetByTitlePayload,
    BookLeaseByTitlePayload, BookLeaseByTitleRequestBody, BookPublic, BookPublicListPayload,
};
use sqlx::{postgres::PgRow, Done, PgPool, Row};
use std::time::SystemTime;
use tide::{Request, StatusCode};

pub fn from_row(row: &PgRow) -> Result<Book, sqlx::Error> {
    Ok(Book {
        id: row.try_get("id")?,
        title: row.try_get("title")?,
        lease_id: row.try_get("lease_id")?,
        lease_until: row.try_get("lease_until")?,
    })
}

pub fn public_from_row(row: &PgRow) -> Result<BookPublic, sqlx::Error> {
    Ok(BookPublic {
        id: row.try_get("id")?,
        title: row.try_get("title")?,
        lease_until: row.try_get("lease_until")?,
    })
}

#[inline(always)]
pub async fn end_loan_by_title(
    msg: &BookEndLoanByTitleMsg,
) -> Result<ResponseData<()>, sqlx::Error> {
    let raw = sqlx::query("SELECT lease_id FROM book WHERE title=$1")
        .bind(&msg.payload.title)
        .fetch_optional(msg.db_pool)
        .await?;

    if let Some(row) = raw {
        let lease_id: Option<String> = row.try_get("lease_id")?;
        if let Some(lease_id) = lease_id {
            if lease_id == msg.payload.access_token
                || check_access_mask(
                    &msg.payload.access_token,
                    access_mask::LIBRARIAN,
                    msg.db_pool,
                )
                .await?
            {
                let try_return = sqlx::query(
                    "UPDATE book SET lease_id = NULL, lease_until = NULL WHERE title=$1",
                )
                .bind(&msg.payload.title)
                .execute(msg.db_pool)
                .await?;

                if try_return.rows_affected() == 0 {
                    Ok(ResponseData(StatusCode::NotFound, None))
                } else {
                    Ok(ResponseData(StatusCode::Ok, None))
                }
            } else {
                Ok(ResponseData(StatusCode::Forbidden, None))
            }
        } else {
            Ok(ResponseData(StatusCode::Ok, None))
        }
    } else {
        Ok(ResponseData(StatusCode::NotFound, None))
    }
}
#[inline(always)]
async fn extract_end_loan(
    req: &mut Request<ServerState>,
) -> tide::Result<BookEndLoanByTitlePayload> {
    match require_auth_token(&req).await {
        (StatusCode::Ok, Some(access_token)) => Ok(BookEndLoanByTitlePayload {
            title: req.param("title")?,
            lease_id: req.param("lease_id")?,
            access_token,
        }),
        (status_code, _) => Err(tide::Error::from_str(status_code, "")),
    }
}
actor_response_handler::generate!(Config {
    name: end_loan,
    actor: Book,
    response_type: (),
    tag: EndLoanByTitle
});

#[inline(always)]
pub async fn get_by_title(msg: &BookGetByTitleMsg) -> Result<ResponseData<Book>, sqlx::Error> {
    let raw = sqlx::query("SELECT * FROM book WHERE title=$1")
        .bind(&msg.payload.title)
        .fetch_optional(msg.db_pool)
        .await?;

    if let Some(row) = raw {
        Ok(ResponseData(StatusCode::Ok, Some(from_row(&row)?)))
    } else {
        Ok(ResponseData(StatusCode::NotFound, None))
    }
}
#[inline(always)]
async fn extract_get(req: &Request<ServerState>) -> tide::Result<BookGetByTitlePayload> {
    Ok(BookGetByTitlePayload {
        title: req.param("title")?,
    })
}
actor_response_handler::generate!(Config {
    name: get,
    actor: Book,
    response_type: Book,
    tag: GetByTitle
});

#[inline(always)]
pub async fn create(msg: &BookCreationMsg) -> Result<ResponseData<Book>, sqlx::Error> {
    let is_authorized = check_access_mask(
        &msg.payload.access_token,
        access_mask::LIBRARIAN,
        msg.db_pool,
    )
    .await?;

    if !is_authorized {
        return Ok(ResponseData(StatusCode::Forbidden, None));
    }

    let row = sqlx::query("INSERT INTO book (title) VALUES ($1) RETURNING *")
        .bind(&msg.payload.title)
        .fetch_one(msg.db_pool)
        .await?;
    Ok(ResponseData(StatusCode::Created, Some(from_row(&row)?)))
}
#[inline(always)]
async fn extract_post(req: &mut Request<ServerState>) -> tide::Result<BookCreationPayload> {
    match require_auth_token(&req).await {
        (StatusCode::Ok, Some(_)) => Ok(req.body_json::<BookCreationPayload>().await?),
        (status_code, _) => Err(tide::Error::from_str(status_code, "")),
    }
}
actor_response_handler::generate!(Config {
    name: post,
    actor: Book,
    response_type: Book,
    tag: Creation
});

#[inline(always)]
pub async fn lease_by_id(msg: &BookLeaseByTitleMsg) -> Result<ResponseData<String>, sqlx::Error> {
    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    } as i64;
    let title = &msg.payload.title;

    let book_exists = sqlx::query("SELECT EXISTS(SELECT 1 FROM book WHERE title=$1)")
        .bind(title)
        .fetch_one(msg.db_pool)
        .await?
        .try_get::<bool, usize>(0)?;

    if !book_exists {
        return Ok(ResponseData(StatusCode::NotFound, None));
    }

    let lease_length = &msg.payload.lease_length;
    let lease_id = &msg.payload.lease_id;
    let try_lease = sqlx::query(
        "UPDATE book SET lease_until=$1,lease_id=$2 WHERE title=$3 AND lease_until IS NULL OR lease_until < $4",
    )
    .bind(now + lease_length)
    .bind(lease_id)
    .bind(title)
    .bind(now)
    .execute(msg.db_pool)
    .await?;

    if try_lease.rows_affected() == 0 {
        Ok(ResponseData(
            StatusCode::Forbidden,
            Some("This book is not available for borrowing".to_string()),
        ))
    } else {
        Ok(ResponseData(StatusCode::Ok, None))
    }
}
#[inline(always)]
async fn extract_lease_book(
    req: &mut Request<ServerState>,
) -> tide::Result<BookLeaseByTitlePayload> {
    match require_auth_token(&req).await {
        (StatusCode::Ok, Some(_)) => {
            let body = req.body_json::<BookLeaseByTitleRequestBody>().await?;

            Ok(BookLeaseByTitlePayload {
                title: req.param("title")?,
                lease_length: body.lease_length,
                lease_id: req.param("lease_id")?,
            })
        }
        (status_code, _) => Err(tide::Error::from_str(status_code, "")),
    }
}
actor_response_handler::generate!(Config {
    name: lease_book,
    actor: Book,
    response_type: String,
    tag: LeaseByTitle
});

#[inline(always)]
pub async fn public_list_by_query(
    msg: &BookPublicListMsg,
) -> Result<ResponseData<Vec<BookPublic>>, sqlx::Error> {
    let result = if let Some(title_query) = &msg.payload.query {
        sqlx::query("SELECT id, title, lease_until FROM book WHERE title ILIKE CONCAT('%',$1,'%')")
            .bind(title_query)
    } else {
        sqlx::query("SELECT id, title, lease_until FROM book")
    }
    .fetch_all(msg.db_pool)
    .await?;

    let mut books: Vec<BookPublic> = Vec::with_capacity(result.len());
    for row in result.iter().collect::<Vec<_>>() {
        books.push(public_from_row(row)?)
    }
    Ok(ResponseData(StatusCode::Ok, Some(books)))
}
#[inline(always)]
async fn extract_public_list(req: &Request<ServerState>) -> tide::Result<BookPublicListPayload> {
    Ok(BookPublicListPayload {
        query: req.param("query").ok(),
    })
}
actor_response_handler::generate!(Config {
    name: public_list,
    actor: Book,
    response_type: Vec<BookPublic>,
    tag: PublicList
});

endpoint_actor::generate!({ actor: Book }, {
    GetByTitle: get_by_title,
    LeaseByTitle: lease_by_id,
    EndLoanByTitle: end_loan_by_title,
    Creation: create,
    PublicList: public_list_by_query
});

pub async fn seed(pool: &PgPool) -> Vec<Book> {
    sqlx::query("INSERT INTO book (title) VALUES (UNNEST($1::TEXT[])) RETURNING *")
        .bind(vec!["Cinderella", "Rapunzel", "Snow White"])
        .fetch_all(pool)
        .await
        .unwrap()
        .iter()
        .map(|b| from_row(b).unwrap())
        .collect()
}
