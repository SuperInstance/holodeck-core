# holodeck-core

Standalone MUD engine extracted from holodeck-rust. Room graph, agents, gauges, scoped communication.

**Runs anywhere Rust compiles.** No CUDA, no external services, no full monolith.

## What's Here

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

## Usage

```rust
use holodeck_core::{Agent, RoomGraph, CommsSystem, CombatEngine, ManualLibrary};

let mut rooms = RoomGraph::new();
rooms.build_default_ship();
let mut agent = Agent::new("oracle1", "harbor");
let mut comms = CommsSystem::new();
let mut combat = CombatEngine::new();
let mut manuals = ManualLibrary::new();

let (response, quit) = agent.handle_command("look", &mut rooms, &mut comms, &mut combat, &mut manuals, &[]);
println!("{}", response);
```

## Why Split?

holodeck-rust was a monolith — MUD engine, combat, poker, AI director, PLATO bridge, NPC refresh, all in one binary. You couldn't run the MUD without pulling in the full dependency tree including `reqwest`, `serde_json` for API calls, and eventually CUDA.

This crate is the core that actually matters for most use cases: rooms, agents, movement, communication. Everything else is optional.

## Full Stack

- `holodeck-core` — this crate (MUD engine)
- `holodeck-combat` — combat engine + evolving scripts
- `holodeck-programs` — simulation programs + AI director
- `holodeck-ten-forward` — poker + social layer
- `holodeck-bridge` — PLATO bridge + NPC refresh + external APIs
