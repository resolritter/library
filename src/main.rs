mod entities;
mod messages;
mod resources;

use crate::messages::ActorGroups;
use bastion::prelude::*;
use entities::{App, Config, ServerState};
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

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

static mut APP: OnceCell<&'static Arc<&'static App>> = OnceCell::new();
static CONFIG: OnceCell<Config> = OnceCell::new();

fn initialize_config_once() -> &'static Config {
  let _ = CONFIG.set(Config {
    db_url: std::env::var("DB_URL").unwrap(),
  });
  CONFIG.get().unwrap()
}

fn root(children: Children) -> Children {
  let Config { db_url } = initialize_config_once();

  children.with_name(ActorGroups::Input.as_ref()).with_exec(
    move |bastion_ctx: BastionContext| async move {
      let bastion_ptr: &'static BastionContext = Box::leak(Box::new(Arc::new(bastion_ctx)));
      let bastion: &'static Arc<&'static BastionContext> =
        Box::leak(Box::new(Arc::new(bastion_ptr)));
      let db_pool_ptr: &'static PgPool = Box::leak(Box::new(
        PgPoolOptions::new().connect(&db_url).await.unwrap(),
      ));
      let db_pool: &'static Arc<&'static PgPool> = &*Box::leak(Box::new(Arc::new(db_pool_ptr)));

      let app_ptr: &'static App = &*Box::leak(Box::new(App { db_pool, bastion }));
      let app: &'static Arc<&'static App> = Box::leak(Box::new(Arc::new(app_ptr)));

      let mut server = unsafe {
        if let Err(app) = APP.set(app) {
          APP.take();
          APP.set(app).unwrap();
        }
        Server::with_state(ServerState {
          app: APP.get().unwrap(),
        })
      };

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

      let listen_addr = "127.0.0.1:8080";
      println!("Web server listening at {}", listen_addr);
      server.listen(listen_addr).await.unwrap();

      Ok(())
    },
  )
}
