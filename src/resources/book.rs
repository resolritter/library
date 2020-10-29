use crate::entities::{Book, BookSeed, GetBookByTitlePayload, ServerState};
use crate::logging::logged;
use crate::messages::{ActorGroups, BookMsg, BookMsg::*, GetBookByTitleMsg, BOOK};
use crate::{actor_lookup_response, actor_send};
use bastion::prelude::*;
use sqlx::Row;
use sqlx::{postgres::PgRow, PgPool};
use tide::{Request, Response};

pub fn from_row(row: &PgRow) -> Result<Book, sqlx::Error> {
    Ok(Book {
        id: row.try_get("id")?,
        title: row.try_get("title")?,
    })
}

#[inline(always)]
pub async fn fetch_one_by_title(pool: &PgPool, title: &str) -> Result<Option<Book>, sqlx::Error> {
    let raw = sqlx::query("SELECT * FROM Book WHERE title=$1")
        .bind(title)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = raw {
        Ok(Some(from_row(&row)?))
    } else {
        Ok(None)
    }
}

pub async fn get(req: Request<ServerState>) -> tide::Result<Response> {
    let payload = GetBookByTitlePayload {
        title: req.param("title").unwrap(),
    };
    let (reply, r) = crossbeam_channel::bounded::<Option<Book>>(1);
    let state = req.state();

    actor_send!(
        BOOK,
        GetByTitle(GetBookByTitleMsg {
            reply,
            payload,
            db_pool: state.global.db_pool,
        }),
    );
    actor_lookup_response!(r, Book)
}

pub fn actor(children: Children) -> Children {
    children
        .with_name(ActorGroups::Book.as_ref())
        .with_exec(move |_| async move {
            let (channel, r) = crossbeam_channel::unbounded::<BookMsg>();
            unsafe {
                let mut lock = BOOK.get().unwrap().write();
                *lock = Some(channel);
            }
            println!("Book actor is ready!");

            loop {
                match logged(r.recv().unwrap()) {
                    GetByTitle(GetBookByTitleMsg {
                        reply,
                        payload,
                        db_pool,
                    }) => {
                        let _ =
                            reply.send(match fetch_one_by_title(db_pool, &payload.title).await {
                                Ok(output) => output,
                                err => {
                                    // TODO: implement logging
                                    println!("{:#?}", err);
                                    None
                                }
                            });
                    }
                }
            }

            #[allow(unreachable_code)]
            Ok(())
        })
}

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
