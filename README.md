# Chat app built with Rust to learn the language and HTTP libraries

This was built from https://blog.logrocket.com/real-time-chat-app-rust-react/ and explores a lot of things, but it is not incorrect and does not run.

It was a good exercise in understanding using Rust as a backend with a React UI, but is not usable in the current form.

One of the biggest issues is that the tutorial tells you to use the diesel CLI, which does not seem compatible with the codebase, so there are a number of errors related to that.

Here is the output of all the errors:
```
warning: rust-chat v0.1.0 (/Users/mb/code/rust-chat) ignoring invalid dependency `diesel_cli_ext` which is missing a lib target
   Compiling rust-chat v0.1.0 (/Users/mb/code/rust-chat)
error: crate name using dashes are not valid in `extern crate` statements
  --> src/main.rs:10:14
   |
10 | extern crate diesel-cli;
   |              ^^^^^^^^^^ dash-separated idents are not valid
   |
help: if the original crate name uses dashes you need to use underscores in the code
   |
10 | extern crate diesel_cli;
   |                    ~

error[E0463]: can't find crate for `diesel_cli`
  --> src/main.rs:10:1
   |
10 | extern crate diesel-cli;
   | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
 --> src/main.rs:6:5
  |
6 | use diesel::{
  |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
 --> src/db.rs:2:5
  |
2 | use diesel::prelude::*;
  |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
 --> src/routes.rs:6:5
  |
6 | use diesel::{
  |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
 --> src/session.rs:6:5
  |
6 | use diesel::{
  |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0433]: failed to resolve: could not find `users` in `schema`
  --> src/db.rs:18:24
   |
18 |     use crate::schema::users::dsl::*;
   |                        ^^^^^ could not find `users` in `schema`

error[E0433]: failed to resolve: could not find `conversations` in `schema`
  --> src/db.rs:32:24
   |
32 |     use crate::schema::conversations::dsl::*;
   |                        ^^^^^^^^^^^^^ could not find `conversations` in `schema`

error[E0432]: unresolved import `diesel`
 --> src/models.rs:2:5
  |
2 | use diesel::{Queryable, Insertable};
  |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0432]: unresolved import `actix_cors`
 --> src/main.rs:3:5
  |
3 | use actix_cors::Cors;
  |     ^^^^^^^^^^ use of undeclared crate or module `actix_cors`

error[E0432]: unresolved import `diesel`
 --> src/main.rs:6:5
  |
6 | use diesel::{
  |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0432]: unresolved import `chrono`
 --> src/db.rs:1:5
  |
1 | use chrono::{DateTime, Utc};
  |     ^^^^^^ use of undeclared crate or module `chrono`

error[E0432]: unresolved import `diesel`
 --> src/routes.rs:6:5
  |
6 | use diesel::{
  |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0432]: unresolved import `diesel`
 --> src/session.rs:6:5
  |
6 | use diesel::{
  |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
 --> src/schema.rs:1:1
  |
1 | diesel::table! {
  | ^^^^^^ use of undeclared crate or module `diesel`

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
  --> src/schema.rs:10:1
   |
10 | diesel::table! {
   | ^^^^^^ use of undeclared crate or module `diesel`

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
  --> src/schema.rs:19:1
   |
19 | diesel::table! {
   | ^^^^^^ use of undeclared crate or module `diesel`

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
  --> src/schema.rs:27:1
   |
27 | diesel::allow_tables_to_appear_in_same_query!(
   | ^^^^^^ use of undeclared crate or module `diesel`

error: cannot find attribute `table_name` in this scope
 --> src/models.rs:6:3
  |
6 | #[table_name = "users"]
  |   ^^^^^^^^^^

error: cannot find attribute `table_name` in this scope
  --> src/models.rs:13:3
   |
13 | #[table_name = "conversations"]
   |   ^^^^^^^^^^

error: cannot find attribute `table_name` in this scope
  --> src/models.rs:22:3
   |
22 | #[table_name = "rooms"]
   |   ^^^^^^^^^^

error: cannot find attribute `post` in this scope
 --> src/users.rs:1:3
  |
1 | #[post("/users/create")]
  |   ^^^^
  |
help: consider importing this attribute macro
  |
1 + use actix_web::post;
  |

error: cannot find attribute `get` in this scope
  --> src/users.rs:15:3
   |
15 | #[get("/users/{user_id}")]
   |   ^^^
   |
help: consider importing this attribute macro
   |
1  + use actix_web::get;
   |

error: cannot find macro `json` in this scope
  --> src/users.rs:31:13
   |
31 |             json!({
   |             ^^^^
   |
help: consider importing this macro
   |
1  + use serde_json::json;
   |

error[E0412]: cannot find type `SqliteConnection` in this scope
  --> src/db.rs:17:35
   |
17 | pub fn insert_new_user(conn: &mut SqliteConnection, nm: &str, pn: &str) -> Result<User, DbError> {
   |                                   ^^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `users` in this scope
  --> src/db.rs:24:25
   |
24 |     diesel::insert_into(users).values(&new_user).execute(conn)?;
   |                         ^^^^^ not found in this scope

error[E0412]: cannot find type `SqliteConnection` in this scope
  --> src/db.rs:29:16
   |
29 |     conn: &mut SqliteConnection,
   |                ^^^^^^^^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `conversations` in this scope
  --> src/db.rs:40:25
   |
40 |     diesel::insert_into(conversations)
   |                         ^^^^^^^^^^^^^ not found in this scope

error[E0412]: cannot find type `SqliteConnection` in this scope
  --> src/routes.rs:17:55
   |
17 | pub(crate) type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
   |                                                       ^^^^^^^^^^^^^^^^ not found in this scope
   |
help: you might be missing a type parameter
   |
17 | pub(crate) type DbPool<SqliteConnection> = r2d2::Pool<ConnectionManager<SqliteConnection>>;
   |                       ++++++++++++++++++

error[E0412]: cannot find type `SqliteConnection` in this scope
  --> src/session.rs:16:44
   |
16 | type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
   |                                            ^^^^^^^^^^^^^^^^ not found in this scope
   |
help: you might be missing a type parameter
   |
16 | type DbPool<SqliteConnection> = r2d2::Pool<ConnectionManager<SqliteConnection>>;
   |            ++++++++++++++++++

error[E0433]: failed to resolve: use of undeclared crate or module `web`
 --> src/users.rs:3:11
  |
3 |     pool: web::Data<DbPool>,
  |           ^^^ use of undeclared crate or module `web`
  |
help: consider importing one of these modules
  |
1 + use crate::web;
  |
1 + use actix_web::web;
  |

error[E0412]: cannot find type `DbPool` in this scope
 --> src/users.rs:3:21
  |
3 |     pool: web::Data<DbPool>,
  |                     ^^^^^^
  |
help: a builtin type with a similar name exists
  |
3 |     pool: web::Data<bool>,
  |                     ~~~~
help: consider importing this type alias
  |
1 + use crate::routes::DbPool;
  |

error[E0433]: failed to resolve: use of undeclared crate or module `web`
 --> src/users.rs:4:11
  |
4 |     form: web::Json<models::NewUser>,
  |           ^^^ use of undeclared crate or module `web`
  |
help: consider importing one of these modules
  |
1 + use crate::web;
  |
1 + use actix_web::web;
  |

error[E0433]: failed to resolve: use of undeclared crate or module `models`
 --> src/users.rs:4:21
  |
4 |     form: web::Json<models::NewUser>,
  |                     ^^^^^^ use of undeclared crate or module `models`
  |
help: consider importing this module
  |
1 + use crate::models;
  |

error[E0412]: cannot find type `HttpResponse` in this scope
 --> src/users.rs:5:13
  |
5 | ) -> Result<HttpResponse, Error> {
  |             ^^^^^^^^^^^^ not found in this scope
  |
help: consider importing this struct
  |
1 + use actix_web::HttpResponse;
  |

error[E0412]: cannot find type `Error` in this scope
 --> src/users.rs:5:27
  |
5 | ) -> Result<HttpResponse, Error> {
  |                           ^^^^^ not found in this scope
  |
help: consider importing one of these items
  |
1 + use crate::http::Error;
  |
1 + use std::error::Error;
  |
1 + use std::fmt::Error;
  |
1 + use std::io::Error;
  |
    and 10 other candidates

error[E0433]: failed to resolve: use of undeclared crate or module `web`
 --> src/users.rs:6:16
  |
6 |     let user = web::block(move || {
  |                ^^^ use of undeclared crate or module `web`
  |
help: consider importing one of these modules
  |
1 + use crate::web;
  |
1 + use actix_web::web;
  |

error[E0433]: failed to resolve: use of undeclared crate or module `db`
 --> src/users.rs:8:9
  |
8 |         db::insert_new_user(&mut conn, &form.username, &form.phone)
  |         ^^ use of undeclared crate or module `db`
  |
help: consider importing this module
  |
1 + use crate::db;
  |

error[E0433]: failed to resolve: use of undeclared type `HttpResponse`
  --> src/users.rs:12:8
   |
12 |     Ok(HttpResponse::Ok().json(user))
   |        ^^^^^^^^^^^^ use of undeclared type `HttpResponse`
   |
help: consider importing this struct
   |
1  + use actix_web::HttpResponse;
   |

error[E0433]: failed to resolve: use of undeclared crate or module `web`
  --> src/users.rs:17:11
   |
17 |     pool: web::Data<DbPool>,
   |           ^^^ use of undeclared crate or module `web`
   |
help: consider importing one of these modules
   |
1  + use crate::web;
   |
1  + use actix_web::web;
   |

error[E0412]: cannot find type `DbPool` in this scope
  --> src/users.rs:17:21
   |
17 |     pool: web::Data<DbPool>,
   |                     ^^^^^^
   |
help: a builtin type with a similar name exists
   |
17 |     pool: web::Data<bool>,
   |                     ~~~~
help: consider importing this type alias
   |
1  + use crate::routes::DbPool;
   |

error[E0433]: failed to resolve: use of undeclared crate or module `web`
  --> src/users.rs:18:9
   |
18 |     id: web::Path<Uuid>,
   |         ^^^ use of undeclared crate or module `web`
   |
help: consider importing one of these modules
   |
1  + use crate::web;
   |
1  + use actix_web::web;
   |

error[E0412]: cannot find type `Uuid` in this scope
  --> src/users.rs:18:19
   |
18 |     id: web::Path<Uuid>,
   |                   ^^^^ not found in this scope
   |
help: consider importing this struct
   |
1  + use uuid::Uuid;
   |

error[E0412]: cannot find type `HttpResponse` in this scope
  --> src/users.rs:19:13
   |
19 | ) -> Result<HttpResponse, Error> {
   |             ^^^^^^^^^^^^ not found in this scope
   |
help: consider importing this struct
   |
1  + use actix_web::HttpResponse;
   |

error[E0412]: cannot find type `Error` in this scope
  --> src/users.rs:19:27
   |
19 | ) -> Result<HttpResponse, Error> {
   |                           ^^^^^ not found in this scope
   |
help: consider importing one of these items
   |
1  + use crate::http::Error;
   |
1  + use std::error::Error;
   |
1  + use std::fmt::Error;
   |
1  + use std::io::Error;
   |
     and 10 other candidates

error[E0433]: failed to resolve: use of undeclared crate or module `web`
  --> src/users.rs:21:16
   |
21 |     let user = web::block(move || {
   |                ^^^ use of undeclared crate or module `web`
   |
help: consider importing one of these modules
   |
1  + use crate::web;
   |
1  + use actix_web::web;
   |

error[E0433]: failed to resolve: use of undeclared type `HttpResponse`
  --> src/users.rs:28:12
   |
28 |         Ok(HttpResponse::Ok().json(user))
   |            ^^^^^^^^^^^^ use of undeclared type `HttpResponse`
   |
help: consider importing this struct
   |
1  + use actix_web::HttpResponse;
   |

error[E0433]: failed to resolve: use of undeclared type `HttpResponse`
  --> src/users.rs:30:19
   |
30 |         let res = HttpResponse::NotFound().body(
   |                   ^^^^^^^^^^^^ use of undeclared type `HttpResponse`
   |
help: consider importing this struct
   |
1  + use actix_web::HttpResponse;
   |

error[E0412]: cannot find type `Error` in this scope
  --> src/rooms.rs:10:27
   |
10 | ) -> Result<HttpResponse, Error> {
   |                           ^^^^^ not found in this scope
   |
help: consider importing one of these items
   |
1  + use crate::http::Error;
   |
1  + use std::error::Error;
   |
1  + use std::fmt::Error;
   |
1  + use std::io::Error;
   |
     and 10 other candidates

error[E0412]: cannot find type `Error` in this scope
  --> src/rooms.rs:35:27
   |
35 | ) -> Result<HttpResponse, Error> {
   |                           ^^^^^ not found in this scope
   |
help: consider importing one of these items
   |
1  + use crate::http::Error;
   |
1  + use std::error::Error;
   |
1  + use std::fmt::Error;
   |
1  + use std::io::Error;
   |
     and 10 other candidates

error[E0412]: cannot find type `SqliteConnection` in this scope
  --> src/main.rs:23:39
   |
23 |     let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
   |                                       ^^^^^^^^^^^^^^^^ not found in this scope

warning: unused imports: `HashMap` and `HashSet`
 --> src/db.rs:4:19
  |
4 |     collections::{HashMap, HashSet},
  |                   ^^^^^^^  ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `RoomResponse` and `Room`
 --> src/db.rs:8:52
  |
8 | use crate::models::{Conversation, NewConversation, Room, RoomResponse, User};
  |                                                    ^^^^  ^^^^^^^^^^^^

warning: unused import: `crate::schema::*`
 --> src/models.rs:3:5
  |
3 | use crate::schema::*;
  |     ^^^^^^^^^^^^^^^^

warning: unused imports: `get` and `post`
 --> src/routes.rs:4:17
  |
4 | use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
  |                 ^^^  ^^^^

warning: unused import: `serde_json::json`
  --> src/routes.rs:10:5
   |
10 | use serde_json::json;
   |     ^^^^^^^^^^^^^^^^

warning: unused import: `uuid::Uuid`
  --> src/routes.rs:11:5
   |
11 | use uuid::Uuid;
   |     ^^^^^^^^^^

warning: unused import: `serde::Serialize`
 --> src/server.rs:2:5
  |
2 | use serde::Serialize;
  |     ^^^^^^^^^^^^^^^^

warning: `#[macro_use]` only has an effect on `extern crate` and modules
 --> src/main.rs:1:1
  |
1 | #[macro_use]
  | ^^^^^^^^^^^^
  |
  = note: `#[warn(unused_attributes)]` on by default

error[E0433]: failed to resolve: use of undeclared crate or module `db`
  --> src/users.rs:23:9
   |
23 |         db::find_user_by_uid(&mut conn, user_id)
   |         ^^ use of undeclared crate or module `db`

error[E0433]: failed to resolve: use of undeclared crate or module `db`
  --> src/rooms.rs:14:9
   |
14 |         db::get_conversation_by_room_uid(&mut conn, room_id)
   |         ^^ use of undeclared crate or module `db`

error[E0433]: failed to resolve: use of undeclared crate or module `db`
  --> src/rooms.rs:38:9
   |
38 |         db::get_all_rooms(&mut conn)
   |         ^^ use of undeclared crate or module `db`

error[E0599]: no function or associated item named `new_v4` found for struct `Uuid` in the current scope
   --> src/db.rs:20:19
    |
20  |         id: Uuid::new_v4().to_string(),
    |                   ^^^^^^ function or associated item not found in `Uuid`
    |
note: if you're trying to build a new `Uuid` consider using one of the following associated functions:
      uuid::builder::<impl Uuid>::nil
      uuid::builder::<impl Uuid>::max
      uuid::builder::<impl Uuid>::from_fields
      uuid::builder::<impl Uuid>::from_fields_le
      and 10 others
   --> /Users/mb/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.11.0/src/builder.rs:72:5
    |
72  |     pub const fn nil() -> Self {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
98  |     pub const fn max() -> Self {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
122 |     pub const fn from_fields(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Uuid {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
168 |     pub const fn from_fields_le(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Uuid {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
  --> src/db.rs:24:5
   |
24 |     diesel::insert_into(users).values(&new_user).execute(conn)?;
   |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0599]: no function or associated item named `new_v4` found for struct `Uuid` in the current scope
   --> src/db.rs:34:19
    |
34  |         id: Uuid::new_v4().to_string(),
    |                   ^^^^^^ function or associated item not found in `Uuid`
    |
note: if you're trying to build a new `Uuid` consider using one of the following associated functions:
      uuid::builder::<impl Uuid>::nil
      uuid::builder::<impl Uuid>::max
      uuid::builder::<impl Uuid>::from_fields
      uuid::builder::<impl Uuid>::from_fields_le
      and 10 others
   --> /Users/mb/.cargo/registry/src/index.crates.io-6f17d22bba15001f/uuid-1.11.0/src/builder.rs:72:5
    |
72  |     pub const fn nil() -> Self {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
98  |     pub const fn max() -> Self {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
...
122 |     pub const fn from_fields(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Uuid {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
168 |     pub const fn from_fields_le(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Uuid {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0433]: failed to resolve: use of undeclared crate or module `diesel`
  --> src/db.rs:40:5
   |
40 |     diesel::insert_into(conversations)
   |     ^^^^^^ use of undeclared crate or module `diesel`

error[E0277]: the trait bound `ChatServer: actix::Handler<ClientMessage>` is not satisfied
   --> src/session.rs:108:43
    |
108 |                           self.addr.do_send(server::ClientMessage {
    |  ___________________________________-------_^
    | |                                   |
    | |                                   required by a bound introduced by this call
109 | |                             id: self.id,
110 | |                             msg,
111 | |                             room: self.room.clone(),
112 | |                         })
    | |_________________________^ the trait `actix::Handler<ClientMessage>` is not implemented for `ChatServer`
    |
    = help: the following other types implement trait `actix::Handler<M>`:
              `ChatServer` implements `actix::Handler<Connect>`
              `ChatServer` implements `actix::Handler<Disconnect>`
              `ChatServer` implements `actix::Handler<ListRooms>`
              `ChatServer` implements `actix::Handler<server::Join>`
note: required by a bound in `actix::Addr::<A>::do_send`
   --> /Users/mb/.cargo/registry/src/index.crates.io-6f17d22bba15001f/actix-0.13.5/src/address/mod.rs:104:12
    |
100 |     pub fn do_send<M>(&self, msg: M)
    |            ------- required by a bound in this associated function
...
104 |         A: Handler<M>,
    |            ^^^^^^^^^^ required by this bound in `Addr::<A>::do_send`

error[E0277]: the trait bound `ChatServer: actix::Handler<ClientMessage>` is not satisfied
   --> src/session.rs:131:43
    |
131 |                           self.addr.do_send(server::ClientMessage {
    |  ___________________________________-------_^
    | |                                   |
    | |                                   required by a bound introduced by this call
132 | |                             id: self.id,
133 | |                             msg,
134 | |                             room: self.room.clone(),
135 | |                         })
    | |_________________________^ the trait `actix::Handler<ClientMessage>` is not implemented for `ChatServer`
    |
    = help: the following other types implement trait `actix::Handler<M>`:
              `ChatServer` implements `actix::Handler<Connect>`
              `ChatServer` implements `actix::Handler<Disconnect>`
              `ChatServer` implements `actix::Handler<ListRooms>`
              `ChatServer` implements `actix::Handler<server::Join>`
note: required by a bound in `actix::Addr::<A>::do_send`
   --> /Users/mb/.cargo/registry/src/index.crates.io-6f17d22bba15001f/actix-0.13.5/src/address/mod.rs:104:12
    |
100 |     pub fn do_send<M>(&self, msg: M)
    |            ------- required by a bound in this associated function
...
104 |         A: Handler<M>,
    |            ^^^^^^^^^^ required by this bound in `Addr::<A>::do_send`

error[E0277]: the trait bound `fn(actix_web::web::Data<{type error}>, actix_web::web::Path<Uuid>) -> impl std::future::Future<Output = Result<HttpResponse, {type error}>> {get_conversation_by_id}: actix_web::Handler<_>` is not satisfied
   --> src/rooms.rs:7:14
    |
6   | #[get("/conversations/{uid}")]
    | ------------------------------ required by a bound introduced by this call
7   | pub async fn get_conversation_by_id(
    |              ^^^^^^^^^^^^^^^^^^^^^^ the trait `actix_web::Handler<_>` is not implemented for fn item `fn(Data<{type error}>, Path<Uuid>) -> impl Future<Output = Result<HttpResponse, ...>> {get_conversation_by_id}`
    |
note: required by a bound in `Resource::<T>::to`
   --> /Users/mb/.cargo/registry/src/index.crates.io-6f17d22bba15001f/actix-web-4.9.0/src/resource.rs:243:12
    |
241 |     pub fn to<F, Args>(mut self, handler: F) -> Self
    |            -- required by a bound in this associated function
242 |     where
243 |         F: Handler<Args>,
    |            ^^^^^^^^^^^^^ required by this bound in `Resource::<T>::to`

error[E0277]: the trait bound `fn(actix_web::web::Data<{type error}>) -> impl std::future::Future<Output = Result<HttpResponse, {type error}>> {get_rooms}: actix_web::Handler<_>` is not satisfied
   --> src/rooms.rs:33:14
    |
32  | #[get("/rooms")]
    | ---------------- required by a bound introduced by this call
33  | pub async fn get_rooms(
    |              ^^^^^^^^^ the trait `actix_web::Handler<_>` is not implemented for fn item `fn(actix_web::web::Data<{type error}>) -> impl std::future::Future<Output = Result<HttpResponse, {type error}>> {get_rooms}`
    |
note: required by a bound in `Resource::<T>::to`
   --> /Users/mb/.cargo/registry/src/index.crates.io-6f17d22bba15001f/actix-web-4.9.0/src/resource.rs:243:12
    |
241 |     pub fn to<F, Args>(mut self, handler: F) -> Self
    |            -- required by a bound in this associated function
242 |     where
243 |         F: Handler<Args>,
    |            ^^^^^^^^^^^^^ required by this bound in `Resource::<T>::to`

error[E0277]: the trait bound `fn(HttpRequest, actix_web::web::Payload, actix_web::web::Data<{type error}>, actix_web::web::Data<actix::Addr<ChatServer>>) -> impl std::future::Future<Output = Result<HttpResponse, actix_web::Error>> {chat_server}: actix_web::Handler<_>` is not satisfied
   --> src/main.rs:40:41
    |
40  |             .route("/ws", web::get().to(routes::chat_server))
    |                                      -- ^^^^^^^^^^^^^^^^^^^ the trait `actix_web::Handler<_>` is not implemented for fn item `fn(HttpRequest, Payload, Data<{type error}>, Data<Addr<ChatServer>>) -> ... {chat_server}`
    |                                      |
    |                                      required by a bound introduced by this call
    |
note: required by a bound in `Route::to`
   --> /Users/mb/.cargo/registry/src/index.crates.io-6f17d22bba15001f/actix-web-4.9.0/src/route.rs:212:12
    |
210 |     pub fn to<F, Args>(mut self, handler: F) -> Self
    |            -- required by a bound in this associated function
211 |     where
212 |         F: Handler<Args>,
    |            ^^^^^^^^^^^^^ required by this bound in `Route::to`

Some errors have detailed explanations: E0277, E0412, E0425, E0432, E0433, E0463, E0599.
For more information about an error, try `rustc --explain E0277`.
warning: `rust-chat` (bin "rust-chat") generated 8 warnings
error: could not compile `rust-chat` (bin "rust-chat") due to 63 previous errors; 8 warnings emitted
```