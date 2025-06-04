//! ===============================================
//! 📜 Metadata — OmniCommand Registry Module
//! ===============================================
//! _author_:        Seanje Lenox-Wise / Nova Dawn
//! _version_:       0.0.1
//! _status_:        Dev
//! _created_:       2025-06-03
//! _last updated_:  2025-06-03
//! _license_:       CreativeWorkzStudio LLC — Kingdom-First Proprietary Use
//! _component_:     Internal Command Registry
//! _project_:       OmniCode / Millennium OS
//! _description_:   Central registry for internal OmniCommands, used by CLI/GUI
//!
//! _notes_:  
//! - Built for extensibility: register additional commands on init  
//! - Designed to be hot-swappable within terminals, editors, or shells  
//! - Pure Rust, no external runtime dependencies  
//! ===============================================

// ===============================================
// 🌀 Opening — Imports & Declarations
// ===============================================

// std::collections::HashMap:
// Provides a hash-based key/value storage used for registering and dispatching OmniCommands
use std::collections::HashMap;

// crate-local DebugEntry module (for OmniDebug integration)
// This assumes `debugger.rs` is in the same crate/module tree
// use crate::debugger::DebugEntry; // 🧭 Optional: Only needed if run_debuggable uses DebugEntry directly

// ===============================================
// 🔧 Body — Traits, Commands, and Registry Logic
// ===============================================

/// 🎛️ `OmniCommand` — Foundational Trait for All Internal Commands
///
/// This trait defines the core behavior contract for every internal OmniCode command.
/// Implementations must define:
/// - `name()` → The unique keyword used to invoke the command
/// - `execute()` → The function triggered when the command is run, passed with string arguments
///
/// This trait allows commands to be registered dynamically and dispatched uniformly
/// in both CLI and GUI environments.
pub trait OmniCommand {
    fn name(&self) -> &str;                     // 🏷️ Command name used for matching (e.g., "speak")
    fn execute(&self, args: &[&str]) -> String; // 🧠 Command logic that consumes input arguments
}

// -----------------------------------------------
// 🧪 Built-In Command #1 — `speak` (Echo Behavior)
// -----------------------------------------------

/// 🗣️ `SpeakCommand` — Minimal Echo Handler for Input Arguments
///
/// Purpose:
/// - This command takes all arguments passed after the keyword `speak`
///   and returns them as a single joined string.
/// - Used as a **test prototype** to verify registry, parsing, and execution flow.
///
/// Example Usage:
/// ```bash
/// > speak Hello World
/// Hello World
/// ```
pub struct SpeakCommand;

impl OmniCommand for SpeakCommand {
    fn name(&self) -> &str { "speak" } // 🏷️ Invocation keyword ("speak")

    fn execute(&self, args: &[&str]) -> String {
        let output = args.join(" "); // 📤 Outputs all arguments as a space-separated string
        // 🔍 Optional future integration:
        // Send `DebugEntry::new("speak", &args.join(" "), &output, &output)` to logger here
        output
    }
}

// -----------------------------------------------
// 🧭 Registry — Internal Command Dispatcher
// -----------------------------------------------

/// 📦 `CommandRegistry` — Central Dispatch for OmniCommands
///
/// This struct acts as the **internal router** for all commands registered into the system.
/// Used by both CLI and GUI terminals to route user-entered commands
/// to their respective implementations.
///
/// Internally stores commands in a `HashMap` keyed by their invocation name.
pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn OmniCommand>>, // 🗂️ Registry: command name → command object
}

impl CommandRegistry {
    // -----------------------------------------------
    // 1️⃣ Create Registry — Register Built-in Commands
    // -----------------------------------------------

    /// 🔧 `new()` — Constructs a fresh registry instance
    ///
    /// - Initializes the command map.
    /// - Automatically registers all known built-in commands.
    /// - Future expansion: load dynamic commands from file or plug-in source.
    pub fn new() -> Self {
        let mut registry = CommandRegistry {
            commands: HashMap::new(), // 🧺 Start empty
        };

        // 🧩 Register each built-in OmniCommand here
        registry.register(Box::new(SpeakCommand)); // 🔌 Adds 'speak' into the registry

        registry
    }

    // -----------------------------------------------
    // 2️⃣ Register — Add New OmniCommand to Table
    // -----------------------------------------------

    /// 🧬 `register()` — Adds a new OmniCommand to the registry
    ///
    /// - Inserts command using its `name()` as the key.
    /// - Overwrites any existing entry with the same name (intended behavior).
    pub fn register(&mut self, cmd: Box<dyn OmniCommand>) {
        self.commands.insert(cmd.name().to_string(), cmd); // 🧷 Bind name → behavior
    }

    // -----------------------------------------------
    // 3️⃣ Run — Attempt Command Execution if Matched
    // -----------------------------------------------

    /// 🚀 `run()` — Attempts to execute a registered command
    ///
    /// - Parses input into command + arguments.
    /// - If the command is found, it delegates execution and returns result.
    /// - If no match is found or input is empty, returns `None`.
    ///
    /// Example:
    /// ```rust
    /// registry.run("speak Hello World"); // Some("Hello World")
    /// ```
    pub fn run(&self, input: &str) -> Option<String> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect(); // 🧹 Sanitize input into words
        let (cmd, args) = parts.split_first()?; // ❓ Handle case where no input was given
        let output = self.commands.get(*cmd)?.execute(args); // ✅ Dispatch if valid command

        // 🎯 Optional debug integration could go here:
        // let debug_entry = DebugEntry::new(*cmd, &input, &output, &output);
        // let _ = debug_entry.write_scroll("Logs/Debug/scrolls/internal.omni.log");

        Some(output)
    }
}

// ===================================================
// 🔚 Closing — Registry Boundaries & Expansion Notes
// ===================================================
//
// ✅ This module contains no teardown logic by design.
//    - `CommandRegistry` is self-contained and stateless.
//    - Commands execute inline and return plain `String` outputs.
//
// 🧩 Expansion Strategy:
//    - Future OmniCommands should implement `OmniCommand` trait.
//    - Register all commands in `CommandRegistry::new()`.
//    - Consider grouping commands by purpose (e.g., shell, dev, AI).
//
// ---------------------------------------------------
// 🧾 Change Policy Notice:
// ---------------------------------------------------
//    - This file is governed by the OmniCode Scroll Protocol.
//    - All structural or logic changes must be versioned in metadata.
//
// ---------------------------------------------------
// 📅 Last Known Version
// ---------------------------------------------------
//    - Version       : v0.0.1
//    - Last Updated  : 2025-06-03
//    - Change Log    : Initial command system scaffolding + `speak` registered
//
// ---------------------------------------------------
// 🪧 Notes:
// ---------------------------------------------------
//    - This file should remain hot-swappable and minimal.
//    - Later registries may inject context or shared state.
//    - Command results should remain pure-returned, not side-effect driven.
//
// ---------------------------------------------------
