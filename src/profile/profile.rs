use serde::{Deserialize, Serialize};
use crate::manager::monitor::Monitor;

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub monitors: Vec<Monitor>,
}
