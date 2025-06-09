//! Watchtower — The Debugger and Alignment Monitor
//! This module logs scroll misalignments, fallback recoveries, and system scoring.

pub mod debugger;
pub mod alignment_score;

pub fn watchtower_status() -> &'static str {
    "🛡 Watchtower module standing guard."
}
