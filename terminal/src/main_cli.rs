//! ===============================================
//! 📜 Metadata — OmniShell v0.1 (CLI)
//! ===============================================
//! _author_:        Seanje Lenox-Wise / Nova Dawn  
//! _version_:       0.0.2  
//! _status_:        Dev  
//! _created_:       2025-06-03  
//! _last updated_:  2025-06-03  
//! _license_:       CreativeWorkzStudio LLC — Kingdom-First Proprietary Use  
//! _component_:     CLI Terminal Interface  
//! _project_:       OmniCode / Millennium OS  
//! _description_:   Minimal CLI interface to spawn shell commands using cmd.exe
//!
//! _notes_:  
//! - Acts as the fallback terminal for systems without GUI access  
//! - Designed with simplicity and modular expansion in mind  
//! - Opening, Body, Closing structure used for clarity and scroll logic
//!
//! ===============================================

// ===============================================
// 🌀 Opening — Imports & Declarations
// ===============================================

// std::io::{self, Write}:
// Handles user input from the terminal and ensures output is flushed to the screen promptly
use std::io::{self, Write};

// std::process::{Command, Stdio}:
// Spawns subprocesses via the system shell (cmd.exe) and manages standard I/O streams
use std::process::{Command, Stdio};

mod registry; // 🔗 Link to the internal OmniCommand registry module
use registry::CommandRegistry; // ⛓️ Bring the registry struct into scope

mod debugger; // 🧠 Link to OmniDebug scoring + log module
use debugger::{DebugEntry}; // 📜 Bring core diagnostic structs into scope

// ===============================================
// 🔧 Body — I/O Loop and Command Handling
// ===============================================

/// Entrypoint for OmniShell CLI
///
/// A minimal, resilient command loop built for direct terminal use.
/// Designed to echo GUI parity while keeping it lightweight and offline-ready.
///
/// Function Flow:
///   1️⃣ Greet the user and open the loop
///   2️⃣ Read and sanitize input from stdin
///   3️⃣ Check for exit condition
///   4️⃣ Execute command through Windows shell (cmd.exe)
///   5️⃣ Print both stdout and stderr to screen
fn main() {
    // -----------------------------------------------
    // 1️⃣ Startup — Welcome message to orient user
    // -----------------------------------------------
    println!("OmniShell v0.1 — Kingdom Terminal Ready"); // ✨ Announce tool version

    // -----------------------------------------------
    // ⚙️ Internal Registry — Setup for OmniCommands
    // -----------------------------------------------
    let registry = CommandRegistry::new(); // Loads all internal commands (e.g., 'speak')

    // -----------------------------------------------
    // 🔁 Main Loop — Keeps reading input continuously
    // -----------------------------------------------
    loop {
        print!("> "); // 📝 Input prompt
        io::stdout().flush().unwrap(); // ⏩ Ensure prompt prints before input

        // -----------------------------------------------
        // 2️⃣ Input Handling — Capture user input
        // -----------------------------------------------
        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Failed to read input"); // ⚠️ Basic read failure message
            continue;
        }

        let trimmed = input.trim(); // 🧼 Strip whitespace and newline

        // -----------------------------------------------
        // 3️⃣ Exit Condition — Graceful shutdown
        // -----------------------------------------------
        if trimmed.eq_ignore_ascii_case("exit") {
            println!("Exiting OmniShell..."); // 👋 Exit message
            break;
        }

        // -----------------------------------------------
        // 4️⃣ Internal vs External Command Dispatch
        // -----------------------------------------------
        if let Some(output) = registry.run(trimmed) {
            println!("{}", output); // Internal OmniCommand handled

            // 🧪 OmniDebug Internal Execution Log
            let entry = DebugEntry::new("internal", trimmed, "[depends on command]", &output)
                .with_location("OmniCommand")
                .with_suggestion("Validate command alias output mapping");
            let _ = entry.write_scroll("Logs/Debug/scrolls/omnishell.log");
            let _ = entry.write_json("Logs/Debug/json/omnishell.json");
            continue;
        }

        let result = Command::new("cmd")
            .args(&["/C", trimmed]) // 🪞 Execute single-use shell command
            .stdout(Stdio::piped()) // 📤 Capture standard output
            .stderr(Stdio::piped()) // 📛 Capture error output
            .output(); // 🎬 Perform the execution

        // -----------------------------------------------
        // 5️⃣ Output Handling — Print response or errors
        // -----------------------------------------------
        match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout); // 📄 Decode stdout
                let stderr = String::from_utf8_lossy(&output.stderr); // 🔥 Decode stderr

                print!("{}", stdout); // 🖨️ Display shell result
                eprint!("{}", stderr); // ❗ Display errors, if any

                // 🧪 OmniDebug External Execution Log
                let actual = format!("{}{}", stdout, stderr);
                let entry = DebugEntry::new("external", trimmed, "[manual validation]", &actual)
                    .with_location("cmd.exe")
                    .with_suggestion("Review command structure for escaping or path issues");
                let _ = entry.write_scroll("Logs/Debug/scrolls/omnishell.log");
                let _ = entry.write_json("Logs/Debug/json/omnishell.json");
            }
            Err(e) => {
                eprintln!("Error: {}\n", e); // 🧨 Shell execution failure

                // 🧪 OmniDebug Execution Failure Log
                let entry = DebugEntry::new(
                    "external",
                    trimmed,
                    "[successful output]",
                    "[command failed]",
                )
                .with_location("cmd.exe")
                .with_suggestion("Check system PATH or permissions");
                let _ = entry.write_scroll("Logs/Debug/scrolls/omnishell.log");
                let _ = entry.write_json("Logs/Debug/json/omnishell.json");
            }
        }
    }
}

// ===================================================
// 🔚 Closing — Graceful Exit Conditions & Metadata
// ===================================================
//
// ✅ Exit condition is handled inline via the keyword `"exit"`.
//    - This allows the user to gracefully terminate the shell.
//    - Ensures resources are released, and loop breaks cleanly.
//
// ⚠️ Note: This terminal is currently single-threaded and
//    designed for sequential command execution only.
//
// 📌 No post-loop teardown is required in this version.
//    - Stdout/stderr are flushed automatically.
//    - No persistent session state or background processes.
//
// ---------------------------------------------------
// 🧾 Change Policy Notice:
// ---------------------------------------------------
//   This file is governed by the OmniCode Scroll Protocol.
//   All structural changes (function, logic, or metadata)
//   must be versioned and documented at the top of the scroll.
//   Comments marked with ⚠️ or 📌 denote high-impact areas.
//
// ---------------------------------------------------
// 📅 Last Known Version
// ---------------------------------------------------
//   Version       : v0.1
//   Last Updated  : 2025-06-03
//   Change Log    : Initial CLI loop + graceful exit + command piping
//
// ---------------------------------------------------
// 🪧 Notes
// ---------------------------------------------------
// - This CLI version is intentionally minimal.
// - Future features may include:
//     • Command history
//     • Tab completion
//     • Custom command aliases
//     • Error code display
// - GUI version developed in parallel: `OmniShell GUI v0.1`
//
// ---------------------------------------------------
