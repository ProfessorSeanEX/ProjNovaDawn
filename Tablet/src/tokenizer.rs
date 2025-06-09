// ===============================================
// 📜 Metadata - Tokenizer v0.0.1 (Tablet Reader)
// ===============================================
// _author_:        Seanje Lenox-Wise / Nova Dawn  
// _version_:       0.0.1  
// _status_:        Dev
// _created_:       2025-06-04  
// _last updated_:  2025-06-04  
// _license_:       CreativeWorkzStudio LLC — Kingdom-First Proprietary Use  
// _component_:     Tokenizer (Tablet Cog)  
// _project_:       OmniCode / Millennium OS  
// _description_:   Tokenizes `.word`, `.omni`, and `.ns` files into structured tokens for interpretation or compilation.
//
// _notes_:  
// - Parses NovaScript and OmniCode with sacred token structure  
// - Metadata and comments retained for scroll-awareness  
// - Designed for extensibility and AI alignment tracking  
// - Future support: `.logos` registry syncing, macro hooks  
// ===============================================

// ===============================================
// 🌀 Opening — Imports & Declarations
// ===============================================

// std::collections::HashMap:
// Used here for fast lookup of known instruction keywords during token type classification
use std::collections::HashMap;

/// Represents the type/category of a token.
/// Used to determine its role during parsing, interpretation, or debugging.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Whitespace,
    Keyword,     // Reserved NovaScript word (e.g., `let`, `walk`)
    Instruction, // Recognized instruction keyword from `.logos`
    Identifier,  // User-defined name (variable, label, etc.)
    Literal,     // Numeric, char, or string literal (e.g., `"hello"`, `42`)
    Operator,    // Arithmetic or logical symbol (`+`, `-`, `==`)
    Punctuation, // Syntax markers (`;`, `,`, `.`)
    Metadata,    // Line-level metadata marker (e.g., `//`)
    Comment,     // Human-readable comment (`//`, `#`)
    GroupMarker, // Structural tokens (`(`, `)`, `{`, `}`)
    Error,       // Ill-formed or unexpected token (used in diagnostics)
}

/// Core structure representing a token from the source input.
/// Includes token type, captured value, and position metadata.
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType, // Classification of token behavior
    pub value: String,         // Exact source value captured
    pub line: usize,           // Line number where token appears
    pub column: usize,         // Column offset on that line
}

/// Line-level metadata used for whitespace analysis.
/// Enables formatting audits, indent tracking, and code cleanliness checks.
#[derive(Debug)]
pub struct LineMeta {
    pub line_number: usize, // Actual source line index
    pub indentation: usize, // Number of leading spaces/tabs
    pub is_blank: bool,     // True if line contains no tokens
}

/// Result of a tokenization pass over a single source file.
/// Includes all captured tokens, line formatting data, and error tokens.
#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,       // All successful tokens in order
    pub line_meta: Vec<LineMeta>, // Indent/formatting data
    pub errors: Vec<Token>,       // Any misclassified or malformed tokens
}

/// Tokenizer engine for NovaScript / OmniCode source files.
///
/// This tokenizer converts raw `.word`, `.omni`, or `.ns` source code
/// into a stream of structured `Token`s. It tracks spiritual context
/// through metadata and formatting analysis (indentation, comment blocks).
///
/// Designed for scroll-safe parsing and AI-assisted interpretation.
pub struct Tokenizer {
    /// Registry used to identify known keywords and instructions.
    /// This ensures custom opcodes or spiritual commands can be detected.
    pub instruction_registry: HashMap<String, TokenType>,

    /// The raw source code split into characters for fine-grained parsing.
    source: Vec<char>,

    /// The current index in the source vector.
    /// Used to navigate and read characters during parsing.
    position: usize,

    /// The current line number (1-indexed).
    /// Helps with token traceability and error reporting.
    line: usize,

    /// The current column within the current line.
    /// Used to anchor token positions precisely.
    column: usize,

    /// The current indentation level (whitespace before code).
    /// Allows alignment enforcement and style validation.
    current_indent: usize,

    /// Stack of open group markers (e.g., `(` or `{`) for block parsing.
    /// This ensures tokens that need balanced pairing are tracked.
    group_stack: Vec<TokenType>,
}

// ===============================================
// 🔧 Body — Tokenizer Structure & Methods
// ===============================================

impl Tokenizer {
    /// Initializes a new tokenizer from a raw string and keyword map.
    ///
    /// - `source_code`: The raw text to be tokenized.
    /// - `instruction_map`: A preloaded map of known instructions/keywords.
    ///
    /// Returns a tokenizer instance prepared to run `.tokenize()`.
    pub fn new(source_code: &str, instruction_map: HashMap<String, TokenType>) -> Self {
        Self {
            instruction_registry: instruction_map,
            source: source_code.chars().collect(), // Converts string to char stream
            position: 0,
            line: 1,
            column: 0,
            current_indent: 0,
            group_stack: vec![], // Starts with no open groups
        }
    }

    /// Primary method to produce the TokenStream from source input.
    ///
    /// - Iterates character-by-character to extract logical units (tokens).
    /// - Tracks indentation levels, blank lines, and comment blocks.
    /// - Group markers like parentheses are stack-tracked (not yet deeply nested).
    /// - Final result contains tokens, line formatting data, and early errors.
    pub fn tokenize(&mut self) -> TokenStream {
        let mut tokens = vec![]; // Stores all emitted tokens
        let mut line_meta = vec![]; // Stores line-level metadata (indent, blankness)
        let  errors = vec![]; // Error tokens (if needed for reporting later)

        // Core tokenizing loop — walks through each character
        while let Some(ch) = self.peek() {
            match ch {
                // --- Whitespace (indentation only, no token emitted) ---
                ' ' | '\t' => self.consume_whitespace(),

                // --- Newline (resets line/column counters, no token emitted) ---
                '\n' => {
                    self.advance(); // Step forward past newline
                    self.line += 1; // Track next line
                    self.column = 0; // Reset column for new line
                }

                // --- Comments (`#` or `#!`) and Metadata headers ---
                '#' => tokens.push(self.tokenize_comment_or_meta()),

                // --- String Literal (surrounded by double quotes) ---
                '"' => tokens.push(self.tokenize_string()),

                // --- Character Literal (surrounded by single quotes) ---
                '\'' => tokens.push(self.tokenize_char()),

                // --- Operators & Symbols (math, compare, logical) ---
                ':' | '=' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '<' | '>' => {
                    tokens.push(self.tokenize_operator());
                }

                // --- Grouping Symbols (e.g. parentheses) ---
                '(' => {
                    self.group_stack.push(TokenType::GroupMarker); // Push open group to stack
                    tokens.push(self.make_token(TokenType::GroupMarker, "(")); // Emit token
                    self.advance(); // Move past symbol
                }
                ')' => {
                    self.group_stack.pop(); // Pop assumed match
                    tokens.push(self.make_token(TokenType::GroupMarker, ")")); // Emit token
                    self.advance(); // Move past symbol
                }

                // --- Words (instructions, identifiers, keywords) ---
                c if c.is_alphabetic() => tokens.push(self.tokenize_word()),

                // --- Numbers (integer or numeric constants) ---
                c if c.is_numeric() => tokens.push(self.tokenize_number()),

                // --- Unknown or Invalid Character ---
                _ => {
                    tokens.push(self.make_token(TokenType::Error, &ch.to_string())); // Emit error token
                    self.advance(); // Skip unrecognized char
                }
            }
        }

        // ===============================================
        // 🧾 Line Formatting Metadata (indentation map)
        // ===============================================

        let mut line_number = 1;
        for line in self.source.iter().collect::<String>().lines() {
            let indent = line.chars().take_while(|c| c.is_whitespace()).count();

            line_meta.push(LineMeta {
                line_number,
                indentation: indent,
                is_blank: line.trim().is_empty(), // True if only whitespace
            });

            line_number += 1;
        }

        // Return structured token output and diagnostics
        TokenStream {
            tokens,
            line_meta,
            errors,
        }
    }

    // ===============================================
    // 🔧 Subroutines — Core Tokenizer Mechanics
    // ===============================================

    /// Advance the tokenizer cursor by one character.
    ///
    /// Updates both `position` and `column` to maintain source tracking.
    /// Returns the character that was consumed, if available.
    fn advance(&mut self) -> Option<char> {
        let ch = self.source.get(self.position)?; // Safe get with optional fallback
        self.position += 1; // Move position forward
        self.column += 1; // Update column position
        Some(*ch) // Return consumed char
    }

    /// Peek at the current character without consuming it.
    ///
    /// Allows lookahead behavior for complex token structures (e.g., `==`, `!=`).
    fn peek(&self) -> Option<char> {
        self.source.get(self.position).copied() // Get and clone character at position
    }

    /// Constructs a new token with current line/column state.
    ///
    /// All tokens produced should go through this method to ensure accurate metadata.
    fn make_token(&self, token_type: TokenType, value: &str) -> Token {
        Token {
            token_type,               // Enum for what type of token this is
            value: value.to_string(), // Captured raw value (e.g., "let", "42")
            line: self.line,          // Line number at point of emission
            column: self.column,      // Column position at start of token
        }
    }

    /// Skips all contiguous whitespace characters.
    ///
    /// Used during tokenization to manage indentation or ignore gaps.
    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                self.advance(); // Consume known whitespace
            } else {
                break; // Stop at first non-whitespace
            }
        }
    }

    /// Tokenizes either a comment (`#`) or metadata directive (`#!`).
    ///
    /// These are scroll-stable annotations embedded in source code.
    /// - Comments are preserved for scrolls and developer awareness.
    /// - Metadata entries are reserved for engine/system integration.
    fn tokenize_comment_or_meta(&mut self) -> Token {
        let mut content = String::new();

        // Capture until newline or EOF
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            content.push(c); // Add to comment buffer
            self.advance(); // Step past char
        }

        // Determine if this is a metadata directive
        if content.trim_start().starts_with("#!") {
            self.make_token(TokenType::Metadata, &content)
        } else {
            self.make_token(TokenType::Comment, &content)
        }
    }

    // ===============================================
    // 🔣 Token Parsers — Literal, Word, and Operator Handlers
    // ===============================================

    /// Parses a string literal `"..."` with basic escape handling.
    ///
    /// Supported escape codes:
    /// - `\n` → newline
    /// - `\t` → tab
    /// - `\\` → backslash
    /// - `\"` → quote
    /// - `\'` → apostrophe
    fn tokenize_string(&mut self) -> Token {
        let mut content = String::new();
        self.advance(); // Consume opening `"`

        while let Some(c) = self.peek() {
            match c {
                '"' => {
                    self.advance(); // Closing quote
                    break;
                }
                '\\' => {
                    self.advance(); // Consume backslash
                    if let Some(escaped) = self.peek() {
                        // Map escape sequence
                        content.push(match escaped {
                            'n' => '\n',
                            't' => '\t',
                            '\\' => '\\',
                            '"' => '"',
                            '\'' => '\'',
                            _ => escaped, // Unknown escape, keep literal
                        });
                        self.advance(); // Consume escaped character
                    }
                }
                _ => {
                    content.push(c); // Regular character
                    self.advance();
                }
            }
        }

        self.make_token(TokenType::Literal, &content)
    }

    /// Parses a character literal `'c'`.
    ///
    /// No advanced validation yet — assumes well-formed char.
    fn tokenize_char(&mut self) -> Token {
        self.advance(); // Opening `'`
        let ch = self.peek().unwrap_or('�'); // Graceful fallback if empty
        self.advance(); // Consume actual char
        self.advance(); // Consume closing `'`
        self.make_token(TokenType::Literal, &ch.to_string())
    }

    /// Parses one or more operator characters.
    ///
    /// Accepts compound operators like `==`, `+=`, `>>`, etc.
    fn tokenize_operator(&mut self) -> Token {
        let mut content = String::new();
        while let Some(c) = self.peek() {
            if ":=+-*/%&|<>".contains(c) {
                content.push(c); // Collect valid operator chars
                self.advance();
            } else {
                break;
            }
        }
        self.make_token(TokenType::Operator, &content)
    }

    /// Parses a numeric literal (e.g., `42`, `9001`).
    ///
    /// Currently handles only decimal integers.
    fn tokenize_number(&mut self) -> Token {
        let mut num = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num.push(c); // Append digit
                self.advance();
            } else {
                break;
            }
        }
        self.make_token(TokenType::Literal, &num)
    }

    /// Parses a keyword, identifier, or instruction.
    ///
    /// - If matched in the instruction registry, tagged as `Instruction`
    /// - Otherwise, defaults to `Identifier`
    fn tokenize_word(&mut self) -> Token {
        let mut word = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                word.push(c); // Alphanum or underscore allowed
                self.advance();
            } else {
                break;
            }
        }

        let token_type = if self.instruction_registry.contains_key(&word) {
            TokenType::Instruction // Keyword or opcode match
        } else {
            TokenType::Identifier // Regular variable or name
        };

        self.make_token(token_type, &word)
    }

    // ===============================================
    // 🧩 Hooks & Future Validators
    // ===============================================

    /// Validates operand syntax based on the instruction format.
    ///
    /// This is a placeholder for future schema-aware logic.
    /// In the full system, this will:
    /// - Check argument count
    /// - Match operand types (e.g., register, immediate)
    /// - Enforce formatting rules (e.g., separators)
    ///
    /// For now, it always returns true.
    fn validate_instruction_syntax(_keyword: &str, _operands: &[&str]) -> bool {
        true
    }

    /// Post-tokenization hook for token grouping or AST transformation.
    ///
    /// Currently a no-op (returns raw tokens).
    /// Later will handle:
    /// - Nesting of grouped tokens (parens, blocks)
    /// - Macro folding and preprocessing
    /// - Emitting AST nodes or scopes
    fn post_process_tokens(tokens: Vec<Token>) -> Vec<Token> {
        tokens
    }
}

// ===================================================
// 🔚 Closing — Tokenizer Extension Planning & Boundaries
// ===================================================
//
// ✅ This tokenizer module is schema-stable and scoped for MVP parsing.
//    - Handles raw token stream extraction from `.word`, `.omni`, `.ns`
//    - Comments, metadata, and grouping are parsed structurally
//
// 🧩 Expansion Strategy:
//    - Integrate instruction syncing from `.logos` files
//    - Deepen literal handling (escape codes, multi-line strings)
//    - Add grouping structure validation + block tracking
//    - Introduce error trace mapping for `.witness` diagnostics
//
// ---------------------------------------------------
// 🧾 Change Policy Notice:
// ---------------------------------------------------
//    - This cog is governed under OmniCode Token Protocol.
//    - Any future changes must be formally recorded in `.scroll` docs.
//    - Instruction registry wiring must respect schema integrity.
//
// ---------------------------------------------------
// 📅 Last Known Version
// ---------------------------------------------------
//    - Version       : v0.0.1  
//    - Last Updated  : 2025-06-04  
//    - Change Log    : MVP tokenizer structure with docstreams + overcommented methods
//
// ---------------------------------------------------
// 🪧 Notes:
// ---------------------------------------------------
//    - This tokenizer is syntax-first: no semantic pass included.
//    - Group markers are non-enforced (parser validates scope).
//    - Token output is scroll-aligned and AI-aware for interpretation.
//
// ---------------------------------------------------
