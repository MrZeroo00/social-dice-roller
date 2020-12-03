use crate::db;
use crate::models::player;
use diesel::result::Error;
use rocket_contrib::json::Json;

#[post("/api/players/create/<player_name>")]
pub fn create_player_with_name(
    player_name: String,
    conn: db::DbConn,
) -> Result<Json<player::Player>, Error> {
    let room = db::players::create_player_with_name(player_name, &conn);
    Ok(Json(room.unwrap()))
}

#[get("/api/players")]
pub fn get_players(conn: db::DbConn) -> Result<Json<Vec<player::Player>>, Error> {
    let players = db::players::get_all_players(&conn);
    Ok(Json(players.unwrap()))
}

#[put("/api/players/update", format = "json", data = "<player>")]
pub fn update_player_name(
    player: Json<player::Player>,
    conn: db::DbConn,
) -> Result<Json<player::Player>, Error> {
    let player = db::players::update_player_name(player.0.id, player.0.name, &conn);
    Ok(Json(player.unwrap()))
}
