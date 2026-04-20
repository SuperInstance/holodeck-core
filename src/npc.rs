//! Stub: NPC configuration — extracted to holodeck-bridge crate

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcConfig {
    pub name: String,
    pub room_id: String,
    pub role: String,
    pub greeting: String,
}
