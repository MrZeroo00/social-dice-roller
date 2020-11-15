use crate::db;
use crate::models::room;
use diesel::result::Error;
use rocket_contrib::json::Json;

#[post("/api/rooms/create")]
pub fn create_room(conn: db::DbConn) -> Result<Json<room::Room>, Error> {
    let room = db::rooms::create_room(&conn);
    Ok(Json(room.unwrap()))
}

#[post("/api/rooms/create/<room_name>")]
pub fn create_room_with_name(
    room_name: String,
    conn: db::DbConn,
) -> Result<Json<room::Room>, Error> {
    let room = db::rooms::create_room_with_name(room_name, &conn);
    Ok(Json(room.unwrap()))
}
