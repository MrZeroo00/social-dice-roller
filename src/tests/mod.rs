extern crate parking_lot;

use self::parking_lot::Mutex;
use super::db::rooms;
use rocket::http::Status;
use rocket::local::{Client, LocalResponse};
use rocket_contrib::json::JsonValue;

// We use a lock to synchronize between tests so DB operations don't collide.
// For now. In the future, we'll have a nice way to run each test in a DB
// transaction so we can regain concurrency.
static DB_LOCK: Mutex<()> = parking_lot::const_mutex(());

macro_rules! run_test {
    (|$client:ident, $conn:ident| $block:expr) => {{
        let _lock = DB_LOCK.lock();
        let rocket = super::rocket();
        let db = super::db::DbConn::get_one(&rocket);
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("failed to get database connection for testing");
        assert!(
            super::db::rooms::tests::delete_all(&$conn),
            "failed to delete all tasks for testing"
        );
        $block
    }};
}

/// Helper function for converting response to json value.
pub fn response_json_value(response: &mut LocalResponse) -> JsonValue {
    let body = response.body().expect("No body");
    info!("{:?}", body);
    serde_json::from_reader(body.into_inner()).expect("Can't parse value")
}

/// Helper function for getting all rooms using rooms route
pub fn get_rooms_route(client: &Client) -> JsonValue {
    // Get the rooms before making changes.
    let mut init_rooms_response = client.get("/api/rooms").dispatch();
    assert_eq!(init_rooms_response.status(), Status::Ok);
    response_json_value(&mut init_rooms_response)
}

#[test]
fn create_room() {
    run_test!(|client, conn| {
        // Get the rooms before making changes.
        let init_rooms = rooms::get_all_rooms(&conn).unwrap();

        // Issue a request to create a new room.
        let response = client.post("/api/rooms/create").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Ensure we have one more room in the database.
        let new_rooms = rooms::get_all_rooms(&conn).unwrap();
        assert_eq!(new_rooms.len(), init_rooms.len() + 1);
    })
}

#[test]
fn create_room_route() {
    run_test!(|client, conn| {
        // Get the rooms before making changes.
        let init_rooms = get_rooms_route(&client);
        let init_rooms_len = init_rooms.as_array().unwrap().len();

        // Issue a request to create a new room.
        let mut response = client.post("/api/rooms/create").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Check that this is a valid JSON (otherwise this function call would panic)
        let _response_json = response_json_value(&mut response);

        // Ensure we have one more room in the database.
        let new_rooms = get_rooms_route(&client);
        let new_rooms_len = new_rooms.as_array().unwrap().len();

        // Ensure we have one more room in the database.
        assert_eq!(new_rooms_len, init_rooms_len + 1);
    })
}

#[test]
fn create_room_with_name() {
    run_test!(|client, conn| {
        // Get the rooms before making changes.
        let init_rooms = rooms::get_all_rooms(&conn).unwrap();

        // Issue a request to create a new room.
        let response = client.post("/api/rooms/create/happy-cow").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Ensure we have one more room in the database.
        let new_rooms = rooms::get_all_rooms(&conn).unwrap();
        assert_eq!(new_rooms.len(), init_rooms.len() + 1);

        // Ensure it's the room we expect.
        assert_eq!(new_rooms[0].id, "happy-cow");
    })
}

#[test]
fn create_room_with_name_route() {
    run_test!(|client, conn| {
        // Get the rooms before making changes.
        let init_rooms = get_rooms_route(&client);
        let init_rooms_len = init_rooms.as_array().unwrap().len();

        // Issue a request to create a new room.
        let mut response = client.post("/api/rooms/create/happy-cow").dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Check that this is a valid JSON (otherwise this function call would panic)
        let response_json = response_json_value(&mut response);

        // Ensure the endpoint returns a JSON with the room id we expect
        let response_room_id = response_json
            .get("id")
            .expect("must have an 'id' field")
            .as_str()
            .unwrap();
        assert_eq!(response_room_id, "happy-cow");

        // Ensure we have one more room in the database.
        let new_rooms = get_rooms_route(&client);
        let new_rooms_len = new_rooms.as_array().unwrap().len();
        assert_eq!(new_rooms_len, init_rooms_len + 1);

        // Ensure it's the room we expect.
        let new_room_id = new_rooms
            .get(0)
            .unwrap()
            .get("id")
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(new_room_id, "happy-cow");
    })
}

#[test]
fn create_duplicate_room_with_name() {
    run_test!(|client, conn| {
        // Get the rooms before making changes.
        let init_rooms = rooms::get_all_rooms(&conn).unwrap();

        // Issue a request to create a new room.
        let mut response = client.post("/api/rooms/create/happy-cow").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Ensure we have one more room in the database.
        let mut new_rooms = rooms::get_all_rooms(&conn).unwrap();
        assert_eq!(new_rooms.len(), init_rooms.len() + 1);

        // Ensure it's the room we expect.
        assert_eq!(new_rooms[0].id, "happy-cow");

        // Issue a request to create a new room with the same name as the previous one.
        response = client.post("/api/rooms/create/happy-cow").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Ensure we didn't create a new room.
        new_rooms = rooms::get_all_rooms(&conn).unwrap();
        assert_eq!(new_rooms.len(), init_rooms.len() + 1);

        // Ensure it's the room we expect.
        assert_eq!(new_rooms[0].id, "happy-cow");
    })
}

#[test]
fn create_duplicate_room_with_name_route() {
    run_test!(|client, conn| {
        // Get the rooms before making changes.
        let init_rooms = get_rooms_route(&client);
        let init_rooms_len = init_rooms.as_array().unwrap().len();

        // Issue a request to create a new room.
        let mut response = client.post("/api/rooms/create/happy-cow").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Check that this is a valid JSON (otherwise this function call would panic)
        let response_json = response_json_value(&mut response);

        // Ensure the endpoint returns a JSON with the room id we expect
        let response_room_id = response_json
            .get("id")
            .expect("must have an 'id' field")
            .as_str()
            .unwrap();
        assert_eq!(response_room_id, "happy-cow");

        // Ensure we have one more room in the database.
        let new_rooms = get_rooms_route(&client);
        let new_rooms_len = new_rooms.as_array().unwrap().len();
        assert_eq!(new_rooms_len, init_rooms_len + 1);

        // Ensure it's the room we expect.
        let new_room_id = new_rooms
            .get(0)
            .unwrap()
            .get("id")
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(new_room_id, "happy-cow");

        // Issue a request to create a new room with the same name as the previous one.
        response = client.post("/api/rooms/create/happy-cow").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Ensure we didn't create a new room.
        let new_rooms = get_rooms_route(&client);
        let new_rooms_len = new_rooms.as_array().unwrap().len();
        assert_eq!(new_rooms_len, init_rooms_len + 1);

        // Ensure it's the room we expect.
        let new_room_id = new_rooms
            .get(0)
            .unwrap()
            .get("id")
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(new_room_id, "happy-cow");
    })
}
