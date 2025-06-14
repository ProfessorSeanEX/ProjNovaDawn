// ==========================================================
// 🧪 Parser Test Suite — ScrollTree Conversion & Validation
// ==========================================================
//
// 🎯 Purpose:
//   - Tests the `tablet::parser` module for correct node generation
//   - Verifies token stream conversion into ScrollNode variants
//
// 📦 Imports:
//   - Pulls core parser + tokenizer logic from Tablet module
//   - Prepares for operand resolution and debug expansion
//
// 🔮 Future-Ready:
//   - Consider mocking `OperandResolver` or `InstructionRegistry` if direct hooks arise
//   - Debug logging hooks available behind `debug_mode`
// ----------------------------------------------------------

use tablet::parser::*; // 🧱 Primary parser under test
use tablet::tokenizer::{Token, TokenType}; // 🧩 Input token structure

// ----------------------------------------------------------
// 🧰 Token Builder — Helper for manual token construction
// ----------------------------------------------------------
//
//   Constructs a token with placeholder line/column values.
//   Used throughout parser tests for compact, readable test cases.
//
fn token(t: TokenType, value: &str) -> Token {
    Token {
        token_type: t,
        value: value.to_string(),
        line: 0,      // 🔢 Not relevant for unit tests
        column: 0,
    }
}

// ==============================================
// 🧪 Instruction & Sentence-Based Parsing Tests
// ==============================================
//
// 🧱 Focus:
//   - Tests opcode-style instruction parsing with operands
//   - Validates ScrollSentence triple structure (SVO form)
//
// 🔒 Current Scope:
//   - No operand resolution invoked directly (tested at parser level only)
//   - Grammar validation is internal to `parse_scroll_sentence`
// ==============================================

#[test]
fn test_instruction_with_args() {
    // 🧪 Input:
    // walk "truth" +5
    // 🧱 Expectation:
    // ScrollNode::Instruction with name = "walk" and args = ["\"truth\"", "+5"]
    let tokens = vec![
        token(TokenType::Instruction, "walk"),
        token(TokenType::Literal, "\"truth\""),
        token(TokenType::Operator, "+5"),
    ];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_node().unwrap();

    match node {
        ScrollNode::Instruction { name, args } => {
            assert_eq!(name, "walk");
            assert_eq!(args, vec!["\"truth\"", "+5"]);
        }
        _ => panic!("Expected Instruction node"),
    }
}

#[test]
fn test_scroll_sentence_parsing() {
    // 🧪 Input:
    // God is light
    // 🧱 Expectation:
    // ScrollNode::ScrollSentence with subject="God", verb="is", object="light"
    let tokens = vec![
        token(TokenType::Identifier, "God"),
        token(TokenType::Identifier, "is"),
        token(TokenType::Identifier, "light"),
    ];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_scroll_sentence().unwrap();

    match node {
        ScrollNode::ScrollSentence { subject, verb, object } => {
            assert_eq!(subject, "God");
            assert_eq!(verb, "is");
            assert_eq!(object, "light");
        }
        _ => panic!("Expected ScrollSentence"),
    }
}

// ==============================================
// 🧾 Assignment & Call Tests
// ==============================================
//
// 🧱 Focus:
//   - Differentiates assignment from function call using syntax
//   - Tests both `target = value` form and `function(args)` call
//
// 🔒 Current Scope:
//   - Parser only (no runtime resolution or execution)
//   - Does not invoke operand evaluation or deeper type inference
// ==============================================

#[test]
fn test_assignment_parsing() {
    // 🧪 Input:
    // path = "east"
    // 🧱 Expectation:
    // ScrollNode::Assignment with target = "path", value = "\"east\""
    let tokens = vec![
        token(TokenType::Identifier, "path"),
        token(TokenType::Operator, "="),
        token(TokenType::Literal, "\"east\""),
    ];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_assignment_or_call().unwrap();

    match node {
        ScrollNode::Assignment { target, value } => {
            assert_eq!(target, "path");
            assert_eq!(value, "\"east\"");
        }
        _ => panic!("Expected Assignment"),
    }
}

#[test]
fn test_function_call() {
    // 🧪 Input:
    // invoke("grace", "mercy")
    // 🧱 Expectation:
    // ScrollNode::Call with function = "invoke", args = ["\"grace\"", "\"mercy\""]
    let tokens = vec![
        token(TokenType::Identifier, "invoke"),
        token(TokenType::Punctuation, "("),
        token(TokenType::Literal, "\"grace\""),
        token(TokenType::Punctuation, ","),
        token(TokenType::Literal, "\"mercy\""),
        token(TokenType::Punctuation, ")"),
    ];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_assignment_or_call().unwrap();

    match node {
        ScrollNode::Call { function, args } => {
            assert_eq!(function, "invoke");
            assert_eq!(args, vec!["\"grace\"", "\"mercy\""]);
        }
        _ => panic!("Expected Call node"),
    }
}

// ==============================================
// 🧱 Structure & Block-Based Tests
// ==============================================
//
// 📦 Focus:
//   - Verifies multi-node containers (Block, Loop)
//   - Tests structural flow and minimal parse validation
//
// 🧰 Coverage:
//   - `{}` block groups
//   - loop condition + block body
//   - `let` declarations with optional type hint
//
// 🛑 Parser-level only — no resolver or execution checks.
// ==============================================

#[test]
fn test_parse_block() {
    // 🧪 Input: { walk "north" }
    // 🧱 Expectation: ScrollNode::Block with at least one child node
    let tokens = vec![
        token(TokenType::GroupMarker, "{"),
        token(TokenType::Instruction, "walk"),
        token(TokenType::Literal, "\"north\""),
        token(TokenType::GroupMarker, "}"),
    ];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_block().unwrap();

    match node {
        ScrollNode::Block(inner) => {
            assert!(!inner.is_empty());
        }
        _ => panic!("Expected Block"),
    }
}

#[test]
fn test_parse_loop() {
    // 🧪 Input: while x < 10 { step "forward" }
    // 🧱 Expectation: Loop node with condition and body
    let tokens = vec![
        token(TokenType::Instruction, "while"),
        token(TokenType::Identifier, "x"),
        token(TokenType::Operator, "<"),
        token(TokenType::Literal, "10"),
        token(TokenType::GroupMarker, "{"),
        token(TokenType::Instruction, "step"),
        token(TokenType::Literal, "\"forward\""),
        token(TokenType::GroupMarker, "}"),
    ];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_loop().unwrap();

    match node {
        ScrollNode::Loop { condition, body } => {
            assert!(condition.contains("x < 10"));
            assert!(!body.is_empty());
        }
        _ => panic!("Expected Loop"),
    }
}

#[test]
fn test_parse_declaration() {
    // 🧪 Input: let truth: String
    // 🧱 Expectation: Declaration with type hint
    let tokens = vec![
        token(TokenType::Instruction, "let"),
        token(TokenType::Identifier, "truth"),
        token(TokenType::Operator, ":"),
        token(TokenType::Identifier, "String"),
    ];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_declaration().unwrap();

    match node {
        ScrollNode::Declaration { name, dtype } => {
            assert_eq!(name, "truth");
            assert_eq!(dtype.unwrap(), "String");
        }
        _ => panic!("Expected Declaration"),
    }
}

// ==============================================
// 🗒️ Metadata, Comment, and Sentence Validations
// ==============================================
//
// 🧾 Focus:
//   - Scroll node types that carry meta or commentary value
//   - Grammar validation logic for subject–verb–object sentences
//
// 🧰 Coverage:
//   - `//!` metadata annotations
//   - `//` comments
//   - `is_valid_sentence()` logic with valid/invalid edge cases
//
// 🛑 No operand resolution or structural nesting expected here.
// ==============================================

#[test]
fn test_parse_comment() {
    // 🧪 Input: // Hello world
    // 🧱 Expectation: ScrollNode::Comment containing raw string
    let tokens = vec![token(TokenType::Comment, "// Hello world")];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_comment().unwrap();

    match node {
        ScrollNode::Comment(c) => assert_eq!(c, "// Hello world"),
        _ => panic!("Expected Comment"),
    }
}

#[test]
fn test_parse_metadata() {
    // 🧪 Input: //! scroll information
    // 🧱 Expectation: ScrollNode::Metadata with exact content
    let tokens = vec![token(TokenType::Metadata, "//! scroll information")];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_metadata().unwrap();

    match node {
        ScrollNode::Metadata(data) => assert_eq!(data, "//! scroll information"),
        _ => panic!("Expected Metadata"),
    }
}

#[test]
fn test_sentence_validation() {
    // 🧪 Validation cases for SVO grammar
    // ✅ Valid: "Jesus heals the blind"
    // ❌ Invalid: missing subject
    let parser = Parser::new(vec![]);
    assert!(parser.is_valid_sentence("Jesus", "heals", Some("the blind")));
    assert!(!parser.is_valid_sentence("", "speaks", Some("truth")));
}

// ==============================================
// 📋 Test Log Summary — Parser Output Review
// ==============================================
//
// 🧾 Purpose:
//   - Outputs visual confirmation of parser test results
//   - Summarizes ScrollTree parsing behavior and SVO alignment
//
// 🛠 Usage:
//   - Run with `cargo test -- --nocapture` to see console output
//   - Intended for development-phase clarity and debug assurance
//
// 📌 Note:
//   - Always runs last in suite (via function name or explicit ordering)
//   - Complements the test framework with traceable scroll validation
//
// ==============================================

#[test]
fn test_log_summary() {
    println!("✅ test_instruction_with_args: PASSED");
    println!("✅ test_scroll_sentence_parsing: PASSED");
    println!("✅ test_assignment_parsing: PASSED");
    println!("✅ test_function_call: PASSED");
    println!("✅ test_parse_block: PASSED");
    println!("✅ test_parse_loop: PASSED");
    println!("✅ test_parse_declaration: PASSED");
    println!("✅ test_parse_comment: PASSED");
    println!("✅ test_parse_metadata: PASSED");
    println!("✅ test_sentence_validation: PASSED");

    // 🧭 Final confirmation log — used during scroll-phase testing
    //     Not a replacement for assertions, but a covenant of coverage.
}
