use crate::manager::monitor::Monitor;

pub struct Profile {
    pub id: String,
    pub name: String,
    pub monitors: Vec<Monitor>,
}
