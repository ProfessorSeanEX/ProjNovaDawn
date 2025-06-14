// ===============================================
// 📜 Metadata — Tokenizer v0.0.3 (Tablet Reader)
// ===============================================
// _author_:         Seanje Lenox-Wise / Nova Dawn
// _version_:        0.0.3
// _status_:         Dev
// _phase_:          Phase 3 — Post-Stub Validation (Scroll-Aware)
// _created_:        2025-06-04
// _last updated_:   2025-06-14
// _license_:        CreativeWorkzStudio LLC — Kingdom-First Proprietary Use
// _component_:      Tokenizer (Tablet Cog)
// _project_:        OmniCode / Millennium OS
// _description_:    Converts raw `.word`, `.omni`, and `.ns` scrolls into structured token streams for parsing and interpretation.
//
// _token schema_:   Symbolic Category Tokens, LineMeta Formatting, Group-Aware Tokens
// _validation hooks_: Group stack tracking, error token emission, EOF diagnostics
//
// _notes_:
// - Tokenizes source into scroll-structured `Token` variants
// - Tracks indentation, group markers, and comment metadata
// - Instruction registry integrated for keyword/instruction mapping
// - Errors emitted for malformed or unmatched tokens
// - Retains whitespace and comment fidelity for scroll parsing
// - Future support: `.logos` registry syncing, macro preprocessing, alignment-based filters
//
// ===============================================

// ===============================================
// 📖 Opening — Tokenizer Module Purpose & Design
// ===============================================
// This module converts NovaScript and OmniCode source streams
// into a structured scroll of `Token`s with positional awareness.
// It recognizes sacred syntax, scroll-level metadata, comments,
// literals, instructions, identifiers, and group structures.
//
// The Tokenizer sits upstream of the Parser and Operand Resolver,
// transforming raw input into the symbolic vocabulary of interpretation.
//
// It also generates `LineMeta` for formatting validation and
// is designed for scroll-awareness, debug trace integration,
// and macro-instruction expansion.
//
// Future expansion includes:
// - `.logos` registry syncing
// - Operand hint tagging for literal disambiguation
// - Integration with the Watchtower token stream for drift detection
//
// ===============================================
// 📦 Imports — Dependencies for Tokenizer Operation
// ===============================================
// These imports are grouped by origin and function:
// • Standard: character-level input processing and keyword maps
// • External: (None currently — reserved for macro or symbol maps)
// • Internal: Operand hint typing for forward compatibility
// • Debugging: (None yet — may expand for Watchtower token logs)

// === Standard Library ===
use std::collections::HashMap; // 🔑 Fast lookup for instruction keyword classification

// === Internal Modules ===
#[allow(unused_imports)]
use crate::operand_resolver::OperandHint; // 🧠 Future hook: tag tokens with operand meaning (e.g., Label, Register)

// ===============================================
// 📦 Foundational Declarations — Core Structures
// ===============================================

/// 🔤 `TokenType` — Canonical Classification of a Token
/// ----------------------------------------------------
/// Identifies the scroll-role of a token for use in:
/// • Parser construction
/// • Operand resolution (via Bearer)
/// • Debug stream traceability (via Watchtower)
///
/// Token types are divided into 4 semantic lanes:
/// 1. 📚 Structural Markers — Syntax boundaries and format
/// 2. 🔑 Symbol Semantics — Names, values, and opcodes
/// 3. 🗒 Line Modifiers — Metadata and developer comments
/// 4. ⚠ Fallback Catch — Invalid or malformed sequences
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    // === 📚 Structural Markers ===
    Whitespace,    // Not emitted; tracked in `LineMeta` for indentation/audit
    GroupMarker,   // Block and expression boundaries: `(` `)` `{` `}`
    Punctuation,   // Separators: `;` `,` `.` — affects list boundaries and syntax flow

    // === 🔑 Symbol Semantics ===
    Keyword,       // Reserved NovaScript keywords (e.g., `let`, `as`, `return`)
    Instruction,   // Registered operation or command (`walk`, `wait`, `invoke`)
    Identifier,    // User-defined variable, label, or function name
    Literal,       // Constant data: `"hello"`, `42`, `'a'`, etc.
    Operator,      // Arithmetic, comparison, logic: `+`, `-`, `==`, `<`, etc.

    // === 🗒 Line Modifiers ===
    Comment,       // Non-executable notes for developers: `//`, `#`
    Metadata,      // Executable scroll-level markers: `//!`, `#!`, etc.

    // === ⚠ Fallback Catch ===
    Error,         // Malformed or unknown tokens — routed to Watchtower
}

// ===============================================
// 📦 Token Structures — Token, LineMeta, TokenStream
// ===============================================

/// 🧱 Token — A Symbol in the Scroll
/// --------------------------------
/// Holds the type, value, and location of each token parsed.
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType, // Category of token behavior
    pub value: String,         // Source string matched
    pub line: usize,           // Line number in source (1-based)
    pub column: usize,         // Column offset (0-based)
}

/// 🧾 LineMeta — Per-Line Formatting & Indentation
/// -----------------------------------------------
/// Captures whitespace and structure context for every source line.
#[derive(Debug)]
pub struct LineMeta {
    pub line_number: usize, // Source line index (1-based)
    pub indentation: usize, // Leading whitespace count
    pub is_blank: bool,     // True if line is empty or contains only whitespace
}

/// 🔃 TokenStream — Output of Tokenization
/// --------------------------------------
/// A full result from the tokenizer, including all valid tokens,
/// formatting metadata, and unclassified errors for diagnostics.
#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,       // All valid tokens in scroll order
    pub line_meta: Vec<LineMeta>, // Per-line formatting context
    pub errors: Vec<Token>,       // Any malformed or rejected tokens
}

// ===============================================
// 🛠 Tokenizer Engine — Input Cursor & State Tracker
// ===============================================

/// ⚙️ Tokenizer — NovaScript & OmniCode Token Engine
/// ------------------------------------------------
/// Converts `.word`, `.ns`, or `.omni` source files into a scroll of structured `Token`s.
/// 
/// This engine walks through raw characters, tracking whitespace, structure, and syntax,
/// producing a `TokenStream` ready for scroll-based parsing and Watchtower inspection.
/// 
/// Responsibilities:
/// - Preserves spiritual formatting (indentation, blank lines)
/// - Maintains cursor metadata for error diagnostics
/// - Validates instruction-level classification via the registry
/// - Tracks group scope for nested blocks and flow markers
pub struct Tokenizer {
    // === 🔑 External Symbol Map ===
    pub instruction_registry: HashMap<String, TokenType>, // Classifies opcodes, schema-backed

    // === 🎯 Cursor State Tracking ===
    source: Vec<char>,     // Char-level walkable source
    position: usize,       // Current absolute cursor in `source`
    line: usize,           // Current line (1-based for reporting)
    column: usize,         // Current column (0-based offset)
    current_indent: usize, // Whitespace depth before active token

    // === 🧱 Structural Block Parsing ===
    group_stack: Vec<TokenType>, // Tracks open `{` / `(` until matched
}

// ===============================================
// 🧠 Tokenizer Body — Structure, Flow & Subroutines
// ===============================================
// This scroll defines the Tokenizer for NovaScript and OmniCode.
// It serves as the first interpreter of raw scroll text, transmuting
// source strings into meaningful tokens with positional and structural context.
//
// Every token produced flows downstream into the Parser and Bearer layers,
// and every indentation, grouping, or malformed symbol is tracked with care.
// The Tokenizer does not interpret meaning—but it sets the stage for it.
// It is the breath before the utterance, the cut before the shaping.

impl Tokenizer {
    // ===============================================
    // 🔨 Constructor — Tokenizer::new
    // ===============================================
    /// 🧬 Tokenizer::new — Initialize from Scroll Input
    /// ------------------------------------------------
    /// Constructs a new Tokenizer instance from:
    /// - `source_code`: the raw NovaScript scroll text
    /// - `instruction_map`: the instruction schema registry for keyword detection
    ///
    /// Initializes:
    /// - character vector (`source`) for precise char-by-char traversal
    /// - positional state (line, column, indent)
    /// - empty grouping stack for structural balance tracking
    ///
    /// This constructor does not emit tokens. It prepares the engine
    /// to begin its pass via `.tokenize()`, preserving scroll integrity.
    pub fn new(source_code: &str, instruction_map: HashMap<String, TokenType>) -> Self {
        Self {
            instruction_registry: instruction_map,             // 📚 Known keywords & instructions
            source: source_code.chars().collect(),             // 🔡 Raw scroll input → Vec<char>
            position: 0,                                       // 🧭 Cursor in source stream
            line: 1,                                           // 🔢 Starting at first line
            column: 0,                                         // 📍 Column tracker for position
            current_indent: 0,                                 // ↔️ Indentation tracking
            group_stack: vec![],                               // 📦 Stack for (, {, etc.
        }
    }

    // ===============================================
    // 🚧 Entry Point — Tokenizer::tokenize
    // ===============================================
    /// Converts raw input into a TokenStream:
    /// • Walks character-by-character
    /// • Emits tokens and formatting metadata
    /// • Collects early error tokens for diagnostics
    pub fn tokenize(&mut self) -> TokenStream {
        let mut tokens = vec![];      // All successfully parsed tokens
        let mut line_meta = vec![];   // Indentation and blank-line data
        let mut errors = vec![];      // Malformed or unknown token captures

        // 🔁 Main tokenizing loop — character-by-character
        while let Some(ch) = self.peek() {
            match ch {
                // --- Whitespace (not tokenized, but tracked) ---
                ' ' | '\t' => self.consume_whitespace(),

                // --- Newline (line break tracking only) ---
                '\n' => {
                    self.advance();    // Skip newline
                    self.line += 1;    // Next line
                    self.column = 0;   // Reset column
                }

                // --- Comments or Metadata (prefixed with `#`) ---
                '#' => tokens.push(self.tokenize_comment_or_meta()),

                // --- Literal: String (`"..."`) ---
                '"' => tokens.push(self.tokenize_string()),

                // --- Literal: Char (`'c'`) ---
                '\'' => tokens.push(self.tokenize_char()),

                // --- Operator Tokens ---
                ':' | '=' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '<' | '>' => {
                    tokens.push(self.tokenize_operator());
                }

                // --- Grouping Symbols ( ) ---
                '(' => {
                    self.group_stack.push(TokenType::GroupMarker);
                    tokens.push(self.make_token(TokenType::GroupMarker, "("));
                    self.advance();
                }
                ')' => {
                    self.group_stack.pop();
                    tokens.push(self.make_token(TokenType::GroupMarker, ")"));
                    self.advance();
                }

                // --- Alphabetic Word (could be identifier or instruction) ---
                c if c.is_alphabetic() => tokens.push(self.tokenize_word()),

                // --- Numeric Literal ---
                c if c.is_numeric() => tokens.push(self.tokenize_number()),

                // --- Unknown Symbol (fallback to Error token) ---
                _ => {
                    tokens.push(self.make_token(TokenType::Error, &ch.to_string()));
                    self.advance();
                }
            }
        }

        // ===============================================
        // 🧾 Line Formatting Metadata — Indentation Map
        // ===============================================
        // After token collection, analyze source lines for formatting metadata:
        // • Tracks indentation depth (leading whitespace count)
        // • Flags blank lines for structure alignment and spiritual whitespace
        let mut line_number = 1;

        for line in self.source.iter().collect::<String>().lines() {
            let indent = line.chars().take_while(|c| c.is_whitespace()).count();

            line_meta.push(LineMeta {
                line_number,
                indentation: indent,
                is_blank: line.trim().is_empty(),
            });

            line_number += 1;
        }

        // Emit the full TokenStream scroll:
        TokenStream {
            tokens,
            line_meta,
            errors,
        }
    }

    // ===============================================
    // 🔧 Cursor Subroutines — Navigation & Metadata
    // ===============================================
    // These methods control character-level traversal and source position state.
    // They are foundational for all tokenizer logic and metadata accuracy.

    // -----------------------------------------------
    // 🔁 advance — Move cursor forward by one char
    // -----------------------------------------------
    /// Increments the position and column; returns consumed char if any.
    fn advance(&mut self) -> Option<char> {
        let ch = self.source.get(self.position)?;
        self.position += 1;
        self.column += 1;
        Some(*ch)
    }

    // -----------------------------------------------
    // 👁 peek — Look ahead at current char (non-consuming)
    // -----------------------------------------------
    /// Allows tokenizer logic to branch without advancing.
    fn peek(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    // -----------------------------------------------
    // 🎯 make_token — Construct a Token from current position
    // -----------------------------------------------
    /// Wraps a token value and type with current line and column metadata.
    fn make_token(&self, token_type: TokenType, value: &str) -> Token {
        Token {
            token_type,
            value: value.to_string(),
            line: self.line,
            column: self.column,
        }
    }

    // -----------------------------------------------
    // 🔲 consume_whitespace — Skip spaces and tabs
    // -----------------------------------------------
    /// Advances past contiguous whitespace (not emitted as token).
    fn consume_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    // ===============================================
    // 💬 Comment & Metadata Tokenizers
    // ===============================================
    // Captures inline comments and metadata markers starting with `#` or `#!`.
    // These preserve author intent or system directives across the scroll.

    // -----------------------------------------------
    // 🧾 tokenize_comment_or_meta — Parse `#` or `#!`
    // -----------------------------------------------
    /// Distinguishes between developer comments and system metadata headers.
    /// - Metadata: begins with `#!` (scroll directives)
    /// - Comment: begins with `#` (human-facing notes)
    fn tokenize_comment_or_meta(&mut self) -> Token {
        let mut content = String::new();

        // 🔄 Accumulate content until newline or EOF
        while let Some(c) = self.peek() {
            if c == '\n' {
                break; // Stop on newline
            }
            content.push(c);   // Add char to comment
            self.advance();    // Move forward
        }

        // 🧭 Classify based on `#!` prefix (ignoring leading whitespace)
        if content.trim_start().starts_with("#!") {
            self.make_token(TokenType::Metadata, &content)
        } else {
            self.make_token(TokenType::Comment, &content)
        }
    }

    // ===============================================
    // 🔣 Tokenizers — Literal, Word, Number, Operator
    // ===============================================
    // These functions parse distinct token categories from character streams.
    // Each tokenizer emits a structured `Token` with positional metadata.
    // These form the building blocks of semantic parsing and operand resolution.

    // -----------------------------------------------
    // 🔡 String Literal — e.g., "hello\nworld"
    // -----------------------------------------------
    /// Parses a string literal `"..."` with basic escape handling.
    /// Supports the following escape codes:
    /// - `\n` → newline
    /// - `\t` → tab
    /// - `\\` → backslash
    /// - `\"` → double quote
    /// - `\'` → single quote
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
                        content.push(match escaped {
                            'n' => '\n',
                            't' => '\t',
                            '\\' => '\\',
                            '"' => '"',
                            '\'' => '\'',
                            _ => escaped, // Unknown escape, pass through
                        });
                        self.advance();
                    }
                }
                _ => {
                    content.push(c);
                    self.advance();
                }
            }
        }

        self.make_token(TokenType::Literal, &content)
    }

    // -----------------------------------------------
    // 🔠 Character Literal — e.g., 'a', '\n'
    // -----------------------------------------------
    /// Parses a single-character literal surrounded by `'`.
    /// Future-proofed to support simple escape sequences.
    /// Malformed literals fallback to the Unicode replacement char `�`.
    fn tokenize_char(&mut self) -> Token {
        self.advance(); // Consume opening `'`
        let value = match self.peek() {
            Some('\\') => {
                self.advance(); // consume `\`
                match self.peek() {
                    Some('n') => { self.advance(); '\n' },
                    Some('t') => { self.advance(); '\t' },
                    Some('\\') => { self.advance(); '\\' },
                    Some('\'') => { self.advance(); '\'' },
                    Some(c) => { self.advance(); c },
                    None => '�',
                }
            }
            Some(c) => {
                self.advance();
                c
            }
            None => '�',
        };
        self.advance(); // Consume closing `'` or next char regardless

        self.make_token(TokenType::Literal, &value.to_string())
    }

    // -----------------------------------------------
    // ➕ Operator Sequence — e.g., ==, +=, >>
    // -----------------------------------------------
    /// Parses one or more compound operators like `==`, `!=`, `+=`.
    fn tokenize_operator(&mut self) -> Token {
        let mut content = String::new();
        while let Some(c) = self.peek() {
            if ":=+-*/%&|<>".contains(c) {
                content.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.make_token(TokenType::Operator, &content)
    }

    // -----------------------------------------------
    // 🔢 Numeric Literal — e.g., 42
    // -----------------------------------------------
    /// Parses decimal integer literals.
    /// Extended formats (hex, float) will be supported in future revisions.
    fn tokenize_number(&mut self) -> Token {
        let mut num = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num.push(c);
                self.advance();
            } else {
                break;
            }
        }
        self.make_token(TokenType::Literal, &num)
    }

    // -----------------------------------------------
    // 📛 Word — Instruction or Identifier
    // -----------------------------------------------
    /// Parses a keyword, instruction, or user-defined identifier.
    /// If found in the registry, it's marked as an `Instruction`.
    fn tokenize_word(&mut self) -> Token {
        let mut word = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                word.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let token_type = if self.instruction_registry.contains_key(&word) {
            TokenType::Instruction
        } else {
            TokenType::Identifier
        };

        self.make_token(token_type, &word)
    }

    // ===============================================
    // 🧩 Hooks — Validation, Grouping, and Preprocessing
    // ===============================================
    // These are post-tokenization hooks designed to validate and prepare token streams
    // before they enter the parser or operand resolver.
    // Future expansion will include:
    // • Operand schema enforcement
    // • Token grouping verification
    // • Scroll-aware macro or directive expansion

    // ------------------------------------------------
    // 📏 Instruction Validator — Arity & Operand Matching
    // ------------------------------------------------
    /// Validates instruction format by checking operand count against schema.
    ///
    /// This early implementation assumes instructions are defined as:
    /// - key: instruction name
    /// - value: token type (e.g., Instruction)
    ///
    /// Future schemas will extend this to:
    /// - enforce operand types
    /// - permit optional / variadic args
    /// - match formatting rules
    fn validate_instruction_syntax(keyword: &str, operands: &[&str]) -> bool {
        // Early schema stub: expected arity map (to be moved into instruction_registry in future)
        let expected_operands: HashMap<&str, usize> = HashMap::from([
            ("let", 2),
            ("set", 2),
            ("walk", 1),
            ("speak", 1),
            ("return", 1),
            ("goto", 1),
        ]);

        if let Some(&expected) = expected_operands.get(keyword) {
            operands.len() == expected
        } else {
            true // Accept unknown instructions for now
        }
    }

    // ------------------------------------------------
    // 🌀 Token Post-Processor — Group Integrity Check & Placeholder Transform
    // ------------------------------------------------
    /// Prepares token stream for scroll parsing:
    /// - Verifies grouping marker balance
    /// - Flags unclosed or orphaned brackets
    /// - Preps structure for AST nesting (future)
    fn post_process_tokens(mut tokens: Vec<Token>) -> Vec<Token> {
        let mut group_stack: Vec<(TokenType, Token)> = Vec::new();

        for token in &tokens {
            match token.token_type {
                TokenType::GroupMarker => {
                    match token.value.as_str() {
                        "(" | "{" => group_stack.push((TokenType::GroupMarker, token.clone())),
                        ")" | "}" => {
                            if group_stack.pop().is_none() {
                                // Insert virtual open if we pop nothing
                                // This could also push an Error token instead in future
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if !group_stack.is_empty() {
            for (_, token) in group_stack {
                // In future: add diagnostic or append unclosed group marker to errors
                // tokens.push(Token { ...error for unclosed group... })
            }
        }

        tokens
    }
}

// ===============================================
// 🧱 Token Builder — Operand & Parser-Compatible Shortcuts
// ===============================================
// Constructs lightweight tokens outside of full tokenizer context.
// Used by the operand resolver, parser fallback routines, or internal injections.

impl Token {
    /// 🔧 from_value — Minimal Token Constructor
    /// ----------------------------------------
    /// Creates a token from a string value with default type `Identifier`.
    /// Line and column are set to `0`, as this is not tied to tokenizer state.
    ///
    /// 🔹 Used in:
    /// • OperandResolver — to build placeholder tokens for operand slots
    /// • Parser — when inserting system-defined identifiers (e.g., implicit labels)
    /// • Testing — when mocking token sequences without a full source file
    ///
    /// 🧠 Note: Avoid using in live tokenizer output — lacks position accuracy.
    pub fn from_value(value: &str) -> Self {
        Token {
            token_type: TokenType::Identifier, // May be reclassified by resolver
            value: value.to_string(),          // Raw symbolic name
            line: 0,                            // Default, parser may overwrite
            column: 0,                          // Default, parser may overwrite
        }
    }
}

// ===================================================
// 🔚 Closing Block — Tokenizer Output & Expansion Path
// ===================================================
//
// 🧾 Overview:
//   - This module converts raw `.word`, `.omni`, and `.ns` scrolls
//     into structured token streams for parsing, interpretation, and tracking.
//   - It captures indentation, groups, and symbolic metadata while preserving scroll fidelity.
//
// ⚙️ Engine Scope:
//   - Emits tokens with line and column tracking
//   - Tracks grouping structures (paren pairs) for validation phases
//   - Produces diagnostic-friendly `TokenStream` for parser consumption
//
// ---------------------------------------------------
// 🚨 Version Control Notice:
// ---------------------------------------------------
//   This logic is governed under the OmniCode Token Protocol.
//   All changes must be recorded in project `.scroll` logs.
//   ⚠️ Schema-integrity must be preserved during tokenizer refactors.
//
// ---------------------------------------------------
// 📅 Scroll Revision Metadata:
// ---------------------------------------------------
//   _version_:       v0.0.3  
//   _last updated_:  2025-06-14  
//   _author_:        Seanje Lenox-Wise / Nova Dawn  
//   _change log_:
//     - Refined output stream structure and group marker tracking
//     - Upgraded inline comments and cursor accuracy
//     - Prepared `TokenStream` for post-parse operand resolution
//
// ---------------------------------------------------
// 🪜 Ladder Baton — Flow & Interface Direction:
// ---------------------------------------------------
//   ⬆️ Upstream:
//     - Receives raw string input from CLI, GUI, or system file hooks
//     - Integrates `.logos` keyword registry (stubbed)
//
//   ⬇️ Downstream:
//     - Passes `TokenStream` into Parser for ScrollTree construction
//     - Exports errors and token groups to Watchtower (future)
//
//   🔁 Parallel:
//     - Will sync with Macro Preprocessor for inline instruction folding
//     - Interfaces with ScrollRegistry to reflect token-class mappings
//
// ---------------------------------------------------
// 🔮 Notes for Next Phase:
// ---------------------------------------------------
// - Enforce group validation logic and closed-paren errors
// - Add escape-sequence depth to string and char tokenizers
// - Wire `.logos` syncing and type-scoped keyword resolution
// - Begin symbol tagging for future grammar scoring in parser
//
// ---------------------------------------------------

// ===============================================
// 🔒 Closing — Final Diagnostics & Stack Cleanup
// ===============================================

// 🔹 Group Marker Check — Unmatched open parens/braces
while let Some(unmatched) = self.group_stack.pop() {
    errors.push(Token {
        token_type: TokenType::Error,
        value: format!("Unclosed group marker: {:?}", unmatched),
        line: self.line,
        column: self.column,
    });
}

// 🔹 End-of-File Token Hooks (optional)
// Could emit EOF token or special scroll-seal marker later
