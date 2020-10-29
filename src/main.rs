mod entities;
mod messages;
mod migrations;
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
    // Initialize the database environment
    let db_url = std::env::var("DB_URL").unwrap();
    let is_reset_and_seed = std::env::var("RESET_AND_SEED").is_ok();

    if is_reset_and_seed {
        use sqlx::any::Any;
        use sqlx::migrate::MigrateDatabase;
        if Any::database_exists(&db_url).await.unwrap() {
            Any::drop_database(&db_url).await.unwrap();
        }
        Any::create_database(&db_url).await.unwrap();
    }

    // Set up the database
    let db_pool_ptr: &'static PgPool = &*Box::leak(Box::new(
        PgPoolOptions::new().connect(&db_url).await.unwrap(),
    ));
    let db_pool: &'static PgPool = &*Box::leak(Box::new(db_pool_ptr));
    setup_database(&db_url, db_pool, is_reset_and_seed).await;

    // Initialize the global, read-only, static environment
    GLOBAL
        .set(&*Box::leak(Box::new(Global { db_pool })))
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

async fn setup_database(db_url: &str, pool: &PgPool, is_reset_and_seed: bool) {
    use refinery_core::postgres::{Client, NoTls};
    let mut client = Client::connect(db_url, NoTls).unwrap();

    let reports = crate::migrations::runner().run(&mut client).unwrap();
    for applied in reports.applied_migrations() {
        println!(
            "DB Migration Applied - Name: {}, Version: {}",
            applied.name(),
            applied.version()
        );
    }

    if is_reset_and_seed {
        resources::book::seed(pool).await;
    }
}
