#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket::serde::json::Json;
use rocket_sync_db_pools::database;

#[database("postgres_db")]
struct DbConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
}