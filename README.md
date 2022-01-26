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

This project is meant to be a demonstration of how to design, architect and
implement a full-stack application using an actor system. It was created for
the purpose of exercising software engineering through a concrete product idea.
The requirements are intentionally shallow so that we will not divert focus to
domain-specific details which are not relevant to aforementioned idea of
showcasing the implementation.

Further, the implementation does not aim to be "production-ready" (see the
[Missing for production](#missing-for-production) section).

# Why "Library"  <a name="why-library"></a>

This project is named "Library" because it implements a basic system for a
library (as in a "book library") where users will be able to borrow books and
return them later. This theme is simple and restricted enough that it allows
for us to focus on the software engineering aspects without having to bother
with menial domain-specific details -
[that is not the purpose of this project](#purpose).

## 2.1 User-facing server features <a name="product-features"></a>

Following the traditional Rest API principles, the routes are grouped by
[Resources](https://guides.rubyonrails.org/routing.html#resource-routing-the-rails-default)
with semantic hierarchical nesting by interplay of slashes and parameters. e.g.
`/book/{id}/borrow`.

### User API <a name="user-api"></a>

#### Creation - `POST /user`

Allows for user creation with custom access levels.

- User is the regular (non-special) access level which is able to borrow a book
- Librarian is a User, plus it can create new books and cancel existing borrows
- Admin is a Librarian, plus it has unlimited access to all functionality

Librarian and above can only be created by the *Admin* type.

#### Login - `POST /session`

Generates an access token with given credentials. The authentication strategy
is primitive and crude (see [Missing for production](#missing-for-production)).

### Book API

#### Get - `GET /book/:title`

#### List - `GET /books/:query`

Where `{query}` matches by the book's title.

#### Create - `POST /book`

Requires at least Librarian access level

#### Borrow - `POST /book/:title/borrow`

Lends a book to a given [user](#user-api) for a given length of time.

#### End borrow - `DELETE /book/:title/borrow`

Finishes the borrow for a given [user](#user-api).

## 2.2 Client features <a name="client-features"></a>

[![library](https://img.youtube.com/vi/tR9ohvZu6Qo/0.jpg)](https://www.youtube.com/watch?v=tR9ohvZu6Qo)

A Cypress recording of all the client-side features is available at
https://www.youtube.com/watch?v=tR9ohvZu6Qo. Descriptions of each individual
step executed in the recording can be found in the [integration test
file](./web_ui/integration/cypress/integration/borrow.ts#L13).

## 2.3 Infrastructural server features <a name="infrastructural-server-features"></a>

- Route protection gated by specific access levels per route.
- PgSQL database is ran in a container, which allows for spawning multiple
  environments for parallel integration test runs.
- Logging for all message-passing and errors in actors.
- Integration testing for all public APIs with logging of the message history
  to [snapshots](./server/tests/snapshots).
- Execution is driven by a Bash script instead of a Makefile, which is powerful
  and flexible enough to be used for starting everything.

# 3. Technology stack <a name="tech-stack"></a>

- Rust is used for the server's application code.
- JavaScript is used for the UI.
- TypeScript for the integration test suite with Cypress.
- Bash is used everywhere else as the "glue" language.

## Server

The server is comprised of a Rest API ran inside of a [Bastion](#bastion)
supervisor and served through [Tide](#tide).

### Actors - [Bastion](https://github.com/bastion-rs/bastion) <a name="bastion"></a>

The Bastion Prelude compounds the functionality of [several other
crates](https://github.com/bastion-rs/bastion/tree/48d4c73edbf149e9048dedf7cd5c1e890788709c#bastion-ecosystem)
for a runtime primed for reliability and recovery. Effectively, the supervisor
will spawn each actor's thread in a dedicated thread pool, thus isolating
crashes between actors. The execution is wrapped through
[LightProc](https://github.com/bastion-rs/bastion/tree/48d4c73edbf149e9048dedf7cd5c1e890788709c/src/lightproc)
so that, once there is a crash, the system will react to the error by sending a
message to the parent supervisor, which freshly restarts the actor's closure in
a new thread. [Advantages](https://github.com/bastion-rs/bastion#why-bastion)
and robustness aside, adopting this strategy streamlines the communication
pattern for the system as whole, which I'll come back to in the [How it
works](#how-it-works) section.

### Messages - [crossbeam-channel](https://docs.rs/crate/crossbeam-channel)

Since Bastion spawns each actor in a separate thread, the communication between
them is made possible by sending one-shot (bounded with one capacity) crossbeam
channels through messages, which will eventually reach the target actor's inbox
and be replied to; more details in the [How it works](#how-it-works) section.

### Serving requests - [Tide](https://docs.rs/crate/tide) <a name="tide"></a>

When I started this project by late 2020, Tide had the most straigthforward and
user-friendly API among the available options. I simply wanted a "facade" for
turning HTTP requests into messages for the actors, which Tide is able to do
well enough (see
[main.rs](https://github.com/resolritter/library/blob/c362ad10167740bffacd09bccf275353087ce162/server/src/main.rs#L208));
notably, it does so without much cognitive burden and ceremony - the API seems
simple enough to use, unlike other options I found at the time.

I also considered [warp](https://crates.io/crates/warp) but its filter chain +
rejections model seemed like unneeded ceremony for what I have in mind. Some
other issues regarding the public API
([1](https://github.com/seanmonstar/warp/issues/712),
[2](https://github.com/seanmonstar/warp/issues/168)) further discouraged my
interest in it.

### Database - [sqlx](https://github.com/launchbadge/sqlx) + [refinery](https://docs.rs/refinery/0.4.0/refinery/)

A sqlx + refinery lets me work with the SQL dialect directly. I much prefer
this kind of approach rather than any ORM API.

refinery handles [migrations](./server/src/migrations) while sqlx is used for
[queries](https://github.com/resolritter/library/blob/c362ad10167740bffacd09bccf275353087ce162/server/src/resources/book.rs#L218). I find this setup to be much more productive than trying to mentally translate
SQL into ORM APIs.

## Server test snapshotting - [insta](https://github.com/mitsuhiko/insta) + [flexi_logger](https://github.com/emabee/flexi_logger)

Testing is done by collecting the logs through flexi_logger, then snapshotting
them with insta (see the
[snapshots directory](./server/tests/snapshots)).

## Web UI - [React](https://reactjs.org/docs/getting-started.html) + [Redux](https://react-redux.js.org/)

The UI is implemented in React + Redux because that's what I am most
comfortable with, as corroborated by [my other projects
](https://github.com/resolritter/resolritter/blob/master/readme.md) and
professional experience.

I had looked into other Rust-based frontend solutions such as
[Seed](https://github.com/seed-rs/seed-quickstart) but I judged it wouldn't
as productive to use them since

1. I have experience as a front-end developer using JS
2. Their APIs were not mature at the time

## Web UI tests - [Cypress](https://www.cypress.io/)

Integration testing for the UI is done through Cypress.

While the server integration ensures the APIs are working, Cypress verifies
that the API is being used properly through the front-end features.

## 4. Installing and running <a name="installing-and-running"></a>

### Database

If PostgreSQL is already running at `localhost:5432` and the `$USER` role is
set up already, then Docker is not needed - in that case it'll just use your
existing service instance.

Otherwise, the Docker setup requires `docker-compose` and can be ran with
`run db`.

### Server

Requires the database to be up.

Start it with `run server`.

### Server integration tests <a name="server-integration-tests"></a>

Run them with `run integration_tests`.

By virtue of the testing infrastructure leveraging lots of Unix-specific
programs, it's assumed that the setup, as is, likewise, only works on Linux.
The used executables should already be available in your distribution:

```
bash
pstree
awk
head
tail
kill
pkill
flock
ss
```

Additionally `docker-compose`  should be installed for spawning test instances
and coordinating resources between them. Their purpose is explained in [How
testing is done](#how-testing-is-done).

### Web UI integration tests

Needs every dependency mentioned in
[Server integration tests](#server-integration-tests) plus Node.js
([see package.json](./web_ui/integration/package.json)). The tests can be ran
with `node main.js` ([see main.js](./web_ui/integration/main.js)).

# How it works

## Nomenclature <a name="nomenclature"></a>

Entity refers to an entity in the traditional sense
([ERM](https://en.wikipedia.org/wiki/Entity%E2%80%93relationship_model)).
For this implementation, entities map 1-to-1 with actors, i.e. the Book *actor*
handles all messages related to the Book *entity*.

Resource simply is the entity (which is the data representation) as a
"controller" (which offers API functionality for a certain entity). This
concept was popularized by the
[Rails](https://guides.rubyonrails.org/routing.html#resource-routing-the-rails-default)
framework.

## entities <a name="entities-crate"></a>

A crate which primarily hosts the data-centric aspects of the application,
hence the name "entities". The structures' ([source](./entities/src/data.rs))
names follow a pattern dictated by the macros, as it'll be shown in the
following sections.

## actor_msg_resources <a name="actor_msg_resources"></a>

The `actor_msg_resources`
([source](https://github.com/resolritter/library/blob/c7214bd8e32d39a025538be6f81ff574ef6eb296/actor_msg_resources/src/lib.rs#L108)
and
[usage](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/server/src/messages.rs#L11))
macro has the following roles:

1. Wrap messages in a default structure which carries useful values along with
  the message's content (for instance, a reference to the database pool).

2. Generate enum wrappers for the all message types.

3. Define a `OnceCell` named after the actor's name capitalized (e.g. `BOOK`
  for the Book actor). This cell will be initialized to an empty `RwLock` when
  the app is set up
  ([source](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/server/src/main.rs#L174))
  and later replaced with a channel to the actors when they're spawned
  ([source](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/endpoint_actor/src/lib.rs#L118)). <a name="once-cell-mention"></a>

By virtue of code generation, this macro enforces a predictable naming
convention for messages. For instance, the following:

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

Clearly there's a trend in naming there:

`User` `Msg` enum,

with an `Login` variant,

which wraps a `User` `Login` `Msg`.

The payload field references `User` `Login` `Payload` which is yet another
conventionally-named struct from the [entitities](#entities-crate) crate.

Now that the messages' structures have been taken care of, the HTTP endpoints can be built.

## actor_request_handler <a name="actor_request_handler"></a>

The `actor_response_handler` macro
([source](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/actor_response_handler/src/lib.rs#L116))
expands to a function which handles

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

Since it's named `login`, there should be an accompanying function `extract_`
`login` to parse the HTTP request
([example](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/server/src/resources/user.rs#L57)).
Parser errors are easily enough handled through [Tide](#tide), if any.

The function generated by this macro will be later used as an HTTP endpoint.
Now that there's a way to extract the messages, an actor can be created for
receiving and replying to them.

## endpoint_actor <a name="endpoint_actor"></a>

Given that Bastion's API for spawning actors is

```
children
    .with_exec(move |ctx: BastionContext| async move {
        // actor code
    })
```

The first step for any given actor will be registering his own channel of
communication. As mentioned in the [previous section](#once-cell-mention),
their channel will be globally reachable through a `OnceCell` (note: reliance
on such mechanism means that redundancy cannot be achieved using this approach
**as it currently is implemented**, although it technically is possible).
Availability through the `OnceCell` is done for the sake of making this actor's
channel discoverable, always, whenever it comes up (it might crash at some
point and [that's fine](#let-it-crash)).

`endpoint_actor` expands to the
[repetitive code](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/endpoint_actor/src/lib.rs#L124)
one would normally have to write by hand, which is unwrapping the enums and
forwarding the payload
([source](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/endpoint_actor/src/lib.rs#L97))
to the function which will process it.

As an example:

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

The expansion defines an actor which will handle the HTTP requests parsed in
[request handlers](#actor_request_handler). Now we should have everything
needed for serving a response.

## Recap - Putting it all together <a name="recap"></a>

In case you want to expand the API for a new endpoint, the following steps
would be taken

1. Edit the
  [messages](https://github.com/resolritter/library/blob/2920b06de3762f3a083d99498596d48f0ad3ea83/server/src/messages.rs#L11)
  module where you'll either add another `(#MESSAGE_VARIANT, #REPLY)` tuple to
  one of the existing actor definitions or create a new one.

```diff
actor_msg_resources::generate!(
    Book,
    [
+       (Create, Book),
    ]
);
```

2. Define new structs in the [entities crate](#entities-crate) following the
  naming conventions already explained before

```diff
+ pub struct BookCreatePayload {
+    pub title: String,
+    pub access_token: String,
+ }
```

3. Create or use an existing [resource](./server/src/resources) for handling
  the new message.

4. Define a new request handler with
  [actor_request_handler](#actor_request_handler)

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

There are many resources describing the advantages of actor models (e.g. the
[Akka guide](https://doc.akka.io/docs/akka/current/typed/guide/introduction.html)
goes in-depth on the whole topic), but I hadn't heard of the following
highlights before working in this project

- **Message-first**: the actor model encourages one to model around simple
  plain old structures which can be sent easily across threads, as opposed to
  massive objects which host a lot of context and might hold dependencies to
  non-thread-safe elements.
- **Logging**: since execution is driven through messages, it's extremely easy
  to catpure the flow of execution at the message handler instead of remebering
  to add custom log directives at arbitrary points in the code.
- **Inspectable**: even if a component suddenly breaks before it's replied to,
  the lingering message will still likely offer some insight given that
  execution is driven by the data within them, instead of being arbitrarily
  spread through the execution of the main thread, which is liable to failure.

## Appendix - Let it crash <a name="let-it-crash"></a>

Resilience is the main selling point for wanting to model an application with
actors and supervisors. Having a fault-tolerant runtime motivates the use of
`unwrap` in places where you are expecting some invariant to hold up because
doing so will **not** crash your whole app - instead only the actor will crash
and then be recreated by the supervisor. This allows for avoiding "defensive
programming" against against errors: instead, let it crash and recover itself;
spend development time on testing instead of trying to defensively architect
around unexpected failures at runtime.

# How testing is done <a name="how-testing-is-done"></a>

The [Server integration tests](#server-integration-tests) section had a list of
all the Linux utilities needed. How does it all come together?

It starts from the Bash script, `run`. A command may have dependencies
([example](https://github.com/resolritter/library/blob/53d7c0bf9aa5ba5f521dc7fb3ce9ecde2dcf6646/run.sh#L97));
for instance, when logging is enabled, the logging folder has to be created
before the program is run. The Bash script therefore serves as a wrapper and
general way to configure and set up all the programs it can run.

Integration tests need both a clean database and a fresh server instance in
order to run; accordingly, both need processes need open ports to bind to,
which is where `ss` comes in handy for figuring out which ports are currently
in use.

For the database, a dockerized PostgreSQL instance is spawned especifically for
tests with `run test_db`. It's useful to have this dedicated container in
order to avoid accumulating test databases in the actual work instance, plus it
also means that the volume can be completely discarded when the container is
finished. The port being used will be written to `$TEST_DB_PORT_FILE`, a file
which will be automatically read when the tests are ran. The databases used for
integration tests will, therefore, all be created in this specific container.

For ensuring exclusive system-level resources acquisition, `flock` is used as a
synchronization tool so that two different test instances do not try to bind to
the same port. Ensuring exclusive the port acquisition means that tests can be
independently ran in parallel. As aforementioned in the
[advantages](advantages-of-actor-systems), it's pretty easy to know exactly
what's being executed by snapshotting the logs (see the [snapshot
directory](./server/tests/snapshots)).

Finally, the remaining utilities come in once on the teardown phase. Because
execution is driven by the Bash script, simply killing the process would only
kill the "wrapper", but not all the processes spawned indirectly. `pstree` is
used for finding the server's PID underneath of the wrapper, which is then
terminated with `kill`.

# Missing for production <a name="missing-for-production"></a>

This project [purposefully does not aim to become a production-ready
application](#purpose), but *if it did*, we'd be interested in implementing the
features from this section.

## Completeness requirements

- Authentication does not include password, which would not work.
- Title is used as primary key for books, but of course this wouldn't be
  acceptable normally.
- Books can only be lent for a full week, but the timeline could be
  customizable.
- Books are automatically considered available when their lease time expires,
  thus the current system doesn't account for lateness.

## Nice-to-have features

- See a user's history of borrowing.
- See a book's history of borrowing.
- Plotted metrics (% of books late, % probability of it being late, etc) in
  some sort of Admin dashboard.
- Searching and filtering books in the UI.

## Token refreshing and invalidation

Currently tokens are issued once and don't degrade, ever.

## Have issued tokens for multiple devices

Currently we have the single `access_token` field in the User entity which
wouldn't scale well with multiple devices.

## Uploading logs to the cloud

Currently errors are logged to the file system, but not reported in any manner
to some provider in the cloud.

## Other

The following are self-explanatory

- PostgreSQL setup has only been proven to work without password.
- Lacking CI Setup.
- No API specification (e.g. OpenAPI).
- No verification of the user profile's payload as received from the backend.
- Cached profile information in the front-end is never invalidated or degraded.
