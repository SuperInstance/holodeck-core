//! Stub: combat engine — extracted to holodeck-combat crate
//!
//! This module provides minimal stubs so the core MUD engine compiles
//! without the full combat system. Replace with the full crate when needed.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::gauge::Gauge;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatScript {
    pub name: String,
    pub conditions: Vec<ScriptCondition>,
    pub actions: Vec<ScriptAction>,
    pub priority: u32,
    pub generation: u32,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptCondition {
    pub gauge_name: String,
    pub operator: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptAction {
    pub action_type: String,
    pub target: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlertLevel {
    Green,
    Yellow,
    Red,
}

impl std::fmt::Display for AlertLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertLevel::Green => write!(f, "GREEN"),
            AlertLevel::Yellow => write!(f, "YELLOW"),
            AlertLevel::Red => write!(f, "RED"),
        }
    }
}

pub struct TickResult {
    pub alerts: Vec<String>,
    pub scripts_fired: Vec<String>,
}

#[derive(Default)]
pub struct CombatEngine {
    pub tick_count: u64,
    pub scripts: Vec<CombatScript>,
    pub active_alerts: HashMap<String, AlertLevel>,
}

impl CombatEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, room_id: &str, _gauges: &HashMap<String, Gauge>) -> TickResult {
        self.tick_count += 1;
        TickResult {
            alerts: Vec::new(),
            scripts_fired: Vec::new(),
        }
    }

    pub fn fleet_alert_level(&self) -> AlertLevel {
        if self.active_alerts.values().any(|l| *l == AlertLevel::Red) {
            AlertLevel::Red
        } else if self.active_alerts.values().any(|l| *l == AlertLevel::Yellow) {
            AlertLevel::Yellow
        } else {
            AlertLevel::Green
        }
    }

    pub fn add_script(&mut self, script: CombatScript) {
        self.scripts.push(script);
    }
}
