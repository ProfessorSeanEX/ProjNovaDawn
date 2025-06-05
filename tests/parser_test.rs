use gate::parser::*;
use gate::tokenizer::{Token, TokenType};

fn token(t: TokenType, value: &str) -> Token {
    Token {
        token_type: t,
        value: value.to_string(),
        line: 0,
        column: 0,
    }
}

#[test]
fn test_instruction_with_args() {
    let tokens = vec![
        token(TokenType::Instruction, "invoke"),
        token(TokenType::Literal, "\"truth\""),
        token(TokenType::Operator, "+5"),
    ];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_node().unwrap();

    match node {
        ScrollNode::Instruction { name, args } => {
            assert_eq!(name, "invoke");
            assert_eq!(args, vec!["\"truth\"", "+5"]);
        }
        _ => panic!("Expected Instruction node"),
    }
}

#[test]
fn test_scroll_sentence_parsing() {
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

#[test]
fn test_assignment_parsing() {
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
    let tokens = vec![
        token(TokenType::Identifier, "call"),
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
            assert_eq!(function, "call");
            assert_eq!(args, vec!["\"grace\"", "\"mercy\""]);
        }
        _ => panic!("Expected Call node"),
    }
}

#[test]
fn test_parse_block() {
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
fn test_parse_comment() {
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
    let tokens = vec![token(TokenType::Metadata, "//! scroll information")];
    let mut parser = Parser::new(tokens);
    let node = parser.parse_metadata().unwrap();

    match node {
        ScrollNode::Metadata(data) => assert_eq!(data, "//! scroll information"),
        _ => panic!("Expected Metadata"),
    }
}

#[test]
fn test_parse_loop() {
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

#[test]
fn test_sentence_validation() {
    let parser = Parser::new(vec![]);
    assert!(parser.is_valid_sentence("Jesus", "heals", Some("the blind")));
    assert!(!parser.is_valid_sentence("", "speaks", Some("truth")));
}
