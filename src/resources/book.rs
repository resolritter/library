use crate::entities::{Book, BookGet, BookGetMessage, ServerState};
use crate::messages::ActorGroups;
use crate::resources::respond_with;
use bastion::prelude::*;

use sqlx::{PgPool, Row};
use std::sync::Arc;
use tide::{Request, Response};

#[inline(always)]
pub async fn fetch_one_by_title(pool: &PgPool, title: &str) -> Option<Book> {
  sqlx::query("SELECT * FROM Book WHERE title=$1")
    .bind(title)
    .fetch_optional(pool)
    .await
    .ok()?
    .map(|row| Book {
      id: row.try_get("id").unwrap(),
      title: row.try_get("title").unwrap(),
    })
}

pub async fn get(req: Request<ServerState>) -> tide::Result<Response> {
  let payload = BookGet {
    title: req.param("title").unwrap(),
  };
  respond_with(
    {
      let state = req.state();
      let (channel, r) = crossbeam_channel::bounded(1);

      state.app.bastion.broadcast_message(
        BroadcastTarget::Group(ActorGroups::Book.to_string()),
        BookGetMessage {
          app: state.app,
          channel,
          payload,
        },
      );

      r.recv().unwrap()
    },
    tide::StatusCode::Ok,
  )
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
                  fetch_one_by_title(
                    &message.app.db_pool,
                    &message.payload.title
                  ).await.unwrap()
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
