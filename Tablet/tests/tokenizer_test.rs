use tablet::tokenizer::{Tokenizer, TokenType};
use tablet::instruction_registry::get_instruction_registry;
use std::collections::HashMap;

#[test]
fn test_tokenize_simple_assignment() {
    let source = r#"let flame = "holy fire""#;
    let registry = get_instruction_registry()
        .iter()
        .map(|(k, _)| (k.to_string(), TokenType::Instruction))
        .collect::<HashMap<String, TokenType>>();
    let mut tokenizer = Tokenizer::new(source, registry);

    let stream = tokenizer.tokenize();
    let tokens = stream.tokens;

    assert_eq!(tokens.len(), 4);

    assert_eq!(tokens[0].value, "let");
    assert_eq!(tokens[0].token_type, TokenType::Instruction);

    assert_eq!(tokens[1].value, "flame");
    assert_eq!(tokens[1].token_type, TokenType::Identifier);

    assert_eq!(tokens[2].value, "=");
    assert_eq!(tokens[2].token_type, TokenType::Operator);

    assert_eq!(tokens[3].value, "holy fire");
    assert_eq!(tokens[3].token_type, TokenType::Literal);
}

#[test]
fn test_tokenize_comment_and_metadata() {
    let source = r#"
# just a comment
#! engine: OmniCore
"#;
    let registry = get_instruction_registry()
        .iter()
        .map(|(k, _)| (k.to_string(), TokenType::Instruction))
        .collect::<HashMap<String, TokenType>>();
    let mut tokenizer = Tokenizer::new(source, registry);
    let stream = tokenizer.tokenize();
    let tokens = stream.tokens;

    assert_eq!(tokens.len(), 2);

    assert_eq!(tokens[0].token_type, TokenType::Comment);
    assert!(tokens[0].value.contains("just a comment"));

    assert_eq!(tokens[1].token_type, TokenType::Metadata);
    assert!(tokens[1].value.contains("engine"));
}

#[test]
fn test_tokenize_grouping_and_number() {
    let source = r#"bless(42)"#;
    let registry = get_instruction_registry()
        .iter()
        .map(|(k, _)| (k.to_string(), TokenType::Instruction))
        .collect::<HashMap<String, TokenType>>();
    let mut tokenizer = Tokenizer::new(source, registry);
    let stream = tokenizer.tokenize();
    let tokens = stream.tokens;

    assert_eq!(tokens.len(), 4);

    assert_eq!(tokens[0].value, "bless");
    assert_eq!(tokens[0].token_type, TokenType::Instruction);

    assert_eq!(tokens[1].value, "(");
    assert_eq!(tokens[1].token_type, TokenType::GroupMarker);

    assert_eq!(tokens[2].value, "42");
    assert_eq!(tokens[2].token_type, TokenType::Literal);

    assert_eq!(tokens[3].value, ")");
    assert_eq!(tokens[3].token_type, TokenType::GroupMarker);
}
