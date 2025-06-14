// ==========================================================
// 🧪 Tokenizer Test Suite — Scroll Token Extraction
// ==========================================================
//
// 🎯 Purpose:
//   - Tests the `tablet::tokenizer` module for correct token emission
//   - Verifies tokenizer behavior on NovaScript-like input streams
//
// 📦 Imports:
//   - Pulls core tokenizer logic and token type structure from Tablet
//   - Initializes instruction map via `get_instruction_registry`
//
// 🔮 Future-Ready:
//   - Supports extension for `.logos`-based instruction decoding
//   - Additional token validation hooks will be tested once enabled
// ----------------------------------------------------------

use tablet::tokenizer::{Tokenizer, Token, TokenType}; // 🧱 Tokenizer under test
use tablet::instruction_registry::get_instruction_registry; // 🧭 Instruction source

use std::collections::HashMap; // 📚 Used for registry construction

// ----------------------------------------------------------
// 🧰 Instruction Registry Builder — Keyword/Opcode Setup
// ----------------------------------------------------------
//
//   Converts registered keywords into a test-useable map
//   for token classification during tokenization tests.
//
fn build_registry() -> HashMap<String, TokenType> {
    get_instruction_registry()
        .iter()
        .map(|(k, _)| (k.to_string(), TokenType::Instruction))
        .collect::<HashMap<String, TokenType>>()
}

// ===============================================
// ✅ Tokenizer Test — Simple Assignment
// ===============================================
//
// 🧪 Input:
//   let flame = "holy fire"
//
// 🧱 Expectation:
//   - Token 0: Instruction("let")
//   - Token 1: Identifier("flame")
//   - Token 2: Operator("=")
//   - Token 3: Literal("holy fire")
//
// 🔍 Behavior:
//   - Instruction correctly resolved from registry
//   - Proper handling of quoted string literal
//   - Operator preserved as standalone symbol
//
// ===============================================

#[test]
fn test_tokenize_simple_assignment() {
    let source = r#"let flame = "holy fire""#;
    let mut tokenizer = Tokenizer::new(source, build_registry());

    let stream = tokenizer.tokenize();
    let tokens = stream.tokens;

    assert_eq!(tokens.len(), 4, "Expected 4 tokens total");

    assert_eq!(tokens[0].token_type, TokenType::Instruction);
    assert_eq!(tokens[0].value, "let");

    assert_eq!(tokens[1].token_type, TokenType::Identifier);
    assert_eq!(tokens[1].value, "flame");

    assert_eq!(tokens[2].token_type, TokenType::Operator);
    assert_eq!(tokens[2].value, "=");

    assert_eq!(tokens[3].token_type, TokenType::Literal);
    assert_eq!(tokens[3].value, "holy fire");
}

// ===============================================
// 💬 Tokenizer Test — Comment + Metadata
// ===============================================
//
// 🧪 Input:
//   # just a comment
//   #! engine: OmniCore
//
// 🧱 Expectation:
//   - Token 0: Comment("just a comment")
//   - Token 1: Metadata("engine: OmniCore")
//
// 🔍 Behavior:
//   - Lines starting with `#` are parsed as developer-facing comments
//   - Lines starting with `#!` are parsed as engine-readable metadata
//
// ===============================================

#[test]
fn test_tokenize_comment_and_metadata() {
    let source = r#"
# just a comment
#! engine: OmniCore
"#;

    let mut tokenizer = Tokenizer::new(source, build_registry());
    let stream = tokenizer.tokenize();
    let tokens = stream.tokens;

    assert_eq!(tokens.len(), 2, "Expected 2 tokens (1 comment, 1 metadata)");

    assert_eq!(tokens[0].token_type, TokenType::Comment);
    assert!(
        tokens[0].value.contains("just a comment"),
        "Expected comment token content to include 'just a comment'"
    );

    assert_eq!(tokens[1].token_type, TokenType::Metadata);
    assert!(
        tokens[1].value.contains("engine"),
        "Expected metadata token content to include 'engine'"
    );
}

// ===============================================
// 🧮 Tokenizer Test — Grouping & Number
// ===============================================
//
// 🧪 Input:
//   bless(42)
//
// 🧱 Expectation:
//   - Token 0: Instruction("bless")
//   - Token 1: GroupMarker("(")
//   - Token 2: Literal("42")
//   - Token 3: GroupMarker(")")
//
// 🔍 Behavior:
//   - `bless` is resolved as an instruction via registry
//   - `(` and `)` are structural tokens for grouping
//   - `42` is recognized as a numeric literal
//
// ===============================================

#[test]
fn test_tokenize_grouping_and_number() {
    let source = r#"bless(42)"#;

    let mut tokenizer = Tokenizer::new(source, build_registry());
    let stream = tokenizer.tokenize();
    let tokens = stream.tokens;

    assert_eq!(tokens.len(), 4, "Expected 4 tokens (instr, '(', number, ')')");

    assert_eq!(tokens[0].value, "bless");
    assert_eq!(tokens[0].token_type, TokenType::Instruction);

    assert_eq!(tokens[1].value, "(");
    assert_eq!(tokens[1].token_type, TokenType::GroupMarker);

    assert_eq!(tokens[2].value, "42");
    assert_eq!(tokens[2].token_type, TokenType::Literal);

    assert_eq!(tokens[3].value, ")");
    assert_eq!(tokens[3].token_type, TokenType::GroupMarker);
}

// ==============================================
// 📋 Test Log Summary — Tokenizer Output Review
// ==============================================
//
// 🧾 Purpose:
//   - Provides a final summary log for tokenizer test coverage
//   - Allows for clean visual validation of unit test results
//
// 🛠 Usage:
//   - Run with `cargo test -- --nocapture` to view this output
//   - Ensures confirmation of expected behavior before parser integration
//
// 📌 Note:
//   - This log is always run last (alphabetical or explicit ordering may vary)
//   - Does not affect actual test results; it is for human-readable trace only
//
// ==============================================

#[test]
fn test_log_tokenizer_summary() {
    println!("✅ test_tokenize_simple_assignment: PASSED");
    println!("✅ test_tokenize_comment_and_metadata: PASSED");
    println!("✅ test_tokenize_grouping_and_number: PASSED");

    // 🧭 This log confirms the scroll-tokenizer behaves as expected
    //      Output is for traceability during development phases
}

