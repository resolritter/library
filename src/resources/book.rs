use crate::entities::{Book, BookGet, BookGetMessage, ServerState};
use crate::messages::ActorGroups;
use crate::resources::respond_with;
use bastion::prelude::*;

use sqlx::{PgPool, Row};
use std::sync::Arc;
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

  state.app.bastion.broadcast_message(
    BroadcastTarget::Group(ActorGroups::Book.to_string()),
    BookGetMessage {
      app: state.app,
      channel,
      payload,
    },
  );

  match r.recv() {
    Ok(item) => match item {
      Some(book) => respond_with(Some(book), tide::StatusCode::Ok),
      None => respond_with::<Book>(None, tide::StatusCode::NotFound),
    },
    _ => respond_with::<Book>(None, tide::StatusCode::InternalServerError),
  }
}

pub fn actor(children: Children) -> Children {
  children
    .with_name(ActorGroups::Book.as_ref())
    .with_dispatcher(Dispatcher::with_type(DispatcherType::Named(
      ActorGroups::Book.to_string(),
    )))
    .with_exec(move |ctx: BastionContext| async move {
      println!("Book is started");

      loop {
        msg! { ctx.recv().await?,
          raw: Arc<SignedMessage> => {
            msg! {
              Arc::try_unwrap(raw).unwrap(),
              ref message: BookGetMessage => {
                message.channel.send(
                  match fetch_one_by_title(
                    &message.app.db_pool,
                    &message.payload.title
                  ).await {
                      Ok(output) => output,
                      err => {
                        println!("{:#?}", err);
                        None
                      }
                  }
                ).unwrap();
              };
              _: _ => ();
            }
          };
          _: _ => ();
        }
      }

      #[allow(unreachable_code)]
      Ok(())
    })
}
