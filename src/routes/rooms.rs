use crate::db;

#[post("/api/rooms/create")]
pub fn create_room(conn: db::DbConn) -> &'static str {
    let room = db::rooms::create_room(&conn);
    info!("{}", room.unwrap());
    "Success"
}

#[post("/api/rooms/create/<room_name>")]
pub fn create_room_with_name(room_name: String, conn: db::DbConn) -> &'static str {
    let room = db::rooms::create_room_with_name(room_name, &conn);
    info!("{}", room.unwrap());
    "Success"
}
