#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate names;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate log;
use rocket::fairing::AdHoc;

mod config;
pub mod db;
mod models;
mod routes;
mod schema;
#[cfg(test)]
mod tests;

/// Initialise a rocket instance
pub fn rocket() -> rocket::Rocket {
    rocket::custom(config::from_env())
        .attach(db::DbConn::fairing())
        .attach(AdHoc::on_launch("Database Migrations", |rocket| {
            db::run_db_migrations(rocket).unwrap();
        }))
        .mount(
            "/",
            routes![
                routes::rooms::create_room,
                routes::rooms::create_room_with_name,
                routes::rooms::get_rooms,
            ],
        )
        .register(catchers![routes::not_found])
}
