use crate::db::players;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => {{
        let _lock = super::DB_LOCK.lock();
        let rocket = crate::rocket();
        let db = crate::db::DbConn::get_one(&rocket);
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("failed to get database connection for testing");
        assert!(
            players::tests::delete_all(&$conn),
            "failed to delete all players for testing"
        );
        $block
    }};
}

#[test]
fn create_player_with_name() {
    run_test!(|client, conn| {
        // Get the players list before making changes.
        let init_players = players::get_all_players(&conn).unwrap();

        // Issue a request to create a new player.
        let response = client.post("/api/players/create/roger").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Ensure we have one more player in the database.
        let new_players = players::get_all_players(&conn).unwrap();
        assert_eq!(new_players.len(), init_players.len() + 1);

        // Ensure it's the player we expect.
        assert_eq!(new_players[0].name, "roger");
    })
}

#[test]
fn update_player_name() {
    run_test!(|client, conn| {
        // Issue a request to create a new player.
        let mut response = client.post("/api/players/create/roger").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Check that this is a valid JSON (otherwise this function call would panic).
        let response_json = super::response_json_value(&mut response);

        // Get the inserted player id.
        let response_player_id = response_json
            .get("id")
            .expect("must have an 'id' field")
            .as_i64()
            .unwrap();

        // Issue a request to update player name.
        let mut response = client
            .put("/api/players/update")
            .header(ContentType::JSON)
            .body(format!(r#"{{"id": {}, "name": "alfred" }}"#, response_player_id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Check that this is a valid JSON (otherwise this function call would panic).
        let response_json = super::response_json_value(&mut response);

        // Ensure the endpoint returns a JSON with the new player name we expect.
        let response_player_new_name = response_json
            .get("name")
            .expect("must have an 'name' field")
            .as_str()
            .unwrap();
        assert_eq!(response_player_new_name, "alfred");
    })
}
