#![allow(unused)]

use crate::time;

#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub description: String,
    pub status: String,
    pub created_at: String,
    pub update_at: String,
}

impl Todo {
    pub fn new(id: usize, description: String) -> Self {
        Self {
            id: (id + 1) as u32,
            description,
            status: "todo".to_string(),
            created_at: time::get_time().take_time(),
            update_at: time::get_time().take_time(),
        }
    }
}
