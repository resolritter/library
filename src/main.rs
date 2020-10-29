mod entities;
mod logging;
mod messages;
mod migrations;
mod resources;

use crate::messages::{ActorGroups, BOOK};
use bastion::prelude::*;
use clap::{App, Arg, ArgMatches};
use entities::{Global, ServerState};
use once_cell::sync::OnceCell;
use parking_lot::RwLock;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::thread;
use tide::http::headers::HeaderValue;
use tide::security::CorsMiddleware;
use tide::security::Origin;
use tide::Server;

static GLOBAL: OnceCell<&'static Global> = OnceCell::new();

macro_rules! staticfy {
    ($e: expr $(,)?) => {
        &*Box::leak(Box::new($e))
    };
}

macro_rules! init_actors {
    ($($actors: ident),+) => {
        $($actors)+.set(staticfy!(RwLock::new(None))).unwrap();
    };
}

#[async_std::main]
async fn main() {
    let cli_args: &'static ArgMatches = staticfy!(App::new("library")
        .arg(
            Arg::new("db_url")
                .long("db-url")
                .about("The connection string for the PostgreSQL database")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("log_dir")
                .long("log-dir")
                .about("The directory where logs will be written to")
                .takes_value(true),
        )
        .arg(
            Arg::new("listen")
                .long("listen")
                .about("Address to bind to")
                .takes_value(true),
        )
        .arg(
            Arg::new("log_format")
                .long("log-format")
                .about("Currently only the 'test' format")
                .takes_value(true),
        )
        .arg(
            Arg::new("signal_file")
                .long("signal-file")
                .about("Specify a path to be written to when the program initializes fully.")
                .takes_value(true),
        )
        .arg(
            Arg::new("reset_before_run")
                .long("reset-before-run")
                .about("Resets the database before running the app")
                .takes_value(false),
        )
        .get_matches());

    // Initialize the logger
    let mut logging_conf = flexi_logger::Logger::with_env_or_str("library=debug");
    logging_conf = if let Some(log_format) = cli_args.value_of("log_format") {
        match log_format {
            "test" => logging_conf.format(crate::logging::test_format),
            _ => panic!("Invalid logging format {}", log_format),
        }
    } else {
        logging_conf.format(flexi_logger::opt_format)
    };
    if let Some(log_dir) = cli_args.value_of("log_dir") {
        logging_conf = logging_conf
            .log_to_file()
            .directory(log_dir)
            .duplicate_to_stdout(flexi_logger::Duplicate::All);
    }
    logging_conf.start().unwrap();

    // Initialize the database environment
    let db_url = cli_args.value_of("db_url").unwrap();
    let is_resetting_and_seeding = cli_args.is_present("reset_before_run");

    // Wait until the database comes up - especially useful during testing
    let try_limit = 3;
    let seconds_delay = 5;
    let retry_delay = std::time::Duration::from_secs(seconds_delay);
    for try_count in 0..=try_limit {
        use sqlx::any::Any;
        use sqlx::migrate::MigrateDatabase;

        if is_resetting_and_seeding {
            if let Ok(exists) = Any::database_exists(&db_url).await {
                if exists {
                    Any::drop_database(&db_url).await.unwrap();
                }
                Any::create_database(&db_url).await.unwrap();
                break;
            }
        } else if let Ok(exists) = Any::database_exists(&db_url).await {
            if !exists {
                Any::create_database(&db_url).await.unwrap();
            }
            break;
        }
        if try_count == try_limit {
            panic!(
                "Failed to connect within {} seconds ({} tries)",
                try_limit * seconds_delay,
                try_limit
            );
        } else {
            thread::sleep(retry_delay);
        }
    }

    // Initialize the database
    let db_pool_ptr: &'static PgPool = staticfy!(PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(40))
        .connect(&db_url)
        .await
        .unwrap(),);
    let db_pool: &'static PgPool = staticfy!(db_pool_ptr);
    setup_database(&db_url, db_pool, is_resetting_and_seeding).await;

    // Initialize the global static environment
    GLOBAL.set(staticfy!(Global { db_pool })).unwrap();

    // Initialize the actors
    init_actors!(BOOK);

    // Initialize the supervision tree
    let listen_addr: &'static str =
        staticfy!(cli_args.value_of("listen").unwrap_or("127.0.0.1:8080"),);
    let signal_file: Option<&'static str> =
        if let Some(signal_file_arg) = cli_args.value_of("signal_file") {
            Some(staticfy!(signal_file_arg))
        } else {
            None
        };
    Bastion::init();
    Bastion::supervisor(|sup| sup.children(resources::book::actor))
        .and_then(|_| {
            Bastion::supervisor(|sup| sup.children(|child| root(child, listen_addr, signal_file)))
        })
        .unwrap();
    Bastion::start();
    Bastion::block_until_stopped();
}

fn root(
    children: Children,
    listen_addr: &'static str,
    signal_file: Option<&'static str>,
) -> Children {
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

            server.at("/book/:title").get(resources::book::get);

            let server_handle = async_std::task::spawn(server.listen(listen_addr));
            if let Some(signal_file) = signal_file {
                std::fs::write(signal_file, "READY").unwrap();
            }
            println!("Web server listening at {}", listen_addr);
            server_handle.await.unwrap();

            Ok(())
        })
}

async fn setup_database(db_url: &str, pool: &PgPool, is_seeding: bool) {
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

    if is_seeding {
        resources::book::seed(pool).await;
    }
}
