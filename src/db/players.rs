use crate::models::player::{NewPlayer, Player};
use crate::schema::players::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::SqliteConnection;

/// Returns all rooms in table.
pub fn get_all_players(conn: &SqliteConnection) -> Result<Vec<Player>, Error> {
    players.order(id.desc()).load::<Player>(conn)
}

/// Returns the most recent player with the given name from the table.
pub fn get_player_by_name(player_name: String, conn: &SqliteConnection) -> Result<Player, Error> {
    players
        .order(id.desc())
        .filter(name.eq(player_name))
        .first::<Player>(conn)
}

/// Returns the most recent player with the given id from the table.
pub fn get_player_by_id(player_id: i32, conn: &SqliteConnection) -> Result<Player, Error> {
    players
        .filter(id.eq(player_id))
        .first::<Player>(conn)
}

/// Creates a new player with the name supplied by the user.
pub fn create_player_with_name(
    player_name: String,
    conn: &SqliteConnection,
) -> Result<Player, Error> {
    let new_player = NewPlayer {
        name: player_name.clone(),
    };

    diesel::insert_into(players)
        .values(&new_player)
        .execute(conn)?;

    // Return created player
    get_player_by_name(player_name, conn)
}

/// Creates a new player with the name supplied by the user.
pub fn update_player_name(
    player_id: i32,
    player_new_name: String,
    conn: &SqliteConnection,
) -> Result<Player, Error> {
    diesel::update(players.filter(id.eq(player_id)))
        .set(name.eq(player_new_name))
        .execute(conn)?;

    // Return modified player
    get_player_by_id(player_id, conn)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn delete_all(conn: &SqliteConnection) -> bool {
        diesel::delete(players).execute(conn).is_ok()
    }
}
