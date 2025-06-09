// ===============================================
// 📖 Millennium Core — lib.rs
// ===============================================
// Central module entrypoint for OmniCode systems.
// Binds together tokenizer, parser, instruction registry, and debug utilities.

pub mod registry; // ✅ This one stays. Terminal command registry.
// use tablet::{parser, tokenizer, instruction_registry};
use watchtower::debugger::{DebugEntry}; // 🧠 Debugging utilities for logging and diagnostics