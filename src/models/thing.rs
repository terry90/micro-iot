#![allow(proc_macro_derive_resolution_fallback)]
use chrono::NaiveDateTime;
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::iot_data::IOTData;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket_contrib::Json;
use schema::things;
use serde_json::Value;
use std::str::FromStr;
use web_token::WebToken;

#[derive(Identifiable, Serialize, Deserialize, Queryable)]
pub struct Thing {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub token: WebToken,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct ThingWithIotDatas {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub token: WebToken,
    pub iot_datas: Vec<IOTData>,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NewThing {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Deserialize, Insertable, PartialEq, Debug)]
#[table_name = "things"]
pub struct NewThingWithToken {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub token: WebToken,
}

fn create_error() -> Json<Value> {
    Json(json!({
                "error": "Cannot create thing."
            }))
}

fn find_error() -> Json<Value> {
    Json(json!({
                "error": "Cannot find thing."
            }))
}

fn load_assos_error() -> Json<Value> {
    Json(json!({
                "error": "Cannot load thing associated data."
            }))
}

impl ThingWithIotDatas {
    fn build(thing: &Thing, datas: Vec<IOTData>) -> Self {
        ThingWithIotDatas {
            id: thing.id,
            name: thing.name.clone(),
            type_: thing.type_.clone(),
            token: thing.token.clone(),
            iot_datas: datas,
            created_at: thing.created_at,
        }
    }
}

impl Thing {
    pub fn with_iot_datas(
        &self,
        conn: &SqliteConnection,
    ) -> Result<ThingWithIotDatas, Json<Value>> {
        match IOTData::belonging_to(self).load(conn) {
            Ok(datas) => Ok(ThingWithIotDatas::build(self, datas)),
            Err(_err) => Err(load_assos_error()),
        }
    }

    pub fn find_by_token(_token: &str, conn: &SqliteConnection) -> Result<Thing, Json<Value>> {
        match things::table.filter(things::token.eq(_token)).first(conn) {
            Ok(thing) => Ok(thing),
            Err(_err) => Err(find_error()),
        }
    }

    pub fn create(data: &NewThing, conn: &SqliteConnection) -> Result<Thing, Json<Value>> {
        use schema::things::id;

        let data = NewThingWithToken {
            name: data.name.clone(),
            type_: data.type_.clone(),
            token: WebToken::new(),
        };

        match insert_into(things::table).values(data).execute(conn) {
            Ok(_) => {
                let data: Thing = things::table
                    .order(id.desc())
                    .first(conn)
                    .expect("Cannot retrieve data");
                Ok(data)
            }
            Err(_err) => Err(create_error()),
        }
    }
}

pub struct ThingKey(pub WebToken);

impl<'a, 'r> FromRequest<'a, 'r> for ThingKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ThingKey, ()> {
        let keys: Vec<_> = request.headers().get("x-thing-key").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let key = match WebToken::from_str(keys[0]) {
            Ok(res) => res,
            Err(_err) => return Outcome::Failure((Status::Unauthorized, ())),
        };

        Outcome::Success(ThingKey(key))
    }
}
