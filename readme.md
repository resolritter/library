# Library

# Table of Contents

1. [Purpose](#purpose)

2. [Why "Library"](#why-library)

    2.1 [User-facing server features](#product-features)

    2.2 [Client features](#client-features)

    2.3 [Infrastructural server features](#infrastructural-server-features)

3. [Technology stack](#tech-stack)

4. [Installing and running](#installing-and-running)

5. [How it works](#how-it-works)

    5.1. [Nomenclature](#nomenclature)

    5.2. [entities](#entities-crate)

    5.3. [actor_msg_resources](#actor_msg_resources)

    5.4. [actor_request_handler](#actor_request_handler)

    5.5. [Recap - Putting it all together](#recap)

    5.6. [Finally, start the app](#start)

6. [How testing is done](#how-testing-is-done)

7. [Missing for production](#missing-for-production)

# Purpose  <a name="purpose"></a>

This project is meant to be a demonstration of how to manage, code and structure a full-stack application.
Its main intent is exercising and justifying **engineering decisions and practices**, thus the
implementation purposefully doesn't go into detailed domain-specific requirements - in fact, only the very
primitive set of requirements are implemented, and only for the sake of demonstrating the aforementioned priorities.
On a similar vein, a bunch of things would be missing for it to be a "production-ready" application, which
will be later described in the [Missing for production](#missing-for-production) section.

# Why "Library"   <a name="why-library"></a>

The project is named "Library" because its implementation entails a basic library system, i.e. a system based around
books as a core entity and the activity of borrowing and returning it. This theme allows for a primitive/comprehensible
enough implementation which wouldn't take too much away from the the project's
main intent, afterall the domain-specific details are merely ways to showcase the architecture and technology.

## 2.1 User-facing server features <a name="product-features"></a>

Abilities which matter for the product and are exposed through the API. The routes are
grouped by resource, with semantic hierarchical nesting by interplay of slashes and parameters
(e.g. borrowing is enacted per book, thus the borrow routes are nested within the book route).

### User API <a name="user-api"></a>

#### Creation - `POST /user`

Allows for user creation with different access levels.

- *User* is the regular (non-special) access level which is able to borrow a book
- *Librarian* is a *User*, plus it can create new books and cancel existing borrows
- *Admin* is a *Librarian*, plus it has unlimited access to all functionality

*Librarian* and above can only be created by the *Admin* type.

#### Login - `POST /session`

Generates an access token given credentials. The authentication strategy is primitive
and crude (see [Missing for production](#missing-for-production)).

### Book API

#### Get - `GET /book/:title`

#### List - `GET /books/:query`

Where `{query}` matches by the book's title.

#### Create - `POST /book`

Requires at least **Librarian** access level

#### Borrow - `POST /book/:title/borrow`

Lends a book to a given [user](#user-api) for a given length of time.

#### End borrow - `DELETE /book/:title/borrow`

Finishes the borrow for a given [user](#user-api).

## 2.2 Client features <a name="client-features"></a>

[![library](https://img.youtube.com/vi/tR9ohvZu6Qo/0.jpg)](https://www.youtube.com/watch?v=tR9ohvZu6Qo)

A Cypress recording of all features is available [here](https://www.youtube.com/watch?v=tR9ohvZu6Qo),
which should cover the whole [API](#user-api). Descriptions of each individual step executed in the recording can be found in the [integration test file](https://github.com/resolritter/library/blob/master/web_ui/integration/cypress/integration/borrow.ts#L13).

## 2.3 Infrastructural server features <a name="infrastructural-server-features"></a>

- Route protection gated by specific access levels per route.
- PgSQL database is ran in a container, which allows for spawning multiple environments for parallel integration test runs.
- Logging for all message-passing and errors in actors.
- Integration testing for all public APIs with logging of the message history to [snapshots](https://github.com/resolritter/library/tree/master/server/tests/snapshots).
- Execution is driven by a Bash script instead of a Makefile, which is powerful and flexible enough to be used for starting everything.

# 3. Technology stack <a name="tech-stack"></a>

- Rust is used for the server's application code.

- JavaScript is used for the UI.

- TypeScript for the integration test suite with Cypress.

- Bash is used everywhere else as the "glue" language.

## Server

The server is comprised of a Rest API ran in [Bastion](#bastion) and served through [Tide](#tide).

### Actors - [Bastion](https://github.com/bastion-rs/bastion) <a name="bastion"></a>

The Bastion Prelude compounds the functionality of [several other crates](https://github.com/bastion-rs/bastion#bastion-ecosystem) with a runtime primed for redundancy and recovery. Effectively, the supervisor will spawn each actor's closure into a dedicated thread pool, thus isolating crashes from other threads. The execution is wrapped in such a way (through [LightProc](https://github.com/bastion-rs/bastion/tree/master/src/lightproc)) that, once there is a crash, the wrapper will react to the panic by sending a message to the parent supervisor, which freshly restarts the closure in a new thread. [Advantages](https://github.com/bastion-rs/bastion#why-bastion) and robustness aside, adopting this strategy streamlines the communication pattern for the system as whole, which I'll come back to in the [How it works](#how-it-works) section.

### Messages - [crossbeam-channel](https://docs.rs/crate/crossbeam-channel)

Since Bastion spawns each actor in a separate thread, the communication between them is done by sending one-shot (bounded with one capacity) crossbeam channels through messages, which will eventually reach the target actor's inbox and be replied to (or timed out) in the sender's end; more details in the [How it works](#how-it-works) section.

### Serving requests - [Tide](https://docs.rs/crate/tide) <a name="tide"></a>

Tide had the most straigthforward and well-thought-out API among the available options. I simply wanted a "facade" bridge for getting messages into the actor system, which tide does well (see [here](https://github.com/resolritter/library/blob/c362ad10167740bffacd09bccf275353087ce162/server/src/main.rs#L208)). Importantly, it does so without cognitive burden; contrasting it with another popular option, [warp](https://crates.io/crates/warp)'s filter chain + rejections seemed like unneeded ceremony for what I had in mind. It also didn't help that I ended up reading [some](https://github.com/seanmonstar/warp/issues/712) [issues](https://github.com/seanmonstar/warp/issues/168) which discouraged my interest in it. Effectively, as it was chosen from the very start that everything will be processed through actors, I merely want the web server to hand me a request to turn into a message, process, then respond with something; it should be no more difficult than that, nor does it need to do more than that. With Tide there's no inconvenience in responding with an arbitrary error, either, at any given point, due to how `tide::Result` and `tide::Error::from_str` work together.

Comparatively, Tide is younger and less "solved" than others (for instance, many issues in the tracker are [labeled as design](https://github.com/http-rs/tide/issues?q=is%3Aissue+is%3Aopen+label%3Adesign+)). Neverthereless, I didn't feel like it would become unmanageable if this project's features were to increase in complexity, which is ultimately why it was chosen.

### Database - [sqlx](https://github.com/launchbadge/sqlx) + [refinery](https://docs.rs/refinery/0.4.0/refinery/)

A good duo for plain ol' SQL without ORM cumbersomeness. Have **refinery** handle [migrations](https://github.com/resolritter/library/blob/master/server/src/migrations) and **sqlx** [queries](https://github.com/resolritter/library/blob/c362ad10167740bffacd09bccf275353087ce162/server/src/resources/book.rs#L218) both expressed directly in SQL feels much more predictable and productive.

## Server test snapshotting - [insta](https://github.com/mitsuhiko/insta) + [flexi_logger](https://github.com/emabee/flexi_logger)

Testing is based in logging through **flexi_logger**, then snapshotting the logs with **insta** (see the [snapshots directory](https://github.com/resolritter/library/tree/master/server/tests/snapshots)).

## Web UI - [React](https://reactjs.org/docs/getting-started.html) + [Redux](https://react-redux.js.org/)

The UI is implemented in React + Redux due to my comfort with them ([as my other projects show](https://github.com/resolritter/resolritter/blob/master/readme.md), but also my professional experience). [Seed](https://github.com/seed-rs/seed-quickstart) was considered initially and seems fine, but comparatively not as productive of a choice due to my front-end experience with other technologies.

## Web UI tests - [Cypress](https://www.cypress.io/)

Integration testing for the UI. While the server integration ensures the APIs are working, Cypress tests ensures the API is being used properly in the front-end.

## 4. Installing and running <a name="installing-and-running"></a>

### Database

If PostgreSQL is already running at `localhost:5432` and the `$USER` role is set up already, then Docker is not needed - in that case it'll just use your existing service instance.

Otherwise, the Docker setup requires `docker-compose` and can be ran with `run.sh db`.

### Server

Requires the database to be up. Run through either `run.sh` or `cd server && cargo run`.

### Server integration tests <a name="server-integration-tests"></a>

By virtue of the testing infrastructure leveraging lots of Unix-specific programs, it's assumed that the setup, as is, likewise, only works on Linux. The used executables should already be available in your distribution:

```
bash
pstree
awk
head
tail
kill
flock
nc
ss
```

Additionally `docker-compose` and `memcached` should be installed for, respectively, spawning instances and coordinating resources between them. Their purpose is explained in [How testing is done](#how-testing-is-done).

Once everything is set up, run

1. Spawn the test database

`./run_sh test_db`

2. Run the memcached daemon

`memcached -d`

3. Run cargo test

`cargo test`

### Web UI integration tests

Needs every dependency mentioned in [Server integration tests](#server-integration-tests) plus Node.js ([see package.json](https://github.com/resolritter/library/blob/master/web_ui/integration/package.json)). The tests can be ran with `node main.js` ([see main.js](https://github.com/resolritter/library/blob/master/web_ui/integration/main.js)).

# How it works

## Nomenclature <a name="nomenclature"></a>

Entity refers to an entity in traditional sense ([source](https://en.wikipedia.org/wiki/Entity%E2%80%93relationship_model)). For this implementation, entities map 1-to-1 with actors, i.e. the Book *actor* handles all messages related to the Book *entity*.

Resource simply is the entity (which is the data representation) as a "controller" (which offers API functionality for a certain entity); the concept was popularized by the [Rails](https://guides.rubyonrails.org/routing.html#resource-routing-the-rails-default) framework.

## entities <a name="entities-crate"></a>

A crate which primarily hosts the data-centric aspects of the application, hence the name "entities". The structures' ([source](https://github.com/resolritter/library/blob/master/entities/src/data.rs)) names follow a pattern dictated by the macros, as it'll be shown in the following sections.

## actor_msg_resources <a name="actor_msg_resources"></a>

The **actor_msg_resources** ([source](https://github.com/resolritter/library/blob/c7214bd8e32d39a025538be6f81ff574ef6eb296/actor_msg_resources/src/lib.rs#L108) and [usage](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/server/src/messages.rs#L11)) macro has the following roles

1. Wrap messages in a default structure which carries useful values along with the message's content (for instance, a reference to the database pool).

2. Generate enum wrappers for the all message types.

3. Define a `OnceCell` named after the actor's name capitalized (e.g. `BOOK` for the Book actor). This cell will be initialized to an empty RwLock when the app is set up ([source](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/server/src/main.rs#L174)) and later replaced with a channel to the actors when they're spawned ([source](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/endpoint_actor/src/lib.rs#L118)). <a name="once-cell-mention"></a>

By virtue of code generation, this macro enforces a predictable naming convention for messages. For instance, the following

`actor_msg_resources::generate!(User, [(Login, User)])`

Specified as

`actor_msg_resources::generate!(#ACTOR, [(#MESSAGE_VARIANT, #REPLY)])`

Expands to

```
    pub struct UserLoginMsg {
        pub reply: crossbeam_channel::Sender<Option<crate::resources::ResponseData<User>>>,
        pub payload: entities::UserLoginPayload,
        pub db_pool: &'static sqlx::PgPool,
    }

    pub enum UserMsg {
        Login(UserLoginMsg),
    }
```

Clearly there's a trend in naming there

`User` `Msg` enum,

with an `Login` variant,

which wraps a `User` `Login` `Msg`.

The payload field references `User` `Login` `Payload` which is yet another conventionally-named struct from the [entitities](#entities-crate) crate.

Now that the messages' structures have been taken care of, the HTTP endpoints can be built.

## actor_request_handler <a name="actor_request_handler"></a>

The **actor_response_handler** macro ([source](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/actor_response_handler/src/lib.rs#L116)) expands to a function which handles

1. Parsing a request into a message
2. Sending the message to its designated actor
3. Wait for the reply message
4. Mount the response body and return it

The parser function's name is dictated by convention. For a macro

```
actor_request_handler::generate!({
    name: login,
    actor: User,
    response_type: User,
    tag: Login
});
```

Since it's named `login`, there should be an accompanying function `extract_` `login` to parse the HTTP request ([example](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/server/src/resources/user.rs#L57)). Parser errors are easily enough handled through [Tide](#tide), if any.

The function generated by this macro will be later used as an HTTP endpoint. Now that there's a way to extract the messages, an actor can be created for receiving and replying to them.

## endpoint_actor <a name="endpoint_actor"></a>

Given that Bastion's API for spawning actors is

```
children
    .with_exec(move |ctx: BastionContext| async move {
        // actor code
    })
```


The first setup for any given actor will be registering his own channel of communication through a lock which, as mentioned in the [previous section](#once-cell-mention), is globally reachable through a `OnceCell` (note: reliance on such mechanism means that redundancy cannot be achieved using this approach **as it currently is implemented**, although it technically is possible). Availability through the `OnceCell` is done for the sake of making this actor's channel discoverable, always, whenever it comes up (it might crash at some point and [that's fine](#let-it-crash)).

**endpoint_actor** expands to the [repetitive code](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/endpoint_actor/src/lib.rs#L124) you'd normally have to write by hand, which is unwrapping the enums and forwarding the payload ([source](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/endpoint_actor/src/lib.rs#L97)) to the function which will process it.

As an example

```
endpoint_actor::generate!({ actor: User }, {
    Login: create_session,
});
```

Specified as

```
endpoint_actor::generate!({ actor: #ACTOR }, {
    #MESSAGE_VARIANT: #MESSAGE_HANDLER,
    ...
});
```

The expansion defines an actor which will handle the HTTP requests parsed in [request handlers](#actor_request_handler). Now we should have everything needed for serving a response.

## Recap - Putting it all together <a name="recap"></a>

In case you wanted to expand the API with a new endpoint, the following steps would be taken

1. Edit the [messages](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/server/src/messages.rs#L11) module where you'll either add another `(#MESSAGE_VARIANT, #REPLY)` tuple to one of the existing actor definitions or create a new one.

```diff
actor_msg_resources::generate!(
    Book,
    [
+       (Create, Book),
    ]
);
```

2. Define new structs in the [entities crate](#entities-crate) following the naming conventions already explained before

```diff
+ pub struct BookCreatePayload {
+    pub title: String,
+    pub access_token: String,
+ }
```

3. Create or use an existing [resource](https://github.com/resolritter/library/tree/master/server/src/resources) for handling the new message.

4. Define a new request handler with [actor_request_handler](#actor_request_handler)

```diff
+ async fn extract_post(req: &mut Request<ServerState>) -> tide::Result<BookCreatePayload> {
+    // extract the payload
+ }
+ actor_request_handler::generate!(Config {
+    name: post,
+    actor: Book,
+    response_type: Book,
+    tag: Create
+ });
```

Notice the convention between `name: post` and `extract_post`.

5. Define a new message handler in the body of [endpoint_actor](#endpoint_actor)

```diff
+ pub async fn create(msg: &BookCreateMsg) -> Result<ResponseData<Book>, sqlx::Error> {
+     // create the book
+ }

endpoint_actor::generate!({ actor: Book }, {
    GetByTitle: get_by_title,
    LeaseByTitle: lease_by_id,
    EndLoanByTitle: end_loan_by_title,
+   Create: create,
});
```

6. Create the route

```diff
+ server
+     .at(format!(book_route!(), ":title").as_str())
+     .post(resources::book::post);
```

## Finally, start the app <a name="start"></a>

- [Connect to the database](https://github.com/resolritter/library/blob/b61a4625a2ec5fd0d62034e876ef66f75d17a7db/server/src/main.rs#L156)
- [Run migrations](https://github.com/resolritter/library/blob/b61a4625a2ec5fd0d62034e876ef66f75d17a7db/server/src/main.rs#L260)
- [Define the web servers' routes](https://github.com/resolritter/library/blob/b61a4625a2ec5fd0d62034e876ef66f75d17a7db/server/src/main.rs#L218)
- [Initialize the supervision tree](https://github.com/resolritter/library/blob/b61a4625a2ec5fd0d62034e876ef66f75d17a7db/server/src/main.rs#L185)

## Appendix - Advantagens of actor systems <a name="advantages-of-actor-systems"></a>

There are many resources describing the advantages of actor models (e.g. the [Akka guide](https://doc.akka.io/docs/akka/current/typed/guide/introduction.html) goes in-depth on the whole topic), but I hadn't heard of the following highlights before working in this project
  
- **Message-first**: the actor model encourages one to model around simple plain old structures which can be sent easily across threads, as opposed to massive objects which host a lot of context and might hold dependencies to non-thread-safe elements.
- **Logging**: since execution is driven through messages, it's extremely easy to catpure the flow of execution at the message handler instead of remebering to add custom log directives at arbitrary points in the code.
- **Inspectable**: even if a component suddenly breaks before it's replied to, the lingering message will still likely offer some insight given that execution is driven by the data within them, instead of being arbitrarily spread through the execution of the main thread, which is liable to failure.

## Appendix - Let it crash <a name="let-it-crash"></a>

Resilience is the main selling point for wanting to model your application like this. Having a fault-tolerant runtime means you can reliably `unwrap` in places where you are expecting some invariant to hold up, except unwrapping will **not** crash your whole app, thus you don't need to program defensively against errors - let it crash and recover itself! Spend development time on testing instead of trying to architect around failure at runtime.

# How testing is done <a name="how-testing-is-done"></a>

The [Server integration tests](#server-integration-tests) section had a list of all the Linux utilities needed. How does it all come together?

It starts from the Bash script, `run.sh`. A command may have dependencies ([example](https://github.com/resolritter/library/blob/53d7c0bf9aa5ba5f521dc7fb3ce9ecde2dcf6646/run.sh#L97)); for instance, when logging is enabled, the logging folder has to be created before the program is run - that's just how the libraries work. The Bash script therefore serves as a wrapper and general way to configure and set up all the programs it can run.

Integration tests need both a clean database and a fresh server instance in order to run; accordingly, both need processes need open ports to bind to, which is where `ss` comes in handy for figuring out which ports are currently in use.

For the database, a dockerized PostgreSQL instance is spawned especifically for tests with `run.sh test_db`. It's useful to have this dedicated container in order to avoid accumulating test databases in the actual work instance, plus it also means that the volume can be completely discarded when the container is finished. The port being used will be written to `$TEST_DB_PORT_FILE`, a file which will be automatically read when the tests are ran. The databases used for integration tests will, therefore, all be created in this specific container.

`memcached` plays a part in coordinating resource acquisition between tests. Since all tests are ran in parallel, some test might want to spin up the server in a port which would be taken by another, thus making them conflict and fail. Synchronizing the reads (*is the port free?*) and writes (*I've exclusively acquired X port for the test*) is done using `memcached` because the disk storage is, at least on my PC, not fast enough (another approach could've been to write/read the used ports to a file). Note that `ss` does not work here since it only lists ports *currently* being used, but due the tests starting all at the same through parallel execution, individual instances don't bind fast enough for it to be caught by `ss`; a "lock-and-reserve-ahead-of-time" mechanism on a in-memory database is, therefore, needed, since it doesn't suffer from unpredictable flushing and speed disadvantagens file disk accesses have.

Further, for achieving true exclusive read/write synchronization for port acquisition, `flock` wraps around `memcached` (`nc` is used for talking to `memcached`) so that processes don't try to both write/read at the same time. Having synchronized the port acquisition, tests can be independently ran and also log in parallel. As already mentioned in the [advantages](advantages-of-actor-systems), it's pretty easy to know exactly what's being executed by snapshotting the logs (see the [snapshot directory](https://github.com/resolritter/library/tree/master/server/tests/snapshots)).

Finally, when the tests need to be teared down, that's when the other utilities come in. Because execution is driven by the Bash script, simply killing the process itself would only kill the "wrapper", but not all of the processes spawned indirectly. In which case, `pstree` is used for finding the server's PID underneath of the wrapper, then terminated with `kill`.

# Missing for production <a name="missing-for-production"></a>

This project doesn't aim to show much of what would be needed to have in a real-world application.
That being said, *if it were*, some things would obviously be missing, so they'll be listed here.

## Completeness requirements

- Authentication does not include password, which would not work.
- Title is used as primary key for books, but of course this wouldn't be acceptable normally.
- Books can only be lent for a full week, but the timeline could be customizable.
- Books are automatically considered available when their lease time expires, thus the current system doesn't account for lateness.

## Nice-to-have features

- See a user's history of borrowing.
- See a book's history of borrowing.
- Plotted metrics (% of books late, % probability of it being late, etc) in some sort of Admin dashboard.
- Searching and filtering books in the UI.

## Token refreshing and invalidation

Currently tokens are issued once and don't degrade, ever.

## Have issued tokens for multiple devices

Currently we have the single `access_token` field in the User entity which wouldn't scale well with multiple devices.

## Uploading logs to the cloud

Currently errors are logged to the file system, but not reported in any manner to some provider in the cloud.

## Other

The following are self-explanatory

- PostgreSQL setup has only been proven to work without password.
- Lacking CI Setup.
- No API specification (e.g. Swagger).
- No verification of the user profile's payload as received from the backend.
- Cached profile information in the front-end is never invalidated or degraded.
