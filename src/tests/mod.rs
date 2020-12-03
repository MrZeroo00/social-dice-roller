extern crate parking_lot;

use self::parking_lot::Mutex;

// We use a lock to synchronize between tests so DB operations don't collide.
// For now. In the future, we'll have a nice way to run each test in a DB
// transaction so we can regain concurrency.
static DB_LOCK: Mutex<()> = parking_lot::const_mutex(());

mod rooms;
