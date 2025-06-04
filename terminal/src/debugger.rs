//! ===============================================
//! 📜 Metadata — OmniDebug v0.0.1
//! ===============================================
//! _author_:        Seanje Lenox-Wise / Nova Dawn
//! _version_:       0.0.1
//! _status_:        Dev
//! _created_:       2025-06-03
//! _last updated_:  2025-06-03
//! _license_:       CreativeWorkzStudio LLC — Kingdom-First Proprietary Use
//! _component_:     Debug Scoring + Report Framework
//! _project_:       OmniCode / Millennium OS
//! _description_:   Scored debugging system with structured log output
//!
//! _notes_:  
//! - Not just an error catcher, but an alignment assessor
//! - Designed to scale alongside custom OmniCode interpreter
//! ===============================================

// ===============================================
// 🌀 Opening — Imports & Declarations
// ===============================================

// std::fmt:
// For formatting custom enums and debug display logic
// use std::fmt;

// chrono::Utc:
// To timestamp each debug log entry
use chrono::Utc;

// std::fs and std::io:
// Used for writing logs to disk
use std::fs::OpenOptions;
use std::io::{self, Write};

// std::path::Path:
// Used to validate and resolve log paths
// use std::path::Path;

// serde (serialization support):
// Enables JSON-ready debug reports
use serde::{Deserialize, Serialize};

// serde_json:
// Used to serialize structured logs to JSON format
use serde_json;

// ===============================================
// 🔧 Body — Core Scoring + Log Infrastructure
// ===============================================

// ===============================================
// 🧭 Severity Enum — 10-Point Diagnostic Classes
// ===============================================

/// 🎯 `Severity` captures diagnostic health in 10-point intervals.
#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    Fatal,       // 0–9   🛑 Collapse / irreparable failure
    Critical,    // 10–19 🔥 Emergency systemic failure
    Error,       // 20–29 ❌ Major logical break
    Fault,       // 30–39 🧯 Recoverable issue
    Weakness,    // 40–49 ⚠️ Minor vulnerability
    Instability, // 50–59 🌀 Unstable but functional
    Degraded,    // 60–69 🚧 Reduced capacity
    Drift,       // 70–79 🧭 Slight divergence
    Info,        // 80–89 ℹ️ Almost aligned
    Pass,        // 90–100 ✅ Full alignment
}

// ===============================================
// 🧪 DebugResponse — What To Do With This Finding
// ===============================================

#[derive(Debug, Serialize, Deserialize)]
pub enum DebugResponse {
    Ignore, // 🚫 Skip
    Retry,  // 🔁 Reattempt operation
    Halt,   // 🛑 Stop system
    Patch,  // 🩹 Auto-correct
    Prompt, // ❓ Ask for input
}

// ===============================================
// 📋 DebugEntry — Scored Snapshot of System State
// ===============================================

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugEntry {
    pub command: String,             // 🔑 Operation under test
    pub input: String,               // 📤 Raw input
    pub expected: String,            // ✅ Expected output
    pub actual: String,              // 📥 Observed output
    pub discrepancy: Option<String>, // ⚠️ Difference summary
    pub location: Option<String>,    // 📚 File or system area
    pub suggestions: Vec<String>,    // 🛠 Fixes, hints, or guidance notes
    pub response: DebugResponse,     // 📨 What to do next
    pub score: u8,                   // 🌡 0–100 alignment
    pub severity: Severity,          // 🚨 Diagnostic band
    pub timestamp: String,           // 🕰 UTC time
}

impl DebugEntry {
    /// 🛠️ Generate a new debug entry and compute alignment score + severity
    pub fn new(command: &str, input: &str, expected: &str, actual: &str) -> Self {
        let discrepancy = if expected != actual {
            Some(format!(
                "Mismatch — expected '{}', got '{}'",
                expected, actual
            ))
        } else {
            None
        };

        // 🔍 Word-based scoring heuristic
        let score = if expected == actual {
            100
        } else {
            let exp_words: Vec<&str> = expected.split_whitespace().collect();
            let act_words: Vec<&str> = actual.split_whitespace().collect();
            let mismatches = exp_words
                .iter()
                .zip(&act_words)
                .filter(|(a, b)| a != b)
                .count();
            100u8.saturating_sub((mismatches * 10) as u8)
        };

        let severity = Self::resolve_severity(score);
        let timestamp = Utc::now().to_rfc3339();

        DebugEntry {
            command: command.to_string(),
            input: input.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
            discrepancy,
            location: None,
            suggestions: vec![],
            response: DebugResponse::Prompt,
            score,
            severity,
            timestamp,
        }
    }

    /// 🧭 Classify score range into severity
    fn resolve_severity(score: u8) -> Severity {
        match score {
            0..=9 => Severity::Fatal,
            10..=19 => Severity::Critical,
            20..=29 => Severity::Error,
            30..=39 => Severity::Fault,
            40..=49 => Severity::Weakness,
            50..=59 => Severity::Instability,
            60..=69 => Severity::Degraded,
            70..=79 => Severity::Drift,
            80..=89 => Severity::Info,
            90..=100 => Severity::Pass,
            _ => Severity::Info,
        }
    }

    /// ➕ Chain a location to this entry
    pub fn with_location(mut self, loc: &str) -> Self {
        self.location = Some(loc.to_string());
        self
    }

    /// ➕ Add a suggestion to this entry
    // pub fn add_suggestion(mut self, note: &str) -> Self {
    //    self.suggestions.push(note.to_string());
    //    self
    // }

    pub fn with_suggestion(mut self, note: &str) -> Self {
        self.suggestions.push(note.to_string());
        self
    }

    /// 📜 Format as scroll
    pub fn to_scroll(&self) -> String {
        let mut block = format!(
            "\
==============================================
📜 OmniDebug Scroll — {}
==============================================
🔑 Command:     {}
📤 Input:       {}
✅ Expected:    {}
📥 Actual:      {}
🌡 Score:       {}/100
🚨 Severity:    {:?}",
            self.timestamp,
            self.command,
            self.input,
            self.expected,
            self.actual,
            self.score,
            self.severity
        );

        if let Some(ref d) = self.discrepancy {
            block += &format!("\n⚠️ Discrepancy:  {}", d);
        }

        if let Some(ref l) = self.location {
            block += &format!("\n📚 Location:     {}", l);
        }

        if !self.suggestions.is_empty() {
            block += "\n🛠 Suggestions:";
            for s in &self.suggestions {
                block += &format!("\n  - {}", s);
            }
        }

        block += &format!("\n📨 Response:     {:?}\n", self.response);
        block
    }

    /// 🧾 Write JSON format to disk
    pub fn write_json(&self, path: &str) -> io::Result<()> {
        // 🌱 Ensure parent directories exist
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let serialized = serde_json::to_string_pretty(&self)?;
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", serialized)?;
        Ok(())
    }

    /// 🪶 Write plain-text scroll to disk
    pub fn write_scroll(&self, path: &str) -> io::Result<()> {
        // 🌱 Ensure parent directories exist
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", self.to_scroll())?;
        Ok(())
    }
}

// ===================================================
// 🔚 Closing — Scroll Finalization & Writer Hooks
// ===================================================
//
// ✅ OmniDebug supports two write modes:
//    - `write_scroll()` for human-readable summaries
//    - `write_json()` for structured machine parsing
//
// ✅ Diagnostic metadata includes alignment, severity,
//    discrepancy, suggestions, and UTC timestamp.
//
// ⚠️ Current implementation uses `append` mode:
//    - Multiple logs may be written in one session
//    - No deduplication or overwrite guards exist yet
//
// ---------------------------------------------------
// 🧾 Change Policy Notice:
// ---------------------------------------------------
//   This file is governed by the OmniCode Scroll Protocol.
//   All structural changes (logic, scoring, or format)
//   must be versioned in the metadata block above.
//   Comments marked with ⚠️ or 📌 denote logic-sensitive areas.
//
// ---------------------------------------------------
// 📅 Last Known Version
// ---------------------------------------------------
//   Version       : v0.0.1
//   Last Updated  : 2025-06-03
//   Change Log    : Initial scoring engine + log writing system
//
// ---------------------------------------------------
// 🪧 Notes
// ---------------------------------------------------
// - This system is the diagnostic scribe of OmniCode.
// - Future features may include:
//     • GUI replay timeline with scroll parsing
//     • Hook into live CLI/GUI command cycles
//     • Auto-suggestion based on common drift patterns
//     • Alignment heuristics based on NovaAI learning
//
// ---------------------------------------------------
