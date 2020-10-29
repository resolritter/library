use crate::entities::{
    Book, BookSeed, GetBookByTitlePayload, LeaseBookByTitlePayload, LeaseBookByTitleRequestBody,
    ServerState,
};
use crate::logging::logged;
use crate::messages::{
    ActorGroups, BookMsg, BookMsg::*, GetBookByTitleMsg, LeaseBookByTitleMsg, BOOK,
};
use crate::resources::ResponseData;
use log::error;
use sqlx::Done;
use sqlx::Row;
use sqlx::{postgres::PgRow, PgPool};
use std::time::SystemTime;
use tide::{Request, Response, StatusCode};

pub fn from_row(row: &PgRow) -> Result<Book, sqlx::Error> {
    Ok(Book {
        id: row.try_get("id")?,
        title: row.try_get("title")?,
        lease_id: row.try_get("lease_id")?,
        lease_until: row.try_get("lease_until")?,
    })
}

#[inline(always)]
async fn extract_get(req: &Request<ServerState>) -> tide::Result<GetBookByTitlePayload> {
    Ok(GetBookByTitlePayload {
        title: req.param("title")?,
    })
}
actor_response_handler::generate!(Config {
    name: get,
    actor: Book,
    response_type: Book,
    tag: GetBookByTitle
});

#[inline(always)]
async fn extract_lease_book(
    req: &mut Request<ServerState>,
) -> tide::Result<LeaseBookByTitlePayload> {
    let params = GetBookByTitlePayload {
        title: req.param("title")?,
    };
    let body = req.body_json::<LeaseBookByTitleRequestBody>().await?;

    Ok(LeaseBookByTitlePayload {
        title: params.title,
        lease_length: body.lease_length,
    })
}
actor_response_handler::generate!(Config {
    name: lease_book,
    actor: Book,
    response_type: String,
    tag: LeaseBookByTitle
});

#[inline(always)]
pub async fn get_by_title(msg: &GetBookByTitleMsg) -> Result<ResponseData<Book>, sqlx::Error> {
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
pub async fn lease_by_id(msg: &LeaseBookByTitleMsg) -> Result<ResponseData<String>, sqlx::Error> {
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
    let try_lease = sqlx::query(
        "UPDATE book SET lease_until=$1 WHERE title=$2 AND lease_until IS NULL OR lease_until < $3",
    )
    .bind(now + lease_length)
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

endpoint_actor::generate!({ actor: Book }, {
    GetBookByTitle: get_by_title,
    LeaseBookByTitle: lease_by_id,
});

pub fn seed_entities() -> [BookSeed; 3] {
    [
        BookSeed {
            title: "Cinderella".to_string(),
        },
        BookSeed {
            title: "Rapunzel".to_string(),
        },
        BookSeed {
            title: "Snow White".to_string(),
        },
    ]
}

pub async fn seed(pool: &PgPool) -> Vec<Book> {
    let seeding = seed_entities();
    let titles = &seeding
        .iter()
        .map(|b| b.title.clone())
        .collect::<Vec<String>>();

    sqlx::query("INSERT INTO book (title) VALUES (UNNEST($1::TEXT[])) RETURNING *")
        .bind(titles)
        .fetch_all(pool)
        .await
        .unwrap()
        .iter()
        .map(|b| from_row(b).unwrap())
        .collect()
}
