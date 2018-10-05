use db::DbConn;
use diesel::prelude::*;
use models::thing::{NewThing, Thing, ThingWithIotDatas};
use rocket_contrib::{Json, Value};

#[post("/things", data = "<data>")]
fn create(data: Json<NewThing>, conn: DbConn) -> Result<Json<Thing>, Json<Value>> {
    let iot_data = Thing::create(&data, &conn)?;

    Ok(Json(iot_data))
}

#[get("/things")]
fn index(conn: DbConn) -> Result<Json<Vec<Thing>>, Json<Value>> {
    use schema::things;

    match things::table.load::<Thing>(&*conn) {
        Ok(records) => Ok(Json(records)),
        Err(_) => Err(Json(json!({ "error": "Could not load records" }))),
    }
}

#[get("/things_with_data")]
fn index_with_data(conn: DbConn) -> Result<Json<Vec<ThingWithIotDatas>>, Json<Value>> {
    use schema::things;

    match things::table.load::<Thing>(&*conn) {
        Ok(records) => {
            let embedded_data = records
                .into_iter()
                .map(|thing| thing.with_iot_datas(&conn))
                .collect::<Vec<Result<ThingWithIotDatas, Json<Value>>>>();

            // Collect here returns the first err encountered or the Vec
            let res: Result<Vec<ThingWithIotDatas>, Json<Value>> =
                embedded_data.into_iter().collect();

            Ok(Json(res?))
        }
        Err(_) => Err(Json(json!({ "error": "Could not load records" }))),
    }
}
