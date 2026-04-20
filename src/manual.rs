//! Stub: manual library — extracted to holodeck-programs crate
//!
//! Minimal stub for the core MUD engine. Replace with full crate when needed.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualEntry {
    pub room_id: String,
    pub content: String,
    pub feedback: Vec<(String, u8, String)>, // (agent, rating, comment)
}

impl ManualEntry {
    pub fn add_feedback(&mut self, agent: &str, rating: u8, comment: &str) {
        self.feedback.push((agent.to_string(), rating, comment.to_string()));
    }
}

#[derive(Default)]
pub struct ManualLibrary {
    manuals: HashMap<String, ManualEntry>,
}

impl ManualLibrary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_manual(&mut self, room_id: &str) -> String {
        let entry = self.get_or_create(room_id);
        if entry.content.is_empty() {
            format!("No manual for {} yet. Be the first to write one.", room_id)
        } else {
            entry.content.clone()
        }
    }

    pub fn get_or_create(&mut self, room_id: &str) -> &mut ManualEntry {
        self.manuals.entry(room_id.to_string())
            .or_insert_with(|| ManualEntry {
                room_id: room_id.to_string(),
                content: String::new(),
                feedback: Vec::new(),
            })
    }
}
