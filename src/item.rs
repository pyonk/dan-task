use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Category {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub title: String,
    pub done: bool,
    pub created_at: DateTime<Local>,
    pub categories: Vec<Category>,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            title: "".to_string(),
            done: false,
            created_at: Local::now(),
            categories: Vec::<Category>::new(),
        }
    }
}

impl Item {
    pub fn toggle(&mut self) {
        self.done = !self.done;
    }
}
