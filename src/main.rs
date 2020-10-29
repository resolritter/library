mod entities;
mod messages;
mod resources;

use crate::messages::{ActorGroups, BOOK};
use bastion::prelude::*;
use entities::{Global, ServerState};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tide::http::headers::HeaderValue;
use tide::security::CorsMiddleware;
use tide::security::Origin;
use tide::Server;

static GLOBAL: OnceCell<&'static Global> = OnceCell::new();

macro_rules! init_actors {
    ($($actors: ident),+) => {
        $($actors)+.set(&*Box::leak(Box::new(RwLock::new(None)))).unwrap();
    };
}

#[async_std::main]
async fn main() {
    // Initialize the global, read-only, static environment
    let db_url: &'static str = &*Box::leak(Box::new(std::env::var("DB_URL").unwrap()));
    let db_pool_ptr: &'static PgPool = Box::leak(Box::new(
        PgPoolOptions::new().connect(&db_url).await.unwrap(),
    ));
    let db_pool: &'static PgPool = &*Box::leak(Box::new(db_pool_ptr));
    GLOBAL
        .set(&*Box::leak(Box::new(Global { db_pool, db_url })))
        .unwrap();

    // Initialize the actors
    unsafe {
        init_actors!(BOOK);
    }

    // Initialize the supervision tree
    Bastion::init();
    Bastion::supervisor(|sup| sup.children(resources::book::actor))
        .and_then(|_| Bastion::supervisor(|sup| sup.children(root)))
        .unwrap();
    Bastion::start();
    Bastion::block_until_stopped();
}

fn root(children: Children) -> Children {
    children
        .with_name(ActorGroups::Input.as_ref())
        .with_exec(move |_| async move {
            let mut server = Server::with_state(ServerState {
                global: GLOBAL.get().unwrap(),
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

            let listen_addr = "127.0.0.1:8080";
            println!("Web server listening at {}", listen_addr);
            server.listen(listen_addr).await.unwrap();

            Ok(())
        })
}
