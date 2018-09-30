#![feature(plugin, decl_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

#[macro_use]
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

mod db;
mod iot_data;
mod schema;

use diesel::prelude::*;
use rocket::Rocket;
use rocket_contrib::Json;
use serde_json::Value;

use db::{init_pool, DbConn, SqlitePool};
use iot_data::{IOTData, NewIOTData};
use std::io;

embed_migrations!();

fn run_migrations(conn: &SqlitePool) {
    let conn = conn.get().expect("Cannot get a connection from the pool !");
    embedded_migrations::run_with_output(&conn, &mut io::stdout())
        .expect("Failed to run migrations");
}

#[post("/", data = "<data>")]
fn new(data: Json<NewIOTData>, conn: DbConn) -> Result<Json<IOTData>, Json<Value>> {
    let iot_data = IOTData::create(&conn, &data)?;

    Ok(Json(iot_data))
}

#[get("/")]
fn index(conn: DbConn) -> Result<Json<Vec<IOTData>>, Json<Value>> {
    use schema::iot_datas;

    match iot_datas::table.load::<IOTData>(&*conn) {
        Ok(records) => Ok(Json(records)),
        Err(_) => Err(Json(json!({ "error": "Could not load records" }))),
    }
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

    rocket::ignite()
        .manage(db_pool)
        .mount("/", routes![new, index])
}

fn main() {
    rocket().launch();
}
