# holodeck-core

Standalone MUD engine for room-based agent simulation. Room graph, agents, gauges, scoped communication. Runs anywhere Rust compiles — no CUDA, no external services, no full monolith.

## Brand Line

> holodeck-core is the Cocapn fleet's simulation engine — extracted from holodeck-rust, providing the core MUD (Multi-User Dungeon) primitives that make PLATO rooms real: agents, movement, communication, and time-series gauges.

## Installation

```bash
# From crates.io
cargo install holodeck-core

# Or from source
git clone https://github.com/SuperInstance/holodeck-core.git
cd holodeck-core
cargo build --release
```

## Usage

```rust
use holodeck_core::{Agent, RoomGraph, CommsSystem, CombatEngine, ManualLibrary};

let mut rooms = RoomGraph::new();
rooms.build_default_ship();
let mut agent = Agent::new("oracle1", "harbor");
let mut comms = CommsSystem::new();
let mut combat = CombatEngine::new();
let mut manuals = ManualLibrary::new();

let (response, quit) = agent.handle_command(
    "look", &mut rooms, &mut comms, &mut combat, &mut manuals, &[]
);
println!("{}", response);
```

## Modules

| Module | What it does |
|--------|-------------|
| `agent` | Agent entity with 25+ command dispatcher |
| `room` | Room graph with exits, gauges, boot/leave, data sources |
| `gauge` | Time-series gauge with status tracking (green/yellow/red) |
| `comms` | Scoped communication (say/tell/yell/gossip/notes/mail) |
| `permission` | Role-based access (Greenhorn → Architect) |
| `combat` | Stub — full version in `holodeck-combat` |
| `manual` | Stub — full version in `holodeck-programs` |
| `npc` | Stub — full version in `holodeck-bridge` |

## Why Split?

holodeck-rust was a monolith — MUD engine, combat, poker, AI director, PLATO bridge, NPC refresh, all in one binary. You couldn't run the MUD without pulling in the full dependency tree including `reqwest`, `serde_json` for API calls, and eventually CUDA.

This crate is the core that actually matters for most use cases: rooms, agents, movement, communication. Everything else is optional.

## Fleet Context

Part of the Cocapn fleet. Related repos:

- [plato-sdk](https://github.com/SuperInstance/plato-sdk) — SDK for PLATO room-based agent coordination (uses holodeck for room primitives)
- [holodeck-combat](https://github.com/SuperInstance/holodeck-combat) — Combat engine + evolving scripts (full version of stub)
- [holodeck-programs](https://github.com/SuperInstance/holodeck-programs) — Simulation programs + AI director
- [holodeck-bridge](https://github.com/SuperInstance/holodeck-bridge) — PLATO bridge + NPC refresh + external APIs

## Full Stack

```
holodeck-core       → MUD engine (this crate)
holodeck-combat     → Combat engine + evolving scripts
holodeck-programs   → Simulation programs + AI director
holodeck-ten-forward → Poker + social layer
holodeck-bridge     → PLATO bridge + NPC refresh + external APIs
```

---
🦐 Cocapn fleet — lighthouse keeper architecture