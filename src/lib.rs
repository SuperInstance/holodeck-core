//! holodeck-core — Standalone MUD Engine
//!
//! Lightweight room graph, agents, gauges, scoped communication.
//! Runs anywhere Rust compiles. No CUDA, no external services.
//!
//! # Crate Layout
//! - `agent` — Agent entity with command dispatcher
//! - `room` — Room graph with exits, gauges, data sources
//! - `gauge` — Time-series gauge with status tracking
//! - `comms` — Scoped communication (say/tell/yell/gossip/notes/mail)
//! - `permission` — Role-based permission levels
//! - `combat` — Combat engine stub (full version in holodeck-combat)
//! - `manual` — Manual library stub (full version in holodeck-programs)
//! - `npc` — NPC config stub (full version in holodeck-bridge)

pub mod agent;
pub mod comms;
pub mod combat;
pub mod gauge;
pub mod manual;
pub mod npc;
pub mod permission;
pub mod room;

pub use agent::Agent;
pub use comms::CommsSystem;
pub use combat::CombatEngine;
pub use gauge::Gauge;
pub use manual::ManualLibrary;
pub use npc::NpcConfig;
pub use permission::Permission as PermissionLevel;
pub use room::RoomGraph;
