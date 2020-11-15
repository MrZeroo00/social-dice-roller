use diesel::SqliteConnection;
use rocket::Rocket;

pub mod rooms;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

#[database("sqlite_database")]
pub struct DbConn(SqliteConnection);

/// Creates the migration function to be used by rocket's `on_launch` callback
pub fn run_db_migrations(rocket: &Rocket) -> Result<(), &'static str> {
    let conn = DbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(()),
        Err(_) => Err("Failed to run database migrations"),
    }
}
