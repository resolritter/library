use crate::entities::{Book, BookByTitlePayload, ServerState};
use crate::messages::{ActorGroups, BookByTitleMsg, BOOK};
use crate::resources::respond_with;
use bastion::prelude::*;
use sqlx::{PgPool, Row};
use tide::{Request, Response};

#[inline(always)]
pub async fn fetch_one_by_title(pool: &PgPool, title: &str) -> Result<Option<Book>, sqlx::Error> {
    let raw = sqlx::query("SELECT * FROM Book WHERE title=$1")
        .bind(title)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = raw {
        Ok(Some(Book {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
        }))
    } else {
        Ok(None)
    }
}

pub async fn get(req: Request<ServerState>) -> tide::Result<Response> {
    let payload = BookByTitlePayload {
        title: req.param("title").unwrap(),
    };
    let (reply, r) = crossbeam_channel::bounded::<Option<Book>>(1);
    let state = req.state();

    unsafe {
        BOOK.get()
            .unwrap()
            .read()
            .as_ref()
            .unwrap()
            .send(BookByTitleMsg {
                reply,
                payload,
                db_pool: state.global.db_pool,
            })
            .unwrap();
    }

    crossbeam_channel::select! {
      recv(r) -> msg => {
        match msg {
          Ok(item) => match item {
            Some(book) => respond_with::<Book>(Some(book), tide::StatusCode::Ok),
            None => respond_with::<Book>(None, tide::StatusCode::NotFound),
          },
          _ => respond_with::<Book>(None, tide::StatusCode::InternalServerError)
        }
      },
      default(std::time::Duration::from_secs(3)) => respond_with::<Book>(None, tide::StatusCode::RequestTimeout),
    }
}

pub fn actor(children: Children) -> Children {
    children
        .with_name(ActorGroups::Book.as_ref())
        .with_exec(move |_| async move {
            let (channel, r) = crossbeam_channel::unbounded::<BookByTitleMsg>();
            unsafe {
                let mut lock = BOOK.get().unwrap().write();
                *lock = Some(channel);
            }
            println!("Book actor is ready!");

            loop {
                let BookByTitleMsg {
                    reply,
                    payload,
                    db_pool,
                } = r.recv().unwrap();

                let _ = reply.send(match fetch_one_by_title(db_pool, &payload.title).await {
                    Ok(output) => output,
                    err => {
                        println!("{:#?}", err);
                        None
                    }
                });
            }

            #[allow(unreachable_code)]
            Ok(())
        })
}
