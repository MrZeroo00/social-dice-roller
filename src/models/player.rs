use crate::schema::players;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub name: String,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Player ID {}, named {})", self.id, self.name)
    }
}

#[derive(Insertable)]
#[table_name = "players"]
pub struct NewPlayer {
    pub name: String,
}

impl fmt::Display for NewPlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Player named {})", self.name)
    }
}
