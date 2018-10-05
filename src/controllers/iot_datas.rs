use db::DbConn;
use diesel::prelude::*;
use models::iot_data::{IOTData, NewIOTData};
use models::thing::ThingKey;
use rocket_contrib::{Json, Value};

#[post("/iot_datas", data = "<data>")]
fn create(
    data: Json<NewIOTData>,
    thing_key: ThingKey,
    conn: DbConn,
) -> Result<Json<IOTData>, Json<Value>> {
    let iot_data = IOTData::create(&thing_key, &data, &conn)?;

    Ok(Json(iot_data))
}

#[get("/iot_datas")]
fn index(conn: DbConn) -> Result<Json<Vec<IOTData>>, Json<Value>> {
    use schema::iot_datas;

    match iot_datas::table.load::<IOTData>(&*conn) {
        Ok(records) => Ok(Json(records)),
        Err(_) => Err(Json(json!({ "error": "Could not load records" }))),
    }
}
