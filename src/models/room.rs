use crate::schema::rooms;
use serde::Serialize;
use std::fmt;

#[derive(Queryable, Debug, Serialize)]
pub struct Room {
    pub id: String,
    pub created_at: chrono::NaiveDateTime,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Room {} was created at {})", self.id, self.created_at)
    }
}

#[derive(Insertable)]
#[table_name = "rooms"]
pub struct NewRoom {
    pub id: String,
}

impl fmt::Display for NewRoom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.id)
    }
}
