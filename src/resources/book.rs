use crate::entities::{Book, BookGet, BookGetMessage, ServerState};
use crate::messages::{ActorGroups, BOOK};
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
    let payload = BookGet {
        title: req.param("title").unwrap(),
    };
    let state = req.state();
    let (channel, r) = crossbeam_channel::bounded::<Option<Book>>(1);

    unsafe {
        let lock = BOOK.get().unwrap().lock().unwrap();
        lock.as_ref()
            .unwrap()
            .send(BookGetMessage {
                app: state.app,
                channel,
                payload,
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
    children.with_name(ActorGroups::Book.as_ref()).with_exec(
        move |_ctx: BastionContext| async move {
            let (channel, r) = crossbeam_channel::unbounded::<BookGetMessage>();
            unsafe {
                let mut lock = BOOK.get().unwrap().lock().unwrap();
                *lock = Some(channel);
            }
            println!("Book actor started!");

            loop {
                let BookGetMessage {
                    channel,
                    app,
                    payload,
                } = r.recv().unwrap();
                channel
                    .send(
                        match fetch_one_by_title(app.db_pool, &payload.title).await {
                            Ok(output) => output,
                            err => {
                                println!("{:#?}", err);
                                None
                            }
                        },
                    )
                    .unwrap();
            }

            #[allow(unreachable_code)]
            Ok(())
        },
    )
}
