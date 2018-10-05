#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate rocket_contrib;
extern crate web_token;

mod controllers;
mod db;
mod models;
mod schema;

use rocket::Rocket;
use rocket_contrib::{Json, Value};

use controllers::{iot_datas, things};
use db::{init_pool, SqlitePool};

use std::io;

embed_migrations!();

fn run_migrations(conn: &SqlitePool) {
    let conn = conn.get().expect("Cannot get a connection from the pool !");
    embedded_migrations::run_with_output(&conn, &mut io::stdout())
        .expect("Failed to run migrations");
}

#[catch(404)]
fn not_found() -> Json<Value> {
    Json(json!({
        "error": "Resource was not found"
    }))
}

fn rocket() -> Rocket {
    let db_pool = init_pool();

    run_migrations(&db_pool);

    rocket::ignite().manage(db_pool).mount(
        "/",
        routes![
            iot_datas::create,
            iot_datas::index,
            things::create,
            things::index,
            things::index_with_data,
        ],
    )
}

fn main() {
    rocket().launch();
}
