use crate::models::room::{NewRoom, Room};
use crate::schema::rooms::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::SqliteConnection;
use names::Generator;

/// Checks if a room exists in the room database table.
fn room_exists(room_name: &str, conn: &SqliteConnection) -> Result<bool, Error> {
    // let sql = debug_query::<diesel::sqlite::Sqlite, _>(&rooms.filter(id.eq(room_name)).count()).to_string();
    let result: i64 = rooms
        .filter(id.eq(room_name))
        .count()
        .load(conn)?
        .pop()
        .unwrap();
    if result == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Creates a new room with a random generated name.
pub fn create_room(conn: &SqliteConnection) -> Result<Room, Error> {
    let mut generator = Generator::default();
    let mut rand_room_name = generator.next().unwrap();

    while room_exists(&rand_room_name, conn)? {
        rand_room_name = generator.next().unwrap();
    }

    let new_room = NewRoom {
        id: rand_room_name.clone(),
    };

    diesel::insert_into(rooms).values(&new_room).execute(conn)?;
    // Return created room
    get_room(rand_room_name, conn)
}

/// Creates a new room with the name supplied by the user. If no name is supplied, it generates a
/// random new name and create the room.
pub fn create_room_with_name(room_name: String, conn: &SqliteConnection) -> Result<Room, Error> {
    let new_room = NewRoom {
        id: room_name.clone(),
    };

    // If room exists, return existing room
    if room_exists(&room_name, conn)? {
        get_room(room_name, conn)
    // If room does not exist, create a new room and return created room
    } else {
        diesel::insert_into(rooms).values(&new_room).execute(conn)?;
        get_room(room_name, conn)
    }
}

/// Returns all rooms in table.
pub fn get_all_rooms(conn: &SqliteConnection) -> Vec<Room> {
    rooms.order(id.desc()).load::<Room>(conn).unwrap()
}

/// Returns a room with the given name from the table.
pub fn get_room(room_name: String, conn: &SqliteConnection) -> Result<Room, Error> {
    rooms.filter(id.eq(room_name)).first::<Room>(conn)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(rooms).execute(conn).is_ok()
    }
}
