extern crate parking_lot;

use self::parking_lot::Mutex;
use rocket::local::LocalResponse;
use rocket_contrib::json::JsonValue;

// We use a lock to synchronize between tests so DB operations don't collide.
// For now. In the future, we'll have a nice way to run each test in a DB
// transaction so we can regain concurrency.
static DB_LOCK: Mutex<()> = parking_lot::const_mutex(());

/// Helper function for converting response to json value.
pub fn response_json_value(response: &mut LocalResponse) -> JsonValue {
    let body = response.body().expect("No body");
    info!("{:?}", body);
    serde_json::from_reader(body.into_inner()).expect("Can't parse value")
}

mod players;
mod rooms;
