// ===============================================
// 📜 Metadata - Parser v0.0.1 (Tablet Priest)
// ===============================================
// _author_:        Seanje Lenox-Wise / Nova Dawn
// _version_:       0.0.1
// _status_:        Dev
// _created_:       2025-06-04
// _last updated_:  2025-06-04
// _license_:       CreativeWorkzStudio LLC — Kingdom-First Proprietary Use
// _component_:     Parser (Tablet Cog)
// _project_:       OmniCode / Millennium OS
// _description_:   Converts token streams into Scroll Trees (OmniCode ASTs) using sentence-based grammar rules.
//
// _notes_:
// - Parses tokenized input into executable logical nodes
// - Supports sentence-structure and scroll-style node types
// - Future support: grammar inference, instruction decoding hooks, error correction
// - Core link between tokenizer and compiler backend
// ===============================================

// ===============================================
// 🌀 Opening — Imports & Declarations
// ===============================================
// This section declares all dependencies used in the Parser module.
// It includes standard libraries, timestamping, tokenizer input, instruction metadata,
// and debugging scaffolding required for scroll parsing and sentence validation.

// === Standard Library Imports ===

#[allow(unused_imports)]
use chrono::Utc;
use std::collections::VecDeque; // 🔁 Used as a token queue for recursive descent parsing—ensures ordered traversal

#[allow(unused_imports)]
use std::fmt; // 🧾 Enables custom Display/Debug formatting for AST or ScrollTree output // 🕰 Timestamps each parse event for metadata anchoring, debug traceability

// === Internal Module Imports ===

use crate::tokenizer::{Token, TokenType};
// 🧱 Core units of NovaScript: token value, type classification, and source location (line, column)

use crate::instruction_registry::get_instruction_registry;
// 📚 Registry of valid instructions—used to validate opcodes, operand schemas, and spiritual posture

#[allow(unused_imports)]
use crate::debugger::{
    DebugEntry, // 📋 Snapshot of a single parse attempt—contains source, line, message, severity
    DebugResponse, // 🔧 Represents corrective or confirmational feedback for system or AI agent
    Severity,   // 🌡 Enum to classify alignment breaches: Fatal, Drifted, Valid, etc.
}; // 🪛 The Watchtower speaks through these: Parser reports all sentence validation here

// ===============================================
// 📦 Foundational Declarations — Core Structures
// ===============================================
// These declarations form the base architecture of the Scroll Parser.
// No execution logic resides here—only core structures that define
// how sentences are captured, represented, and prepared for interpretation.
//
// This section includes:
// • `ScrollNode`: The building blocks of parsed sentence meaning
// • `ScrollTree`: A structured container for scroll-level node sets
// • Parser structs (`ScrollParser`, `Parser`): Responsible for walking tokens and forming node chains

/// 🧩 Enum representing all valid node types produced by the parser.
/// These are the elemental scroll structures—each one representing a distinct sentence form,
/// value expression, or system directive.
#[derive(Debug, Clone)]
pub enum ScrollNode {
    Instruction {
        name: String,
        args: Vec<String>,
    },
    // 🪶 A named instruction with one or more arguments (e.g., invoke("light.fire"))
    ScrollSentence {
        subject: String,
        verb: String,
        object: String,
    },
    // 🧾 A full NovaScript sentence with structure (e.g., Let x be set to 6)
    Assignment {
        target: String,
        value: String,
    },
    // 📦 Variable binding or mutation (e.g., holiness = 100)
    Literal(String),
    // ✍️ A raw or primitive value (string, number, boolean, etc.)
    Metadata(String),
    // 📘 System or scroll metadata, often marked by special comment notation (e.g., // author)
    Block(Vec<ScrollNode>),
    // 🧱 A grouped sequence of child nodes (e.g., loop body, function scope)
    Error(String),
    // ❌ A fallback node when parsing fails—contains diagnostic message

    // ⚙️ Optional & emerging structures — extensible architecture
    Declaration {
        name: String,
        dtype: Option<String>,
    },
    // ✒️ Variable or symbol declaration with optional type (e.g., let x: int)
    Conditional {
        condition: String,
        body: Vec<ScrollNode>,
    },
    // 🧭 Conditional block structure (e.g., if/else with internal nodes)
    Loop {
        condition: String,
        body: Vec<ScrollNode>,
    },
    // 🔁 Loop block structure (e.g., while condition { ... })
    Import(String),
    // 📥 File or scroll import directive
    Return(String),
    // 🔚 Return value from within function or block
    Call {
        function: String,
        args: Vec<String>,
    },
    // 📞 Function call or pipeline invocation (used in nested expressions)
    Comment(String),
    // 💬 Non-executing annotation or note (inline or overcomment)
}

/// 📚 The full parsed result of a NovaScript scroll.
/// Acts as an AST-like container and provides a complete, ordered structure
/// of what the system can interpret, compile, or review.
pub struct ScrollTree {
    pub nodes: Vec<ScrollNode>,
    // 🔗 All top-level nodes in the scroll—order matters
}

/// 🌀 Legacy parser implementation.
/// Retained for phased migration and test coverage.
/// Provides simple parsing loop over token stream to node conversion.
pub struct ScrollParser {
    tokens: VecDeque<Token>,
    // 🪙 Token queue for ordered consumption in legacy mode
    scroll: Vec<ScrollNode>,
    // 🧾 Accumulated scroll under construction (pre-tree finalization)
}

/// 🎯 The primary parser implementation.
/// Responsible for sentence interpretation, node construction, and scroll validation.
/// Uses a flat token stream with explicit position tracking.
pub struct Parser {
    tokens: Vec<Token>,
    // 📜 Linear token list derived from the tokenizer
    position: usize,
    // 🔍 Current position within token stream (cursor for descent)
}

// ===============================================
// 🛠 Constructors & Initializers
// ===============================================
// Responsible for preparing parser structures before interpretation begins.
// These methods instantiate stateful containers for token walking, node construction,
// and scroll preparation. No parsing logic occurs here—only structure creation.

impl ScrollParser {
    /// 🧱 Constructs a new instance of the legacy `ScrollParser`.
    /// This structure uses a queued token stream (VecDeque) and a pre-allocated scroll buffer.
    /// ⚠️ This is a transitional parser and will be deprecated in favor of `Parser`.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: VecDeque::from(tokens), // 🔁 Initializes token stream with queue ordering
            scroll: Vec::new(),             // 🧾 Prepares an empty scroll for node construction
        }
    }

    /// 🧠 Legacy placeholder — does not perform parsing.
    /// Returns the unmodified scroll structure for compatibility.
    pub fn parse(self) -> Vec<ScrollNode> {
        self.scroll // 🚧 Acts as a stub method until logic migration is complete
    }
}

impl Parser {
    /// 🎬 Constructs a new `Parser` from a linear token stream.
    /// Sets internal cursor to the starting position (0).
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,      // 📜 Token list sourced from tokenizer
            position: 0, // 🧭 Begin at the first token in the stream
        }
    }
}

// ===============================================
// ⚠️ ParseError System for OmniCode
// ===============================================
// This module defines all parser-side error handling used
// to catch misalignment, invalid syntax, and scroll violations.
// Includes error typing, message composition, and trait wiring
// for ergonomic propagation using `?`.
//
// Each error is a scroll breach, requiring insight or repentance.

/// 🧯 Enum representing categories of parser failure.
/// Each variant defines a unique class of misalignment between scroll syntax and expected sentence logic.
#[derive(Debug)]
pub enum ParseErrorType {
    UnexpectedEOF,
    InvalidArgument(String),
    UnexpectedToken,
    MissingToken,
    InvalidInstruction,
    InvalidGrammar,
    UnknownSymbol,
}

/// 🩺 Represents a single error encountered while parsing a scroll.
/// Contains type, readable message, and positional metadata.
#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorType, // 🧭 Classification of the issue
    pub message: String,      // 📝 Explanation of what went wrong
    pub line: usize,          // 📍 Line number in the scroll
    pub column: usize,        // 📏 Character offset in the line
}

impl ParseError {
    /// 🔧 Create a new parse error with full detail
    pub fn new(
        kind: ParseErrorType,
        message: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
            line,
            column,
        }
    }

    /// 📃 Lightweight builder for structural errors without location
    pub fn basic(kind: ParseErrorType) -> Self {
        Self {
            message: format!("Parser failed due to: {:?}", kind),
            kind,
            line: 0,
            column: 0,
        }
    }
}

/// 💡 Enables use of `?` on `ParseErrorType` inside parser Result functions
impl From<ParseErrorType> for ParseError {
    fn from(kind: ParseErrorType) -> Self {
        ParseError::basic(kind)
    }
}

/// 🧾 Optional: Pretty formatter for logging or dev tools
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Line {}, Col {}] {:?}: {}",
            self.line, self.column, self.kind, self.message
        )
    }
}

// ===============================================
// 🧠 Body Block — Parsing Logic & Node Walkers
// ===============================================
// This section defines the Parser’s core behavior: walking through
// the token stream, interpreting sentence structure, and constructing
// `ScrollNode`s that represent fully parsed instructions or declarations.
//
// Walkers act like discerners — interpreting scrolls line by line,
// transforming raw syntax into meaningful structure.

impl Parser {
    /// 🧠 Master parsing loop — constructs a complete `ScrollTree`.
    ///
    /// Parses token-by-token, generating sentence-shaped nodes
    /// until the scroll is fully interpreted.
    ///
    /// 🔁 Logic:
    /// • Walks token stream to exhaustion
    /// • Delegates parsing to `parse_node()` for each top-level line
    /// • Skips malformed or invalid tokens gracefully
    ///
    /// 📜 Output:
    /// A `ScrollTree` containing all top-level sentence nodes.
    pub fn parse(&mut self) -> ScrollTree {
        let mut nodes = vec![];

        // 🔁 Loop until all tokens have been read
        while self.peek().is_some() {
            // ✏️ Attempt to parse next scroll sentence
            if let Some(node) = self.parse_node() {
                nodes.push(node); // ✅ If valid, add to scroll
            }
        }

        // 🌳 Return structured tree of interpreted sentences
        ScrollTree { nodes }
    }

    /// 🔍 Node dispatcher — determines how to interpret each token.
    ///
    /// Examines the current token and routes it to the correct parsing function
    /// based on its token type and value. Acts as a scroll sentence router.
    ///
    /// 🧩 Token Routing:
    /// • `Instruction` → `parse_instruction()`  (e.g., `invoke("flame")`)
    /// • `Literal`     → `parse_literal()`      (e.g., `"Holy Fire"`)
    /// • `Identifier`  → `parse_assignment_or_call()` (e.g., `x = 3`)
    /// • `Metadata`    → `parse_metadata()`     (e.g., `// system info`)
    /// • `Comment`     → `parse_comment()`      (e.g., `# speak only truth`)
    /// • `GroupMarker` → `parse_block()`        (e.g., `{ let x = 5 }`)
    ///
    /// ❗ Any unknown or invalid token yields a `ScrollNode::Error`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_node(&mut self) -> Option<ScrollNode> {
        let token = self.peek()?; // 👁 Preview current token without consuming it

        match token.token_type {
            TokenType::Instruction => self.parse_instruction(), // ⚙️ Scroll instruction
            TokenType::Literal => self.parse_literal(),         // 🔢 Raw literal value
            TokenType::Identifier => self.parse_assignment_or_call(), // 🪶 Variable or call logic
            TokenType::Metadata => self.parse_metadata(),       // 📘 Metadata directives
            TokenType::Comment => self.parse_comment(),         // 💬 Human-facing notes

            // 🧱 Start of scroll block (e.g., loop, function body)
            TokenType::GroupMarker if token.value == "{" => self.parse_block(),

            _ => {
                // 🚨 Token does not match known sentence starters
                self.advance(); // Avoid infinite loop on invalid token
                Some(ScrollNode::Error("Unrecognized token".into())) // ❌ Sentence rejected
            }
        }
    }

    // ===============================================
    // 🧩 Token Walker & Dispatch Layer — Core Interpreters
    // ===============================================
    //
    // These functions convert individual tokens into `ScrollNode`s.
    // Each walker embodies a unique grammatical route in NovaScript.
    // Cursor utilities like `advance` and `peek` allow precise control
    // during recursive descent, enabling sentence-by-sentence discernment.

    /// 📌 Advance the token stream — move cursor forward and consume token.
    ///
    /// Retrieves and returns the token at the current position, then
    /// advances the parser’s cursor. Returns `None` if end of stream is reached.
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn advance(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.position).cloned(); // 🧤 Clone for safety — tokens are immutable
        if tok.is_some() {
            self.position += 1; // ➡️ Shift parser focus forward
        }
        tok
    }

    /// 🔍 Peek at the current token without consuming it.
    ///
    /// Returns a reference to the token at the parser’s current position.
    /// This allows routing decisions without modifying cursor state.
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.position) // 🔭 Look ahead for interpretation without movement
    }

    /// ⚙️ Instruction walker — parses an opcode-like token into `ScrollNode::Instruction`.
    ///
    /// - Consumes the instruction keyword (e.g., `invoke`)
    /// - Collects all following tokens that qualify as arguments (identifiers, literals, operators)
    /// - Stops parsing on invalid types, block openers, or newline boundaries
    ///
    /// 🧭 Example Input:
    /// - Token stream: `invoke "truth" +5`
    /// - Output: `ScrollNode::Instruction { name: "invoke", args: ["truth", "+5"] }`
    ///
    /// 🔧 Debug mode (when enabled):
    /// - Logs instruction parse event with name and argument count
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_instruction(&mut self) -> Option<ScrollNode> {
        let token = self.advance()?; // 🎯 Consume the instruction keyword

        if self.decode_instruction(&token).is_none() {
            return Some(ScrollNode::Error(format!(
                "Unknown instruction '{}'",
                token.value
            )));
        }

        let mut args = Vec::new();

        // 🔁 Gather tokens as arguments while valid
        while let Some(tok) = self.peek() {
            match tok.token_type {
                TokenType::Literal | TokenType::Identifier | TokenType::Operator => {
                    args.push(tok.value.clone()); // 🧾 Push token value into arg list
                    self.advance(); // ➡️ Move to next token
                }
                TokenType::Whitespace => {
                    self.advance(); // 🧹 Skip spacers silently
                }
                _ => break, // ⛔ Stop parsing args at invalid boundary
            }
        }

        // 🧪 Debug logging — shows result and argument set
        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, DebugResponse, Severity};

            let entry = DebugEntry::new(
                "parse_instruction",
                &token.value,
                "Instruction ScrollNode",
                format!("Instruction ScrollNode with {} args", args.len()).as_str(),
            )
            .with_location("Parser::parse_instruction")
            .with_suggestion("Ensure argument types align with instruction schema.");

            println!("{:#?}", entry); // 🪵 Emit structured debug entry
        }

        // 🧱 Construct final instruction node
        Some(ScrollNode::Instruction {
            name: token.value,
            args,
        })
    }

    /// ✍️ Parses a literal token into `ScrollNode::Literal`.
    ///
    /// Captures basic primitives such as strings, numbers, booleans, and symbols.
    /// Does not attempt type coercion or expression evaluation—this occurs later.
    ///
    /// 🧭 Example:
    /// - Input: `"truth"` → Output: `ScrollNode::Literal("truth")`
    ///
    /// 🔧 Debug mode:
    /// - Logs captured literal and confirms parse success
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_literal(&mut self) -> Option<ScrollNode> {
        let token = self.advance()?; // 📥 Retrieve and consume literal token

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, DebugResponse, Severity};
            let entry = DebugEntry::new(
                "parse_literal",
                &token.value,
                "Literal ScrollNode",
                "Literal ScrollNode",
            )
            .with_location("Parser::parse_literal");
            println!("{entry:#?}"); // 📊 Log successful interpretation
        }

        Some(ScrollNode::Literal(token.value)) // ✅ Return valid node
    }

    /// 🧭 Assignment/Call Branch Walker — Resolves ambiguity on identifiers.
    ///
    /// Determines whether the identifier begins:
    /// - An assignment (e.g., `path = "truth"`)
    /// - A function or command call (e.g., `proclaim("glory")`)
    ///
    /// Walks one token ahead to route behavior.
    ///
    /// 🧠 Fallback behavior:
    /// - If next token is not `=` or `(`, logs `Error` node
    ///
    /// 🔧 Debug mode:
    /// - Logs identifier, expected branching pattern, and actual next token
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_assignment_or_call(&mut self) -> Option<ScrollNode> {
        let identifier = self.advance()?; // 🔑 Consume variable or function name
        let next = self.peek()?; // 👁️ Inspect next token to resolve grammar type

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, DebugResponse, Severity};
            let expected = "`=` or `(`";
            let actual = next.value.clone();
            let entry = DebugEntry::new(
                "parse_assignment_or_call",
                &identifier.value,
                expected,
                &actual,
            )
            .with_location("Parser::parse_assignment_or_call")
            .with_suggestion("Check next token to distinguish assignment or call.");
            println!("{entry:#?}");
        }

        match next.value.as_str() {
            "=" => {
                self.advance(); // 🪜 Skip `=`
                let value_token = self.advance()?; // 📥 Capture right-hand side
                Some(ScrollNode::Assignment {
                    target: identifier.value,
                    value: value_token.value,
                })
            }
            "(" => {
                self.parse_call(identifier.value.clone()) // 📞 Hand off to function call walker
            }
            _ => {
                // ❗ Unexpected pattern — raise error node for ambiguity
                Some(ScrollNode::Error(format!(
                    "Ambiguous identifier usage near '{}'",
                    identifier.value
                )))
            }
        }
    }

    /// 📘 Metadata Interpreter — parses scroll-level directives.
    ///
    /// Captures special comments used to describe the scroll’s purpose,
    /// subsystem context, or execution framing.
    ///
    /// Recognizes lines starting with:
    /// - `//`, `##!`, `///`, etc.
    ///
    /// These lines are **not executed**, but hold **contextual authority** for scroll alignment.
    ///
    /// 🧭 Example:
    /// - `// this scroll governs the NovaGate` → `ScrollNode::Metadata(...)`
    ///
    /// 🔧 Debug mode:
    /// - Logs captured metadata and its parsing context
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_metadata(&mut self) -> Option<ScrollNode> {
        let token = self.advance()?; // 🧾 Consume metadata token from token stream

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, DebugResponse, Severity};
            let entry = DebugEntry::new(
                "parse_metadata",
                &token.value,
                "Metadata ScrollNode",
                "Metadata ScrollNode",
            )
            .with_location("Parser::parse_metadata");
            println!("{entry:#?}"); // 🪵 Log metadata parsing
        }

        Some(ScrollNode::Metadata(token.value)) // 🧱 Emit metadata node
    }

    /// 💬 Comment Interpreter — parses human-facing notes.
    ///
    /// Captures developer commentary or spiritual reminders
    /// embedded within the scroll. These are **never executed**
    /// but are preserved to maintain voice, clarity, and design memory.
    ///
    /// 🧭 Example:
    /// - `# This section controls the gate logic` → `ScrollNode::Comment(...)`
    ///
    /// 🔧 Debug mode:
    /// - Logs parsing of comment token and associated content
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_comment(&mut self) -> Option<ScrollNode> {
        let token = self.advance()?; // ✏️ Pull comment token from token stream

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, DebugResponse, Severity};
            let entry = DebugEntry::new(
                "parse_comment",
                &token.value,
                "Comment ScrollNode",
                "Comment ScrollNode",
            )
            .with_location("Parser::parse_comment");
            println!("{entry:#?}"); // 🗒️ Print comment for audit
        }

        Some(ScrollNode::Comment(token.value)) // 🧱 Emit comment node
    }

    // ===============================================
    // 🧭 Grammar Walkers — Expression & Structure Parsers
    // ===============================================

    /// 🧠 Condition Extractor — builds conditional expressions.
    ///
    /// Walks forward through the token stream to extract conditions
    /// used in `if`, `when`, `while`, and similar constructs.
    ///
    /// The walk stops when:
    /// - A block delimiter `{` is found
    /// - A statement terminator `;` is encountered
    ///
    /// 🧭 Example:
    /// `if x > 5 {` → will extract `x > 5`
    ///
    /// 🔧 Debug mode:
    /// - Logs the full condition string and hints at structural expectation
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn walk_condition(&mut self) -> Option<String> {
        let mut condition = String::new(); // 🧱 Accumulator for token values

        while let Some(token) = self.peek() {
            match token.value.as_str() {
                "{" | ";" => break, // 🧱 Stop at structure break
                _ => {
                    let t = self.advance()?; // 🎯 Consume valid token
                    if !condition.is_empty() {
                        condition.push(' '); // 🔗 Preserve token separation
                    }
                    condition.push_str(&t.value); // 📎 Append token to condition
                }
            }
        }

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "walk_condition",
                &condition,
                "Condition expression",
                "Condition parsed from tokens",
            )
            .with_location("Parser::walk_condition")
            .with_suggestion("Ensure block follows valid grammar");
            println!("{entry:#?}"); // 🪵 Emit trace log
        }

        if condition.is_empty() {
            None // 🚫 No usable condition
        } else {
            Some(condition) // ✅ Return extracted expression
        }
    }

    /// 🧾 Type Annotation Parser — extracts inline type hints.
    ///
    /// Recognizes optional type signatures in variable declarations.
    /// Walks the pattern: `:` → `TypeName`
    ///
    /// 🧭 Example:
    /// `let x: Int` → extracts `"Int"`
    ///
    /// Returns `None` if `:` is not present.
    ///
    /// 🔧 Currently does not validate type name itself—reserved for type checker layer.
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn walk_type_annotation(&mut self) -> Option<String> {
        let colon = self.peek()?; // 👁️ Look ahead for type indicator
        if colon.value != ":" {
            return None; // 🚫 No type hint present
        }

        self.advance()?; // ✅ Consume `:`
        let next = self.advance()?; // 🔤 Get type name token

        Some(next.value.clone()) // 📦 Return extracted type name
    }

    /// 🪶 Parses a comma-separated argument list enclosed in `(...)`.
    ///
    /// Used in function and instruction calls such as:
    /// `invoke(reveal, glory)` → args = ["reveal", "glory"]
    ///
    /// Returns:
    /// - `Vec<String>` of raw argument tokens
    /// - Will return empty if no `(` is detected
    ///
    /// 🧭 Walk Logic:
    /// - Starts after seeing `(`
    /// - Accepts identifiers, literals, and raw tokens
    /// - Skips commas, stops at `)`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_argument_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut args = vec![];

        // 🔍 Ensure argument block starts with `(`
        let peeked = self.peek().ok_or(ParseErrorType::UnexpectedEOF)?;
        if peeked.value != "(" {
            return Ok(args); // ✅ match return type
        }
        self.advance(); // ✅ Consume opening parenthesis

        while let Some(token) = self.peek() {
            match token.value.as_str() {
                ")" => {
                    self.advance(); // ✅ End of arguments
                    break;
                }
                "," => {
                    self.advance(); // 🧹 Clean comma
                    continue;
                }
                _ => {
                    let arg_token = self.advance().ok_or(ParseErrorType::UnexpectedEOF)?;
                    // 🎯 Grab argument
                    args.push(arg_token.value.clone()); // 📦 Store argument
                }
            }
        }

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let joined = args.join(", ");
            let entry = DebugEntry::new(
                "parse_argument_list",
                &joined,
                "Function args",
                "Collected from () group",
            )
            .with_location("Parser::parse_argument_list")
            .with_suggestion("Validate argument arity if required");
            println!("{entry:#?}"); // 🪵 Emit log
        }

        Ok(args)
    }

    /// 📜 Parses a Scroll Sentence in Subject-Verb-Object form.
    ///
    /// Pattern:
    /// - `subject verb object` → becomes `ScrollNode::ScrollSentence`
    ///
    /// Assumes three consecutive tokens with clear semantic weight.
    /// Example:
    /// - `The priest speaks truth` → subject = "The priest", verb = "speaks", object = "truth"
    ///
    /// 🔎 Does not currently validate grammar or perform plural/singular agreement checks.
    /// Suitable for embedded natural language execution or proto-schema walking.
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_scroll_sentence(&mut self) -> Option<ScrollNode> {
        let subject = self.advance()?.value; // 🙋 Who is acting
        let verb = self.advance()?.value; // 🗣️ What they do
        let object = self.advance()?.value; // 🎯 What is acted upon

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let phrase = format!("{subject} {verb} {object}");
            let entry = DebugEntry::new(
                "parse_scroll_sentence",
                &phrase,
                "Subject Verb Object",
                "Parsed SVO triple",
            )
            .with_location("Parser::parse_scroll_sentence")
            .with_suggestion("Validate grammar structure with schema");
            println!("{entry:#?}");
        }

        Some(ScrollNode::ScrollSentence {
            subject,
            verb,
            object,
        })
    }

    /// ===============================================
    /// 🧩 Optional & Advanced Node Handlers (Wired Stubs)
    /// ===============================================

    /// 📐 Parses a typed variable declaration into a `ScrollNode::Declaration`.
    ///
    /// Pattern:
    /// - `let name: Type` → parsed into named binding with optional type annotation
    ///
    /// Flow:
    /// - Consumes `let` keyword
    /// - Grabs the identifier
    /// - Optionally parses a `: Type` suffix
    ///
    /// Returns:
    /// - `ScrollNode::Declaration { name, dtype }`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_declaration(&mut self) -> Option<ScrollNode> {
        let _keyword = self.advance()?; // Expect `let`
        let name_token = self.advance()?; // Capture variable name

        let dtype = self.walk_type_annotation(); // Parse optional `: Type`

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let display = format!(
                "{}{}",
                name_token.value,
                dtype.as_ref().map(|d| format!(": {d}")).unwrap_or_default()
            );

            let entry = DebugEntry::new(
                "parse_declaration",
                &display,
                "let name: Type",
                "Parsed declaration structure",
            )
            .with_location("Parser::parse_declaration")
            .with_suggestion("Ensure name is a valid identifier and type is registered");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Declaration {
            name: name_token.value,
            dtype,
        })
    }

    /// 🔀 Parses a conditional block like `if condition { ... }`
    ///
    /// Handles:
    /// - Condition expressions (`walk_condition`)
    /// - Body blocks (`parse_block`)
    ///
    /// Example:
    /// ```plaintext
    /// if faith > fear {
    ///     proclaim("Victory")
    /// }
    /// ```
    ///
    /// Returns:
    /// - `ScrollNode::Conditional { condition, body }`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_conditional(&mut self) -> Option<ScrollNode> {
        let _keyword = self.advance()?; // Expect `if` or similar keyword
        let condition = self.walk_condition()?; // Parse inline expression
        let body = self.parse_block(); // Parse following block as body

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "parse_conditional",
                &condition,
                "if <condition> { block }",
                "Parsed if-statement",
            )
            .with_location("Parser::parse_conditional")
            .with_suggestion("Ensure condition is valid and block is non-empty");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Conditional {
            condition,
            body: vec![body.unwrap()],
        })
    }

    /// 🔁 Parses a loop construct into `ScrollNode::Loop`.
    ///
    /// Supported Pattern:
    /// - `while <condition> { ... }`
    ///
    /// Logic:
    /// - Consumes the loop keyword (`while`, `for`, etc.)
    /// - Extracts condition expression using `walk_condition()`
    /// - Parses body block via `parse_block()`
    ///
    /// Example:
    /// ```plaintext
    /// while obedience < 100 {
    ///     walk("path")
    /// }
    /// ```
    ///
    /// Returns:
    /// - `ScrollNode::Loop { condition, body }`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_loop(&mut self) -> Option<ScrollNode> {
        let _keyword = self.advance()?; // Expect `while`, `for`, etc.
        let condition = self.walk_condition()?; // Extract loop condition
        let body = self.parse_block(); // Extract associated loop body

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "parse_loop",
                &condition,
                "while <condition> { block }",
                "Parsed loop construct",
            )
            .with_location("Parser::parse_loop")
            .with_suggestion("Ensure loop condition and body are syntactically aligned");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Loop {
            condition,
            body: vec![body.unwrap()],
        })
    }

    /// 🔗 Parses a bracketed sequence of instructions into `ScrollNode::Block`.
    ///
    /// Pattern:
    /// - `[ instr1, instr2, instr3 ]`
    ///
    /// This allows inline grouping of multiple nodes without block indentation.
    /// Useful for array-style sequences or scroll-style command chains.
    ///
    /// Logic:
    /// - Consumes opening bracket `[`, then reads nested instructions
    /// - Dispatches each inner token via `parse_node()`
    /// - Stops at closing bracket `]`
    ///
    /// Example:
    /// ```plaintext
    /// [ walk("north"), invoke("bless"), proclaim("victory") ]
    /// ```
    ///
    /// Returns:
    /// - `ScrollNode::Block(Vec<ScrollNode>)`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_instruction_group(&mut self) -> Option<ScrollNode> {
        let _open = self.advance()?; // Consume `[` token
        let mut group_nodes = vec![];

        while let Some(token) = self.peek() {
            if token.value == "]" {
                self.advance(); // Consume closing `]`
                break;
            }

            // Delegate node parsing for each group element
            if let Some(node) = self.parse_node() {
                group_nodes.push(node);
            } else {
                break;
            }
        }

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "parse_instruction_group",
                "[ ... ]",
                "instruction_group",
                &format!("Grouped {} nodes", group_nodes.len()),
            )
            .with_location("Parser::parse_instruction_group")
            .with_suggestion("Ensure all instructions inside brackets are valid scroll nodes");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Block(group_nodes))
    }

    /// 📦 Parses a scroll import statement into `ScrollNode::Import`.
    ///
    /// Pattern:
    /// - `import "path/to/scroll.omni"`
    ///
    /// This currently supports **literal string imports only**—meaning the path must be
    /// wrapped in quotes and appear directly after the `import` keyword.
    ///
    /// Logic:
    /// - Consumes `import` token
    /// - Expects next token to be a valid string literal
    ///
    /// Example:
    /// ```plaintext
    /// import "modules/divine_scroll.omni"
    /// ```
    ///
    /// Returns:
    /// - `ScrollNode::Import(path_string)`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_import(&mut self) -> Option<ScrollNode> {
        let _keyword = self.advance()?; // 📥 Consume `import`
        let path_token = self.advance()?; // 📦 Expect string literal path (e.g. `"scroll.omni"`)

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "parse_import",
                &path_token.value,
                "import \"filename\"",
                "Parsed import path",
            )
            .with_location("Parser::parse_import")
            .with_suggestion("Validate path is a literal and properly quoted");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Import(path_token.value)) // 🔗 Emit import node
    }

    /// 🔚 Parses a return statement into `ScrollNode::Return`.
    ///
    /// Pattern:
    /// - `return value`
    ///
    /// This function currently supports **single-token return values**,
    /// such as a literal, variable, or simple identifier.
    ///
    /// Logic:
    /// - Consumes `return` keyword
    /// - Extracts one following token (if any) as the return payload
    ///
    /// Example:
    /// ```plaintext
    /// return "peace"
    /// return result
    /// ```
    ///
    /// Returns:
    /// - `ScrollNode::Return(value_string)`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_return(&mut self) -> Option<ScrollNode> {
        let _keyword = self.advance()?; // ⏎ Consume `return`
        let value_token = self.advance()?; // 🔍 Extract following literal or identifier
        let value = value_token.value;

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "parse_return",
                &value,
                "return <value>",
                "Captured return statement",
            )
            .with_location("Parser::parse_return")
            .with_suggestion("Support expressions as future return values");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Return(value)) // 📤 Emit return node
    }

    /// 🔮 Parses a function call into `ScrollNode::Call`.
    ///
    /// Pattern:
    /// - `function(arg1, arg2, ...)`
    ///
    /// This is only invoked when the `parse_assignment_or_call` detects a `(`
    /// following an identifier. It parses **comma-separated** arguments and
    /// emits a callable node structure.
    ///
    /// 🧾 Example:
    /// ```plaintext
    /// bless("family", "peace")
    /// ```
    ///
    /// Logic Flow:
    /// - Consume identifier (function name)
    /// - Verify and consume `(`
    /// - Walk argument list until `)`
    /// - Return as `ScrollNode::Call`
    ///
    /// Notes:
    /// - Currently supports **flat** arguments only (no nested expressions)
    /// - Commas are treated as separators, not syntax
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_call(&mut self, function_token: String) -> Option<ScrollNode> {
        let open_paren = self.advance()?; // 🔓 Expect `(`

        if open_paren.value != "(" {
            return Some(ScrollNode::Error(
                "Expected '(' after function name.".into(),
            ));
        }

        let mut args = vec![];

        // 🔁 Walk tokens until closing paren or stream end
        while let Some(token) = self.peek() {
            if token.value == ")" {
                self.advance(); // ✅ Close the argument list
                break;
            }

            let arg_token = self.advance()?; // ➕ Extract argument
            if arg_token.token_type != TokenType::Punctuation {
                args.push(arg_token.value);
            }

            // Skip over commas
            if let Some(t) = self.peek() {
                if t.value == "," {
                    self.advance();
                }
            }
        }

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "parse_call",
                &function_token.value,
                "call(function, args...)",
                &format!("{} args parsed", args.len()),
            )
            .with_location("Parser::parse_call")
            .with_suggestion("Consider supporting nested expressions in arguments");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Call {
            function: function_token,
            args,
        })
    }

    /// 🧷 Parses a variable assignment into `ScrollNode::Assignment`.
    ///
    /// Pattern:
    /// - `target = value`
    ///
    /// This function is **usually called directly** from external paths
    /// that already resolved the target, allowing partial injection.
    ///
    /// 🧾 Example:
    /// ```plaintext
    /// truth = "eternal"
    /// ```
    ///
    /// Logic:
    /// - Confirms presence of `=`
    /// - Captures next token as right-hand side value
    ///
    /// Returns:
    /// - `ScrollNode::Assignment { target, value }`
    ///
    /// Error Handling:
    /// - Emits `Error` node if `=` is missing
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_assignment(&mut self, target: String) -> Option<ScrollNode> {
        let next = self.advance()?; // 🔍 Expect '='

        if next.value != "=" {
            return Some(ScrollNode::Error(format!(
                "Expected '=' after '{}', got '{}'",
                target, next.value
            )));
        }

        let value_token = self.advance()?; // 🧾 Right-hand value

        Some(ScrollNode::Assignment {
            target,
            value: value_token.value,
        })
    }

    /// 🧱 Parses a grouped logic block delimited by `{ ... }`.
    ///
    /// Used in compound constructs like functions, conditionals, loops,
    /// and any nested scroll sequences.
    ///
    /// Pattern:
    /// ```plaintext
    /// {
    ///     instruction "value"
    ///     let x = 5
    /// }
    /// ```
    ///
    /// Flow:
    /// - Expects `{` to begin
    /// - Repeatedly calls `parse_node()` until it finds `}`
    /// - Emits `ScrollNode::Block` with collected inner nodes
    ///
    /// Notes:
    /// - Gracefully halts if malformed or EOF encountered mid-block
    /// - Debug trail logs node count for auditing
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_block(&mut self) -> Option<ScrollNode> {
        // 🧩 Expecting opening `{` group marker
        let open = self.advance()?;
        if open.value != "{" {
            return Some(ScrollNode::Error(format!(
                "Expected '{{' to open block, found '{}'",
                open.value
            )));
        }

        let mut nodes = vec![];

        // 🌀 Loop until closing `}` or stream ends
        while let Some(token) = self.peek() {
            if token.token_type == TokenType::GroupMarker && token.value == "}" {
                self.advance(); // ✅ Close the group
                break;
            }

            if let Some(node) = self.parse_node() {
                nodes.push(node); // 🧱 Add parsed child node
            } else {
                break; // 🚨 Stop on failure to parse
            }
        }

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "parse_block",
                "{...}",
                "Block",
                &format!("Parsed block with {} nodes", nodes.len()),
            )
            .with_location("Parser::parse_block")
            .with_suggestion("Ensure matching `{}` and valid inner structure");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Block(nodes))
    }

    // ===============================================
    // 🔐 Instruction Decoding & Grammar Checking
    // ===============================================

    /// 🧩 Attempts to decode a raw instruction token using the instruction registry.
    ///
    /// Looks up a token against the global `InstructionRegistry`, which houses
    /// all recognized opcode-like commands and sacred instruction keywords.
    ///
    /// Outcome:
    /// - `Some(String)` if the instruction is valid
    /// - `None` if the instruction is unknown or malformed
    ///
    /// 📌 Example:
    /// - Input: Token with value `"invoke"`
    /// - Output: Some("invoke") if `"invoke"` is in the registry
    ///
    /// 🔍 Debug mode logs:
    /// - Whether the instruction is known
    /// - Suggests registry check or update if unrecognized
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn decode_instruction(&self, token: &Token) -> Option<String> {
        use crate::instruction_registry::get_instruction_registry; // 🧠 Registry of known instruction names

        let instruction = token.value.clone(); // Already a String 🧽 Normalize for consistent lookup

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let found = InstructionRegistry::contains(&instruction);
            let expected = "Known instruction";
            let actual = if found {
                instruction.clone()
            } else {
                "Unknown".into()
            };

            let entry = DebugEntry::new("decode_instruction", &instruction, expected, &actual)
                .with_location("Parser::decode_instruction")
                .with_suggestion("Verify token is a valid instruction or update registry");
            println!("{entry:#?}");
        }

        if get_instruction_registry().contains_key(instruction.as_str()) {
            Some(instruction)
        } else {
            None
        }
    }

    /// 🧪 Validates if a scroll sentence aligns with grammar expectations.
    ///
    /// This is a basic structure validator for subject–verb–object form.
    /// Currently:
    /// - Ensures non-empty subject and verb
    /// - Allows optional object if present
    ///
    /// Future upgrades:
    /// - Schema-matching by verb roles
    /// - Object-verb compatibility matrices
    /// - Modifier and preposition handling
    ///
    /// 📌 Usage:
    /// - Called during scroll sentence parsing for soft grammar enforcement
    ///
    /// 📊 Debug logging (if enabled):
    /// - Shows raw SVO values
    /// - Suggests integration with more advanced validation logic
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn is_valid_sentence(&self, subject: &str, verb: &str, object: Option<&str>) -> bool {
        let has_subject = !subject.trim().is_empty();
        let has_verb = !verb.trim().is_empty();
        let has_valid_object = object.map(|o| !o.trim().is_empty()).unwrap_or(true);

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let expected = "Non-empty subject and verb, optional object";
            let actual = format!("s='{}', v='{}', o='{:?}'", subject, verb, object);

            let entry = DebugEntry::new("is_valid_sentence", verb, expected, &actual)
                .with_location("Parser::is_valid_sentence")
                .with_suggestion("Improve validation using verb-object grammar matrix");
            println!("{entry:#?}");
        }

        has_subject && has_verb && has_valid_object
    }
}

// ===================================================
// 🔚 Closing Block — Output & Validation Hooks
// ===================================================
//
// 🧾 Overview:
//   - This section defines post-parse behavior for ScrollTree,
//     including format conversion and spiritual alignment validation.
//
// ⚙️ Engine Scope:
//   - Converts internal node structures into `.stone` form
//   - Offers semantic or scroll-style interpretation against the registry
//
// ---------------------------------------------------
// 🚨 Version Control Notice:
// ---------------------------------------------------
//   This logic is part of the OmniCode Parser Scroll.
//   Any updates here must be reviewed for downstream effects.
//   Comments marked ⚠️ indicate validator or compiler interface contracts.
//
// ---------------------------------------------------
// 📅 Last Updated:
// ---------------------------------------------------
//   Version       : v0.0.1
//   Last Updated  : 2025-06-04
//   Change Log    : Initial closing logic for ScrollTree output + validation
//
// ---------------------------------------------------
// 🔮 Notes for Next Phase:
// ---------------------------------------------------
// - Consider expanding `to_stone()` to serialize node metadata.
// - Future alignment check may include trust-level tiers or discrepancy tags.
// - These outputs will flow into the OmniDebug protocol.
//
// ---------------------------------------------------

impl ScrollTree {
    /// 🔁 Converts `ScrollTree` into intermediate `.stone` format.
    ///
    /// This method serializes all top-level nodes into a placeholder format
    /// used for debugging, transport, or readable display during IR inspection.
    /// Each node is converted into a line or block, depending on type.
    ///
    /// 🧱 Future evolution:
    /// - Prettify block formatting
    /// - Support nested indentation
    /// - Integrate schema-aware emitters
    pub fn to_stone(&self) -> String {
        // 📜 Begin composing `.stone` lines from node contents
        let mut output = String::new();
        for node in &self.nodes {
            match node {
                ScrollNode::Instruction { name, args } => {
                    output += &format!("{} {}\n", name, args.join(" "));
                }
                ScrollNode::ScrollSentence {
                    subject,
                    verb,
                    object,
                } => {
                    output += &format!("{} {} {}\n", subject, verb, object);
                }
                ScrollNode::Assignment { target, value } => {
                    output += &format!("{} = {}\n", target, value);
                }
                ScrollNode::Literal(val) => {
                    output += &format!("literal {}\n", val);
                }
                ScrollNode::Metadata(data) => {
                    output += &format!("meta {}\n", data);
                }
                ScrollNode::Block(inner) => {
                    output += "{\n";
                    for child in inner {
                        output += &format!("{:?}\n", child); // 📌 Replace with prettier .stone render
                    }
                    output += "}\n";
                }
                ScrollNode::Error(err) => {
                    output += &format!("!error {}\n", err);
                }
                ScrollNode::Declaration { name, dtype } => {
                    let type_part = dtype.clone().unwrap_or_else(|| "Unknown".into());
                    output += &format!("let {}: {}\n", name, type_part);
                }
                ScrollNode::Conditional { condition, .. } => {
                    output += &format!("if {}\n", condition);
                }
                ScrollNode::Loop { condition, .. } => {
                    output += &format!("loop {}\n", condition);
                }
                ScrollNode::Import(path) => {
                    output += &format!("import {}\n", path);
                }
                ScrollNode::Return(value) => {
                    output += &format!("return {}\n", value);
                }
                ScrollNode::Call { function, args } => {
                    output += &format!("{}({})\n", function, args.join(", "));
                }
                ScrollNode::Comment(text) => {
                    output += &format!("// {}\n", text);
                }
            }
        }
        output
    }

    /// 📖 Validates the `ScrollTree` against .logos grammar and Scripture alignment.
    ///
    /// Placeholder for spiritual validation logic.
    /// Will eventually walk each scroll node against a sentence validator
    /// wired to Scripture schema, checking alignment to Kingdom protocol.
    ///
    /// 🌾 Use case:
    /// - Grammar audits
    /// - Sentence holiness checks
    /// - Instruction alignment with truth
    ///
    /// 🔍 Debug output (when enabled):
    /// - Shows validation phase
    /// - Suggests future `.logos` wiring
    pub fn validate_with_scripture(&self) -> bool {
        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "validate_with_scripture",
                "ScrollTree",
                "Spiritual grammar",
                "Validation passed",
            )
            .with_location("ScrollTree::validate_with_scripture")
            .with_suggestion("Wire in `.logos` sentence walker and Scripture hooks");
            println!("{entry:#?}");
        }

        // 🛐 TODO: Implement spiritual grammar validator
        // ------------------------------------------------------
        // - Hook into the .logos engine and instruction schema
        // - Walk each ScrollNode for alignment with sacred patterns
        // - Validate ScrollSentences by subject–verb–object logic
        // - Verify instruction usage aligns with .logos roles
        // - Attach scripture references or error severity if drifted
        // - Return `false` on fatal theological misalignment
        // ------------------------------------------------------

        true // Temporary grace — assumes scroll is valid
    }
}
