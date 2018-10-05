#![allow(proc_macro_derive_resolution_fallback)]
use chrono::NaiveDateTime;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::thing::{Thing, ThingKey};
use rocket_contrib::Json;
use schema::iot_datas;
use serde_json::Value;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
#[belongs_to(Thing, foreign_key = "thing_id")]
#[table_name = "iot_datas"]
pub struct IOTData {
    pub id: i32,
    pub thing_id: i32,
    pub value: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NewIOTData {
    pub value: String,
}

#[derive(Serialize, Deserialize, Insertable, PartialEq, Debug)]
#[table_name = "iot_datas"]
pub struct NewIOTDataAssociated {
    pub thing_id: i32,
    pub value: String,
}

fn create_error() -> Json<Value> {
    Json(json!({
                "error": "Cannot create entry."
            }))
}

impl IOTData {
    pub fn create(
        thing_key: &ThingKey,
        data: &NewIOTData,
        conn: &SqliteConnection,
    ) -> Result<IOTData, Json<Value>> {
        use schema::iot_datas::id;

        let thing = match Thing::find_by_token(&thing_key.0, conn) {
            Ok(res) => res,
            Err(err) => return Err(err),
        };

        let data = NewIOTDataAssociated {
            thing_id: thing.id,
            value: data.value.clone(),
        };

        match insert_into(iot_datas::table).values(data).execute(conn) {
            Ok(_) => {
                let data: IOTData = iot_datas::table
                    .order(id.desc())
                    .first(conn)
                    .expect("Cannot retrieve data");
                Ok(data)
            }
            Err(_err) => Err(create_error()),
        }
    }
}
