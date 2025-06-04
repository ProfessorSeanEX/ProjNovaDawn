//! ===============================================
//! 📜 Metadata — OmniCode Terminal v0.1 (GUI)
//! ===============================================
//! _author_:        Seanje Lenox-Wise / Nova Dawn  
//! _version_:       0.0.2  
//! _status_:        Dev  
//! _created_:       2025-06-03  
//! _last updated_:  2025-06-03  
//! _license_:       CreativeWorkzStudio LLC — Kingdom-First Proprietary Use  
//! _component_:     GUI Terminal Interface  
//! _project_:       OmniCode / Millennium OS  
//! _description_:   Graphical terminal UI for spawning cmd.exe commands
//!
//! _notes_:  
//! - Built using `eframe` (egui framework) for native rendering  
//! - Operates asynchronously to preserve UI responsiveness  
//! - Opening, Body, Closing structure used for clarity and scroll logic  
//! ===============================================

// ===============================================
// 🌀 Opening — Imports & Declarations
// ===============================================

// eframe (egui + framework integration):
// Provides the core application shell and GUI engine
use eframe::{egui, App, CreationContext};

// std::process::Command & Stdio:
// For spawning system-level shell commands (via "cmd")
// and capturing their standard output and error streams
use std::process::{Command, Stdio};

// std::sync::mpsc (multi-producer, single-consumer):
// Enables communication between the GUI thread and the command execution thread
use std::sync::mpsc::{channel, Receiver, Sender};

// std::thread:
// Used to spawn a background thread that handles command execution asynchronously
use std::thread;

mod registry; // 🔗 Link to the internal OmniCommand registry module
use registry::CommandRegistry; // ⛓️ Import the command registry for OmniCommands

mod debugger; // 🧪 Link to OmniDebug diagnostics module
use debugger::DebugEntry; // 📜 Import primary debug structure

// ===============================================
// 🔧 Body — TerminalApp Struct & GUI Logic
// ===============================================

/// `TerminalApp` governs the GUI layer of OmniShell,
/// stewarding all user input, shell output, and async messaging.
///
/// This struct serves as the live interface between human commands
/// and system execution—designed for real-time feedback, expansion
/// into themed terminals, OS-level hooks, or embedded shell layers.
struct TerminalApp {
    input: String,              // 🔤 Holds text input typed by the user
    output: String,             // 📜 Cumulative shell output shown in scroll area
    sender: Sender<String>,     // 📤 Channel: UI → Shell executor thread
    receiver: Receiver<String>, // 📥 Channel: Shell thread → UI for display
    registry: CommandRegistry,  // 📦 Holds internal OmniCommand logic (e.g., 'speak')
}

impl TerminalApp {
    /// Initializes a fresh GUI terminal instance (`TerminalApp::new`)
    ///
    /// Sets up communication channels and launches a persistent thread
    /// that handles background execution of commands via Windows `cmd.exe`.
    ///
    /// Command responses are streamed back to the UI for display,
    /// allowing real-time feedback in a responsive, scrollable terminal.
    fn new(_cc: &CreationContext<'_>) -> Self {
        // -----------------------------------------------
        // 1️⃣ Channel Setup — UI <=> Shell Communication
        // -----------------------------------------------
        let (tx, rx) = channel::<String>(); // UI → Command executor thread
        let (tx_out, rx_out) = channel::<String>(); // Command output → UI renderer

        // -----------------------------------------------
        // 2️⃣ Background Thread — Command Processing Loop
        // -----------------------------------------------
        thread::spawn(move || {
            while let Ok(cmd) = rx.recv() {
                let expected = "<user expectation>"; // 📌 Placeholder — define per-use or leave empty
                let input = cmd.clone(); // Save raw input before trimming or execution

                // -----------------------------------------------
                // 3️⃣ Shell Execution — Windows cmd (/C)
                // -----------------------------------------------
                let result = Command::new("cmd")
                    .args(&["/C", &cmd])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output();

                // -----------------------------------------------
                // 4️⃣ Output Formatting + Debug Logging
                // -----------------------------------------------
                let (output, _actual) = match result {
                    Ok(output) => {
                        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                        let merged = format!("{}{}", stdout, stderr);

                        // 📜 Log debug entry
                        let debug = DebugEntry::new(&cmd, &input, expected, &merged)
                            .with_location("TerminalApp::new")
                            .with_suggestion("Review command output for minor drift");

                        let _ = debug.write_scroll("Logs/Debug/scrolls/omnishell_gui.log");
                        let _ = debug.write_json("Logs/Debug/json/omnishell_gui.json");

                        (merged, stdout)
                    }
                    Err(e) => {
                        let fail = format!("Error: {}\n", e);

                        // 🧪 Log failure condition
                        let debug = DebugEntry::new(&cmd, &input, expected, &fail)
                            .with_location("TerminalApp::new")
                            .with_suggestion("Shell execution failure");

                        let _ = debug.write_scroll("Logs/Debug/scrolls/omnishell_gui.log");
                        let _ = debug.write_json("Logs/Debug/json/omnishell_gui.json");

                        (fail, String::new())
                    }
                };

                let _ = tx_out.send(output);
            }
        });

        // -----------------------------------------------
        // ✅ Final Return — TerminalApp Instance Ready
        // -----------------------------------------------
        Self {
            input: String::new(),             // 🆕 Start with an empty input buffer
            output: String::new(),            // 📭 Start with no output displayed
            sender: tx,                       // 🔗 Store sender for sending new commands
            receiver: rx_out,                 // 🔗 Store receiver for listening to output
            registry: CommandRegistry::new(), // 🏗️ Construct internal registry during setup
        }
    }
}

// ===============================================
// 🧠 UI Logic — Implements egui Application Trait
// ===============================================

impl App for TerminalApp {
    /// Renders and updates the OmniCode Terminal GUI each frame.
    ///
    /// Defines full interface logic: layout, interaction, async output handling,
    /// and live repaint to ensure responsiveness. This is the beating heart of the shell.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // -------------------------------------------------------
            // 1️⃣ Header — Terminal Title and Top Divider
            // -------------------------------------------------------
            ui.heading("OmniCode Terminal"); // 🧭 Terminal banner
            ui.separator(); // ──── visual break

            // -------------------------------------------------------
            // 2️⃣ Output Scroll — Shows All Accumulated Responses
            // -------------------------------------------------------
            ui.label("Output:"); // 📤 Output section label
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(&self.output); // 📜 Display all terminal output
            });

            ui.separator(); // ━━━ Transition to input controls

            // -------------------------------------------------------
            // 3️⃣ Input Line — Command Field and Execution Button
            // -------------------------------------------------------
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input); // ⌨️ Editable input field
                if ui.button("Run").clicked() {
                    let command = self.input.trim(); // 🧹 Clean input first

                    // 🧠 Internal OmniCommand Dispatch
                    if let Some(response) = self.registry.run(command) {
                        self.output.push_str(&format!("{}\n", response)); // 🪶 Append internal result
                        self.input.clear(); // 🔄 Clear input field
                        return;
                    }

                    // 🪟 External Command Dispatch
                    let _ = self.sender.send(command.to_string()); // ✉️ Send to backend executor
                    self.input.clear(); // 🔄 Clear input field
                }
            });

            // -------------------------------------------------------
            // 4️⃣ Poll Output — Async Shell Response Reception
            // -------------------------------------------------------
            if let Ok(response) = self.receiver.try_recv() {
                let debug_note =
                    format!("\n[🧪 Debug entry logged — see /Logs/Debug for details]\n");
                self.output.push_str(&format!("{}{}", response, debug_note));
            }
        });

        // -------------------------------------------------------
        // 5️⃣ Repaint Request — Keep UI Responsive and Live
        // -------------------------------------------------------
        ctx.request_repaint(); // ♻️ Triggers redraw even when idle
    }
}

// ===================================================
// 🔚 Closing — Entry Point Execution & Metadata
// ===================================================
//
// 🏁 Entrypoint for OmniCode Terminal GUI.
//     - Launches `eframe` application with default native options.
//     - Initializes `TerminalApp` within GUI shell context.
//
// 🚪 Exit behavior is handled internally by `egui` lifecycle.
//     - No manual shutdown logic required.
//     - All async handlers and repaint loops are self-contained.
//
// ⚙️ Engine: `eframe` + `egui` for native GUI rendering.
//     - Current rendering is single-window and non-threaded.
//     - Designed for light, standalone terminals.
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
//   Change Log    : Initial GUI launch scaffold using eframe
//
// ---------------------------------------------------
// 🪧 Notes
// ---------------------------------------------------
// - This GUI version complements the CLI terminal.
// - Future GUI upgrades may include:
//     • Output auto-scrolling
//     • Command result formatting (colors, timestamps)
//     • Persistent terminal session memory
//     • Tabbed interfaces or workspace scenes
//
// ---------------------------------------------------

/// Entrypoint for launching the OmniCode Terminal GUI interface.
///
/// Initializes the native window using `eframe::run_native` with default options.
/// Wraps the `TerminalApp` inside a boxed closure passed to the runtime.
///
/// Behavior:
///   - Delegates control to egui's event/render loop
///   - Handles all UI logic within `TerminalApp::update`
///   - No explicit teardown required; cleanup is handled by framework
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default(); // Default window configuration
    eframe::run_native(
        "OmniCode Terminal v0.1 GUI",                      // Window title
        options,                                           // Pass native options
        Box::new(|cc| Ok(Box::new(TerminalApp::new(cc)))), // App initializer
    )
}
