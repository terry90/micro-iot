#![allow(proc_macro_derive_resolution_fallback)]
use chrono::NaiveDateTime;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rocket_contrib::Json;
use schema::iot_datas;
use serde_json::Value;

#[derive(Serialize, Deserialize, Queryable)]
pub struct IOTData {
    pub id: i32,
    pub thing_name: String,
    pub thing_type: String,
    pub value: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, PartialEq, Debug)]
#[table_name = "iot_datas"]
pub struct NewIOTData {
    pub thing_name: String,
    pub thing_type: String,
    pub value: String,
}

fn create_error() -> Json<Value> {
    Json(json!({
                "error": "Cannot create entry."
            }))
}

impl IOTData {
    pub fn create(conn: &SqliteConnection, data: &NewIOTData) -> Result<IOTData, Json<Value>> {
        use schema::iot_datas::id;

        match insert_into(iot_datas::table).values(data).execute(conn) {
            Ok(_) => {
                let data: IOTData = iot_datas::table
                    .order(id.desc())
                    .first(conn)
                    .expect("Cannot retrieve data");
                Ok(data)
            }
            Err(err) => Err(create_error()),
        }
    }
}
