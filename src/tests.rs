extern crate parking_lot;

use self::parking_lot::Mutex;
use super::db::rooms;
use rocket::http::Status;
use rocket::local::Client;

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

#[test]
fn create_room() {
    run_test!(|client, conn| {
        // Get the rooms before making changes.
        let init_rooms = rooms::get_all_rooms(&conn);

        // Issue a request to create a new room.
        let result = client.post("/api/rooms/create").dispatch();
        assert_eq!(result.status(), Status::Ok);

        // Ensure we have one more room in the database.
        let new_rooms = rooms::get_all_rooms(&conn);
        assert_eq!(new_rooms.len(), init_rooms.len() + 1);
    })
}

#[test]
fn create_room_with_name() {
    run_test!(|client, conn| {
        // Get the rooms before making changes.
        let init_rooms = rooms::get_all_rooms(&conn);

        // Issue a request to create a new room.
        let result = client.post("/api/rooms/create/happy-cow").dispatch();
        assert_eq!(result.status(), Status::Ok);

        // Ensure we have one more room in the database.
        let new_rooms = rooms::get_all_rooms(&conn);
        assert_eq!(new_rooms.len(), init_rooms.len() + 1);

        // Ensure it's the room we expect.
        assert_eq!(new_rooms[0].id, "happy-cow");
    })
}

#[test]
fn create_duplicate_room_with_name() {
    run_test!(|client, conn| {
        // Get the rooms before making changes.
        let init_rooms = rooms::get_all_rooms(&conn);

        // Issue a request to create a new room.
        let mut result = client.post("/api/rooms/create/happy-cow").dispatch();
        assert_eq!(result.status(), Status::Ok);

        // Ensure we have one more room in the database.
        let mut new_rooms = rooms::get_all_rooms(&conn);
        assert_eq!(new_rooms.len(), init_rooms.len() + 1);

        // Ensure it's the room we expect.
        assert_eq!(new_rooms[0].id, "happy-cow");

        // Issue a request to create a new room with the same name as the previous one.
        result = client.post("/api/rooms/create/happy-cow").dispatch();
        assert_eq!(result.status(), Status::Ok);

        // Ensure we didn't create a new room.
        new_rooms = rooms::get_all_rooms(&conn);
        assert_eq!(new_rooms.len(), init_rooms.len() + 1);

        // Ensure it's the room we expect.
        assert_eq!(new_rooms[0].id, "happy-cow");
    })
}
