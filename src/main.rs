mod entities;
mod messages;
mod resources;

use crate::messages::ActorGroups;
use bastion::prelude::*;
use entities::{App, ServerState};
use once_cell::sync::OnceCell;
use sqlx::postgres::PgPoolOptions;

use std::env;

use tide::http::headers::HeaderValue;
use tide::security::CorsMiddleware;
use tide::security::Origin;
use tide::Server;

fn main() {
  Bastion::init();

  Bastion::supervisor(|sup| sup.children(root))
    .and_then(|_| Bastion::supervisor(|sup| sup.children(resources::book::actor)))
    .expect("Couldn't create supervisor chain.");

  Bastion::start();
  Bastion::block_until_stopped();
}

static APP: OnceCell<App> = OnceCell::new();
static BASTION: OnceCell<BastionContext> = OnceCell::new();

fn root(children: Children) -> Children {
  children.with_name(ActorGroups::Input.as_ref()).with_exec(
    move |bastion: BastionContext| async move {
      // if the supervisor has to restart for some reason, put the new instance behind the cell
      if let Err(bastion) = BASTION.set(bastion) {
        BASTION.take();
        BASTION.set(bastion).unwrap();
      }

      let db_pool = PgPoolOptions::new()
        .connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
      APP
        .set(App {
          db_pool,
          bastion: BASTION.get().unwrap(),
        })
        .unwrap();
      let mut server = Server::with_state(ServerState {
        app: APP.get().unwrap(),
      });

      server.with(
        CorsMiddleware::new()
          .allow_methods(
            "GET, POST, PUT, PATCH, DELETE, OPTIONS"
              .parse::<HeaderValue>()
              .unwrap(),
          )
          .allow_origin(Origin::Any),
      );

      // Book routes
      server.at("/book/:title").get(resources::book::get);

      server.listen("127.0.0.1:8080").await.unwrap();

      Ok(())
    },
  )
}
