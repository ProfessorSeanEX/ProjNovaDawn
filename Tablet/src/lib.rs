//! Tablet — The Assembler Engine of OmniCode
//! This module handles token resolution, operand matching, and scroll-to-bytecode logic.

pub mod tokenizer;
pub mod parser;
pub mod instruction_registry;
pub mod operand_resolver;

pub fn tablet_status() -> &'static str {
    "📜 Tablet module loaded and ready."
}
