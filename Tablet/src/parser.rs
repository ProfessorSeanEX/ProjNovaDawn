// ===============================================
// 📜 Metadata — Parser v0.0.3 (Tablet Priest)
// ===============================================
// _author_:         Seanje Lenox-Wise / Nova Dawn
// _version_:        0.0.3
// _status_:         Dev
// _phase_:          Phase 3 — Post-Stub Validation (Scroll-Aware)
// _created_:        2025-06-04
// _last updated_:   2025-06-14
// _license_:        CreativeWorkzStudio LLC — Kingdom-First Proprietary Use
// _component_:      Parser (Tablet Cog)
// _project_:        OmniCode / Millennium OS
// _description_:    Converts token streams into Scroll Trees (OmniCode ASTs) using sentence-based grammar rules and instruction logic.
//
// _grammar schema_: Subject–Verb–Object, Instruction (opcode-style), Expression Blocks
// _validation hooks_: Debug-aware grammar validator, operand resolver integration
//
// _notes_:
// - Parses tokenized input into executable ScrollNode variants
// - Supports sentence, opcode, and logic block structures
// - Instruction decoder uses registry-backed lookup
// - Grammar validation supports early SVO and return checks
// - Operand resolver refactors handled where applicable
// - `.stone` output format is intermediate and version-neutral
// - Future support: Scripture-aligned .logos hooks, type propagation, schema reflection
//
// ===============================================

// ===============================================
// 📖 Opening — Parser Module Purpose & Design
// ===============================================
// This module interprets NovaScript token streams into scroll structures.
// It transforms raw symbols into spiritual and structural meaning,
// producing a ScrollTree composed of ScrollNodes for operand resolution.
//
// The Parser sits between the Tokenizer and Operand Resolver (Bearer),
// bridging syntactic intention with resolved instruction form.
// It reports misalignments to the Watchtower via debug entries and severity signals.

// ===============================================
// 📦 Imports — Dependencies for Parser Construction
// ===============================================
// These imports are grouped by origin and function:
// • Standard: parsing queues, formatting tools
// • External: timestamping for trace metadata
// • Internal: tokenizer, instruction metadata, operand resolver
// • Debugging: Watchtower trace scaffolding

// === Standard Library ===
use std::collections::VecDeque; // 🔁 Token queue for recursive descent parsing
#[allow(unused_imports)]
use std::fmt; // 🧾 Enables custom Display / Debug formatting for ScrollTree or error logs

// === External Crates ===
#[allow(unused_imports)]
use chrono::Utc; // 🕰 Timestamps parse events for trace diagnostics and scroll lineage

// === Internal Modules ===
use super::instruction_registry::get_instruction_registry; // 📚 Instruction schema registry — validates opcodes and operand expectations
use crate::operand_resolver::Bearer;
use crate::tokenizer::{Token, TokenType}; // 🧱 Core units of NovaScript — value, type, and source position // 🧱 Operand Resolver — performs operand classification after parsing

// === Watchtower Integration ===
#[allow(unused_imports)]
use watchtower::debugger::{
    DebugEntry,    // 📋 Individual trace record — includes line, source, and severity
    DebugResponse, // 🔧 Feedback object for system-level debugging or confirmation
    Severity,      // 🌡 Classifies alignment state: Valid, Drifted, Fatal, etc.
}; // 🪛 The Watchtower watches over all misalignment and confirmation logs

// ===============================================
// 📦 Foundational Declarations — Core Structures
// ===============================================
// This section defines the elemental structures of the Scroll Parser.
// These declarations form the spiritual and architectural baseline for parsing logic,
// representing NovaScript scrolls in a structured, intermediate form.
//
// Execution logic does not live here—only the **types** and **forms** the system will interpret.
//
// Included Structures:
// • `ScrollNode` — the atomic meaning-bearing units of NovaScript
// • `ScrollTree` — an AST-like container for parsed scrolls
// • `ScrollParser` (legacy) — basic token walker for backward compatibility
// • `Parser` — the current, operand-aware parser interface

// ------------------------------------------------
// 🧩 ScrollNode — Sentence-Level Grammar Structures
// ------------------------------------------------
/// 🧩 Enum representing all valid node types produced by the parser.
/// These nodes are not yet operands or bindings—they are raw structures,
/// capturing grammatical meaning and scroll intent in intermediate form.
#[derive(Debug, Clone)]
pub enum ScrollNode {
    Instruction {
        name: String,
        args: Vec<String>,
    },
    // 🪶 An explicit instruction invocation with positional arguments
    //     → e.g., `invoke("light.fire")`
    ScrollSentence {
        subject: String,
        verb: String,
        object: String,
    },
    // 🧾 A NovaScript sentence with SVO structure
    //     → e.g., `Let flame be set to 5`
    Assignment {
        target: String,
        value: String,
    },
    // 📦 A binding or reassignment expression
    //     → e.g., `holiness = 100`
    Literal(String),
    // ✍️ A standalone literal value: number, boolean, or raw string
    Metadata(String),
    // 📘 Metadata notation (e.g., authorship, tags)
    //     → e.g., `// author: seanje`
    Block(Vec<ScrollNode>),
    // 🧱 A grouped node set (typically for loops or conditionals)
    Error(String),
    // ❌ An error node—holds parse failure diagnostics

    // ⚙️ Optional & emerging structural variants
    Declaration {
        name: String,
        dtype: Option<String>,
    },
    // ✒️ A variable or type declaration
    //     → e.g., `let x: int`
    Conditional {
        condition: String,
        body: Vec<ScrollNode>,
    },
    // 🧭 An `if` or `match` block with scoped condition and child nodes
    Loop {
        condition: String,
        body: Vec<ScrollNode>,
    },
    // 🔁 A repeat-until or while-style loop with inner body
    Import(String),
    // 📥 Scroll or module import directive
    Return(String),
    // 🔚 Early return with output value
    Call {
        function: String,
        args: Vec<String>,
    },
    // 📞 A function call node (used in nested or procedural expressions)
    Comment(String),
    // 💬 A non-evaluated annotation (inline or floating comment)
}

// ------------------------------------------------
// 📚 ScrollTree — Top-Level Scroll Container
// ------------------------------------------------
/// 📚 Represents a fully parsed NovaScript scroll.
/// Functions as the top-level AST, ordered by sequence of declarations.
pub struct ScrollTree {
    pub nodes: Vec<ScrollNode>,
    // 🔗 All top-level nodes in order of appearance (execution flow matters)
}
// ------------------------------------------------
// 🌀 ScrollParser — Legacy Non-Resolving Parser
// ------------------------------------------------
/// 🌀 Legacy parser implementation — retained for test scaffolding.
/// Uses a token queue and does **not** perform operand resolution.
pub struct ScrollParser {
    tokens: VecDeque<Token>,
    // 🪙 Token queue (ordered consumption during basic parsing)
    scroll: Vec<ScrollNode>,
    // 🧾 Accumulated result vector (pre-resolution AST)
}

// ------------------------------------------------
// 🎯 Parser — Operand-Aware Sentence Parser
// ------------------------------------------------
/// 🎯 The primary parser structure.
/// Parses a linear stream of tokens into `ScrollNode`s and prepares for operand resolution.
/// Does not build operand structures directly, but enables schema-ready flow into the Bearer.
pub struct Parser {
    tokens: Vec<Token>,
    // 📜 Flat token stream (from tokenizer output)
    position: usize,
    // 🔍 Cursor within token stream for ordered access
}

// ===============================================
// 🛠 Constructors & Initializers
// ===============================================
// Responsible for preparing parser structures before interpretation begins.
// These methods instantiate stateful containers for token walking, node construction,
// and scroll preparation. No parsing logic occurs here—only structure creation.

// ===============================================
// === ScrollParser — Legacy Constructor ===
// ===============================================

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

// ===============================================
// === Parser — Primary Constructor ==
// ===============================================

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
// ===============================================

// ===============================================
// === Error Type Enum ===
// ===============================================

/// 🧯 Enum representing categories of parser failure.
/// Each variant defines a unique class of misalignment between scroll syntax and expected sentence logic.
#[derive(Debug)]
pub enum ParseErrorType {
    UnexpectedEOF,           // 📉 Ran out of tokens mid-expression or sentence
    InvalidArgument(String), // ❌ Argument found but doesn't match expected type or structure
    UnexpectedToken,         // 🌀 Found token was out of place grammatically
    MissingToken,            // ⛔ Expected token (e.g., verb, assignment) was not found
    InvalidInstruction,      // 📚 Instruction not found in registry
    InvalidGrammar,          // 🪓 Sentence structure broke grammatical covenant
    UnknownSymbol,           // 🕳 Reference used but not declared or defined
}

// ===============================================
// === ParseError Struct ===
// ===============================================

/// 🩺 Represents a single error encountered while parsing a scroll.
/// Contains type, readable message, and positional metadata for traceability.
#[derive(Debug)]
pub struct ParseError {
    pub kind: ParseErrorType, // 🧭 What kind of misalignment occurred
    pub message: String,      // 📜 Human-readable explanation
    pub line: usize,          // 📍 Where in the scroll the error emerged (line number)
    pub column: usize,        // 📏 Specific character offset in the line
}

impl ParseError {
    /// 🔧 Create a new parse error with full detail.
    /// Used when the parser has full visibility into the scroll position and context.
    pub fn new(
        kind: ParseErrorType,
        message: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            kind,                    // Error category
            message: message.into(), // Description passed in as string or &str
            line,                    // Line number captured during parsing
            column,                  // Column position captured during parsing
        }
    }

    /// 📃 Lightweight builder for structural errors without location.
    /// Used in early failure stages or when positional data is unavailable.
    pub fn basic(kind: ParseErrorType) -> Self {
        Self {
            message: format!("Parser failed due to: {:?}", kind), // Default generic message
            kind,    // Still provides error classification
            line: 0, // Defaults to zero when unknown
            column: 0,
        }
    }
}

// ===============================================
// === Trait Implementations ===
// ===============================================

/// 💡 Enables use of `?` on `ParseErrorType` inside parser Result functions.
/// This makes error propagation elegant and idiomatic—scrolls abort cleanly on breach.
impl From<ParseErrorType> for ParseError {
    fn from(kind: ParseErrorType) -> Self {
        ParseError::basic(kind) // Delegates to the generic builder if only the type is known
    }
}

/// 🧾 Optional: Pretty formatter for logging, CLI display, or dev tools.
/// Produces a clean trace for Watchtower or inline scroll diagnostics.
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Line {}, Col {}] {:?}: {}", // Formatted trace style for debug panels
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
        let token = self.peek()?.clone(); // 👁 Preview current token without consuming it

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
                self.advance(); // ⏭ Skip token to avoid infinite loop

                // ❌ Return error node with embedded token context for debugging
                Some(ScrollNode::Error(format!(
                    "Unrecognized token: {}",
                    token.value
                )))
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

    // -----------------------------------------------
    // 🎯 Cursor Movement Utilities
    // -----------------------------------------------
    // These methods control how the parser walks the token stream.
    // They do not interpret meaning—only manage position within the scroll.
    //
    // Think of them as the parser’s eyes and legs:
    // • `advance()` moves forward one step and returns the token
    // • `peek()` looks at the current token without stepping

    /// 📌 Advance the token stream — move cursor forward and consume token.
    ///
    /// Retrieves and returns the token at the current position,
    /// then increments the parser's position index by one.
    ///
    /// 🔁 Returns:
    /// • `Some(Token)` if a token exists at the current index
    /// • `None` if the end of the token stream has been reached
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn advance(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.position).cloned(); // 🧤 Clone ensures original token remains intact
        if tok.is_some() {
            self.position += 1; // ➡️ Move parser cursor to next token
        }
        tok // 🎯 Return the consumed token (or None if at EOF)
    }

    /// 🔍 Peek at the current token without consuming it.
    ///
    /// Allows the parser to preview the next token to decide routing
    /// without altering the current cursor position.
    ///
    /// 🔭 This is essential for grammar branching (e.g., assignment vs call)
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.position) // 🧿 Non-consuming view of current token
    }

    // -----------------------------------------------
    // ⚙️ Instruction Parser
    // -----------------------------------------------
    // This walker interprets tokens that represent system instructions,
    // turning them into `ScrollNode::Instruction` structures.
    //
    // Instructions act like mini-opcodes or method calls in NovaScript,
    // usually followed by one or more argument tokens.
    //
    // 🧭 Example:
    //   Input:    invoke "truth" +5
    //   Output:   ScrollNode::Instruction { name: "invoke", args: ["truth", "+5"] }

    /// ⚙️ Instruction walker — parses an opcode-like token into `ScrollNode::Instruction`.
    ///
    /// - Consumes the instruction keyword (e.g., `invoke`)
    /// - Collects all following tokens that qualify as arguments
    /// - Terminates when encountering invalid types or scroll delimiters
    ///
    /// 🛠 Grammar Recognized:
    /// - `invoke "truth"`
    /// - `bless x +7`
    ///
    /// 🔧 Debug mode (if enabled):
    /// - Emits log of instruction name and number of args parsed
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_instruction(&mut self) -> Option<ScrollNode> {
        let token = self.advance()?; // 🎯 Step forward to consume the instruction keyword

        // 🚨 Validate instruction name against registry before parsing args
        if self.decode_instruction(&token).is_none() {
            return Some(ScrollNode::Error(format!(
                "Unknown instruction '{}'",
                token.value
            )));
        }

        let mut args = Vec::new(); // 📦 Collector for parsed arguments

        // 🔁 Walk forward through valid argument tokens
        while let Some(tok) = self.peek() {
            match tok.token_type {
                TokenType::Literal | TokenType::Identifier | TokenType::Operator => {
                    args.push(tok.value.clone()); // ✍️ Add to argument list
                    self.advance(); // ➡️ Step forward
                }
                TokenType::Whitespace => {
                    self.advance(); // 🧹 Ignore blank space
                }
                _ => break, // ⛔ Stop on block, newline, or invalid type
            }
        }

        // 🧪 Optional debug trace (prints instruction structure)
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

            println!("{entry:#?}"); // 🪵 Emit structured debug report
        }

        // 🧱 Emit constructed instruction node
        Some(ScrollNode::Instruction {
            name: token.value,
            args,
        })
    }

    // -----------------------------------------------
    // 🔢 Literal Parser
    // -----------------------------------------------
    // This walker converts raw literal tokens into `ScrollNode::Literal`.
    //
    // Literals are **primitive values** in NovaScript: strings, numbers,
    // booleans, symbols—any standalone data value not requiring interpretation.
    //
    // ❗ No transformation is performed here (e.g., no type inference or evaluation).
    // That work is deferred to later stages (e.g., operand resolver or executor).
    //
    // 🧭 Example:
    //   Input:    "Holy Fire"
    //   Output:   ScrollNode::Literal("Holy Fire")

    /// ✍️ Parses a literal token into `ScrollNode::Literal`.
    ///
    /// Captures raw, untyped values for use in assignments, calls, or instructions.
    ///
    /// 🔧 Debug mode:
    /// - Emits trace showing token value and node capture
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_literal(&mut self) -> Option<ScrollNode> {
        let token = self.advance()?; // 📥 Step forward and consume literal token

        // 🧪 Optional: emit debug trace of literal interpretation
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

            println!("{entry:#?}"); // 🪵 Emit debug info
        }

        // 📦 Construct and return literal node directly
        Some(ScrollNode::Literal(token.value))
    }

    // -----------------------------------------------
    // 🧮 Assignment / Call Router
    // -----------------------------------------------
    // This walker discerns the purpose of an `Identifier` token.
    //
    // Based on the next token, it resolves whether the identifier begins:
    // • An assignment (e.g., `faith = "substance"`)
    // • A function or instruction call (e.g., `proclaim("truth")`)
    //
    // 🧠 Ambiguity handling:
    // If the next token is neither `=` nor `(`, this walker emits a ScrollNode::Error.
    //
    // 🧭 Grammar Routes:
    // • Identifier + `=` → Assignment
    // • Identifier + `(` → Call
    // • Identifier + ❓ → Error (Unclear purpose)

    /// 🧭 Assignment/Call Branch Walker — resolves identifier intent.
    ///
    /// Parses grammar pattern following an identifier:
    /// - `=` signals assignment
    /// - `(` signals function or opcode call
    ///
    /// 🔧 Debug mode:
    /// - Logs expected pattern and actual token encountered
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_assignment_or_call(&mut self) -> Option<ScrollNode> {
        let identifier = self.advance()?; // 🔑 Consume the symbol name (variable or callable)
        let next = self.peek()?; // 👁️ Peek at the next token to determine intent

        // 🧪 Emit trace for branching decision
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

            println!("{entry:#?}"); // 🪵 Log the branching context
        }

        match next.value.as_str() {
            // 🧾 Assignment pattern: identifier = value
            "=" => {
                self.advance(); // ➡️ Skip the '=' token
                let value_token = self.advance()?; // 📥 Capture right-hand side value

                Some(ScrollNode::Assignment {
                    target: identifier.value, // 🧱 Variable name
                    value: value_token.value, // 🔢 Bound value
                })
            }

            // 📞 Invocation pattern: identifier(...)
            "(" => self.parse_call(identifier.value.clone()),

            // ❌ Invalid pattern — identifier used ambiguously
            _ => Some(ScrollNode::Error(format!(
                "Ambiguous identifier usage near '{}'",
                identifier.value
            ))),
        }
    }

    // -----------------------------------------------
    // 🧾 Metadata & Comment Parsers
    // -----------------------------------------------
    // These walkers capture non-executing elements in the scroll:
    //
    // • `parse_metadata` gathers system-aligned framing tokens like
    //   `//`, `##!`, `///`, etc., which shape execution context.
    //
    // • `parse_comment` captures human-facing remarks embedded in
    //   the scroll to preserve voice, intent, or spiritual witness.
    //
    // Neither produces runnable code, but both are vital for
    // traceability, alignment, and interpretive clarity.

    /// 📘 Metadata Interpreter — parses scroll-level directives.
    ///
    /// These lines begin with `//`, `##!`, `///`, etc., and frame
    /// the scroll’s purpose, ownership, or subsystem scope.
    ///
    /// 🧭 Example:
    /// - `// governs the Gate subsystem` → `ScrollNode::Metadata(...)`
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

            println!("{entry:#?}"); // 🪵 Emit debug log for metadata
        }

        Some(ScrollNode::Metadata(token.value)) // 🧱 Return node containing directive content
    }

    /// 💬 Comment Interpreter — parses human-facing notes.
    ///
    /// These lines are developer-facing insights, poetic markers,
    /// or reminders for future walkers. They are **preserved**, not
    /// executed, and help hold posture within the scroll.
    ///
    /// 🧭 Example:
    /// - `# This section guards NovaGate` → `ScrollNode::Comment(...)`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_comment(&mut self) -> Option<ScrollNode> {
        let token = self.advance()?; // ✏️ Consume comment token from stream

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

            println!("{entry:#?}"); // 📜 Log for dev traceability
        }

        Some(ScrollNode::Comment(token.value)) // 🧱 Return node preserving the voice
    }

    // ===============================================
    // 🧭 Grammar Walkers — Expression & Structure Parsers
    // ===============================================
    // These walkers operate at the **sub-sentence level**, enabling
    // NovaScript to handle inline conditionals, argument groupings,
    // type annotations, and full sentence declarations (SVO).
    //
    // Each function isolates a grammatical substructure that contributes
    // to sentence execution, enabling nested parsing without losing clarity.

    // -----------------------------------------------
    // 🔍 Condition Extractor
    // -----------------------------------------------

    /// 🧠 Condition Extractor — builds conditional expressions.
    ///
    /// Walks forward through the token stream to extract conditions
    /// used in `if`, `when`, `while`, and similar constructs.
    ///
    /// Halts on grammar boundaries like:
    /// • `{` — block open
    /// • `;` — statement end
    ///
    /// 🧭 Example:
    /// `if x > 5 {` → yields `"x > 5"`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn walk_condition(&mut self) -> Option<String> {
        let mut condition = String::new(); // 🧱 Initialize string accumulator

        while let Some(token) = self.peek() {
            match token.value.as_str() {
                "{" | ";" => break, // 🧱 End condition walk at structure boundary
                _ => {
                    let t = self.advance()?; // 🎯 Consume and validate token

                    if !condition.is_empty() {
                        condition.push(' '); // 🔗 Maintain word spacing
                    }

                    condition.push_str(&t.value); // 📎 Append raw token to condition string
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

            println!("{entry:#?}"); // 🪵 Emit trace log for visual feedback
        }

        if condition.is_empty() {
            None // 🚫 No meaningful condition parsed
        } else {
            Some(condition) // ✅ Return the extracted condition string
        }
    }

    // -----------------------------------------------
    // 🧬 Type Annotation Extractor
    // -----------------------------------------------

    /// 🧾 Type Annotation Parser — extracts inline type hints.
    ///
    /// Looks for a type signature immediately after a variable name.
    /// Pattern: `:` → `TypeName`
    ///
    /// 🧭 Example:
    /// `let x: Int` → extracts `"Int"`
    ///
    /// 🔍 This does **not** validate type correctness — that’s the job of the type checker.
    /// Returns `None` if no `:` is found or if type name is missing.
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn walk_type_annotation(&mut self) -> Option<String> {
        let colon = self.peek()?; // 👁️ Peek ahead — expect `:` for type hint
        if colon.value != ":" {
            return None; // 🚫 No type hint present
        }

        self.advance()?; // ✅ Consume `:`

        // 🆕 Check for missing type name after `:` to prevent silent failure
        let next = self.peek()?;
        if next.token_type != TokenType::Identifier {
            return None; // ❗ Invalid type hint — expected identifier
        }

        let type_token = self.advance()?; // 🔤 Capture type name
        Some(type_token.value.clone()) // 📦 Return raw type string
    }

    // -----------------------------------------------
    // 📦 Argument Group Parser
    // -----------------------------------------------

    /// 🪶 Parses a comma-separated argument list enclosed in `(...)`.
    ///
    /// Used in function or instruction calls such as:
    /// `invoke(reveal, glory)` → returns `["reveal", "glory"]`
    ///
    /// 🛠️ Behavior:
    /// - Begins only if opening `(` is detected
    /// - Accepts raw tokens: literals, identifiers, operators, etc.
    /// - Skips over commas cleanly
    /// - Terminates on closing `)`
    ///
    /// 🧭 Returns:
    /// - A `Result<Vec<String>, ParseError>`
    /// - Will return an empty vector if `(` is not found
    ///
    /// ❗ This parser does not perform operand resolution—
    /// it only collects argument **tokens** for later evaluation.
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_argument_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut args = vec![];

        // 🔍 Verify that an argument group is starting with `(`
        let peeked = self.peek().ok_or(ParseErrorType::UnexpectedEOF)?;
        if peeked.value != "(" {
            return Ok(args); // 🫱 No argument list — return empty, not an error
        }
        self.advance(); // ✅ Consume the opening parenthesis

        // 🔁 Continue gathering until closing `)`
        while let Some(token) = self.peek() {
            match token.value.as_str() {
                ")" => {
                    self.advance(); // ✅ End of group — consume `)` and stop
                    break;
                }
                "," => {
                    self.advance(); // 🧹 Skip over delimiter
                    continue;
                }
                _ => {
                    let arg_token = self.advance().ok_or(ParseErrorType::UnexpectedEOF)?; // 🎯 Grab next argument
                    args.push(arg_token.value.clone()); // 📦 Store raw token
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
            println!("{entry:#?}"); // 🪵 Emit debug trace
        }

        Ok(args)
    }

    // -----------------------------------------------
    // 📜 SVO Sentence Walker
    // -----------------------------------------------

    /// 📜 Parses a Scroll Sentence in Subject-Verb-Object form.
    ///
    /// This is a lightweight natural-language interpreter for structured scrolls.
    /// Designed to walk declarations like:
    ///
    /// 🧾 Examples:
    /// - `"The priest speaks truth"`
    /// - `"scroll invokes clarity"`
    ///
    /// 🧠 Behavior:
    /// • Assumes 3 consecutive tokens = subject, verb, object
    /// • Captures only raw strings — no operand resolution
    /// • Used for declarations, prophetic patterns, or natural scroll grammars
    ///
    /// 🛑 Limitations:
    /// • No grammar validation (e.g., missing or extra tokens)
    /// • No type-checking or verb-object agreement (for now)
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_scroll_sentence(&mut self) -> Option<ScrollNode> {
        let subject = self.advance()?.value; // 🙋 Subject — who is acting
        let verb = self.advance()?.value; // 🗣️ Verb — what they do
        let object = self.advance()?.value; // 🎯 Object — what is acted upon

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};

            let phrase = format!("{subject} {verb} {object}"); // 📖 Full sentence preview
            let entry = DebugEntry::new(
                "parse_scroll_sentence",
                &phrase,
                "Subject Verb Object",
                "Parsed SVO triple",
            )
            .with_location("Parser::parse_scroll_sentence")
            .with_suggestion("Validate grammar structure with schema");

            println!("{entry:#?}"); // 🪵 Debug trace output
        }

        Some(ScrollNode::ScrollSentence {
            subject,
            verb,
            object,
        }) // ✅ Output raw SVO node
    }

    /// ===============================================
    /// 📘 Extended Scroll Parsers — Declarations & Blocks
    /// ===============================================

    // -------------------------------
    // 📝 Variable Declaration Parser
    // -------------------------------

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
        let _keyword = self.advance()?; // 🔑 Expect `let`
        let name_token = self.advance()?; // 🧾 Capture variable name
        let dtype = self.walk_type_annotation(); // 🧬 Optional type suffix (e.g., `: Int`)

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

    // -------------------------------
    // 🧠 Conditional Parser
    // -------------------------------

    /// 🔀 Parses a conditional block like `if condition { ... }`
    ///
    /// Structure:
    /// - Consumes conditional keyword (`if`, etc.)
    /// - Extracts condition expression (to be operand-resolved later)
    /// - Parses body block enclosed in `{ ... }`
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
        let _keyword = self.advance()?; // 🧭 Expect conditional keyword
        let condition = self.walk_condition()?; // 🧠 Extract raw condition string (for later operand resolution)
        let body = self.parse_block()?; // 📦 Parse block under condition

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
            body: vec![body], // 🔗 Emit conditional with 1-block body
        })
    }

    // -------------------------------
    // 🔁 Loop Construct Parser
    // -------------------------------

    /// 🔁 Parses a loop construct into `ScrollNode::Loop`.
    ///
    /// Supports:
    /// - `while <condition> { ... }`
    ///
    /// Flow:
    /// - Consumes loop keyword (`while`, etc.)
    /// - Extracts condition expression string (to be operand-resolved later)
    /// - Parses inner block sequence
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
        let _keyword = self.advance()?; // 🧭 Expect loop keyword
        let condition = self.walk_condition()?; // 🧠 Capture loop condition string (raw)
        let body = self.parse_block()?; // 📦 Parse the loop body block

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
            body: vec![body],
        })
    }

    // -------------------------------
    // 📦 Instruction Group Parser (Bracket Form)
    // -------------------------------

    /// 🔗 Parses a bracketed sequence of instructions into `ScrollNode::Block`.
    ///
    /// Pattern:
    /// - `[ instr1, instr2, instr3 ]`
    ///
    /// Use Case:
    /// - Enables inline scroll-style command chains
    /// - Treats grouped nodes as a list of operands or sequence
    ///
    /// Flow:
    /// - Expects opening `[`
    /// - Delegates parsing to `parse_node()` until `]`
    /// - Collects results into a single `ScrollNode::Block`
    ///
    /// Example:
    /// ```plaintext
    /// [ walk("north"), invoke("bless"), proclaim("victory") ]
    /// ```
    ///
    /// Operand Note:
    /// - Each child node may contain operand expressions that must be resolved later
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_instruction_group(&mut self) -> Option<ScrollNode> {
        let _open = self.advance()?; // 🔓 Consume `[`
        let mut group_nodes = vec![];

        while let Some(token) = self.peek() {
            if token.value == "]" {
                self.advance(); // ✅ Consume closing `]`
                break;
            }

            // ✨ Recursively parse nested instructions
            if let Some(node) = self.parse_node() {
                group_nodes.push(node);
            } else {
                break; // 🚧 Stop on invalid node
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

    // -------------------------------
    // 📥 Import Statement Parser
    // -------------------------------

    /// 📦 Parses a scroll import statement into `ScrollNode::Import`.
    ///
    /// Pattern:
    /// - `import "path/to/scroll.omni"`
    ///
    /// Logic:
    /// - Consumes `import` keyword
    /// - Expects a valid string literal token as the file path
    ///
    /// Example:
    /// ```plaintext
    /// import "modules/divine_scroll.omni"
    /// ```
    ///
    /// ⚠️ Only supports **literal** string imports (no dynamic expressions).
    /// Emits a `ScrollNode::Import` if successful.
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_import(&mut self) -> Option<ScrollNode> {
        let _keyword = self.advance()?; // 📥 Consume `import`
        let path_token = self.advance()?; // 📦 Expect string literal path

        // ⚠️ Validate that the token is a properly quoted string
        if !path_token.value.starts_with('"') || !path_token.value.ends_with('"') {
            return Some(ScrollNode::Error(
                "Import path must be a quoted string literal.".into(),
            ));
        }

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

    // -------------------------------
    // 🔚 Return Statement Parser
    // -------------------------------

    /// 🔚 Parses a return statement into `ScrollNode::Return`.
    ///
    /// 🚧 Currently supports single resolved operand only.
    /// Full expression and block return support planned.
    ///
    /// Pattern:
    /// - `return value`
    ///
    /// Emits:
    /// - `ScrollNode::Return(Operand)`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_return(&mut self) -> Option<ScrollNode> {
        let _keyword = self.advance()?; // ⏎ Consume `return`

        let operand = self.walk_operand()?; // 🧠 Resolve value into Operand

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "parse_return",
                &format!("{operand:?}"),
                "return <value>",
                "Captured return statement (resolved)",
            )
            .with_location("Parser::parse_return")
            .with_suggestion("Support expression trees and multi-token operands in future");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Return(operand)) // 📤 Emit full return node
    }

    // -------------------------------
    // 📞 Function Call Parser
    // -------------------------------

    /// 🔮 Parses a function call into `ScrollNode::Call`.
    ///
    /// Pattern:
    /// - `function(arg1, arg2, ...)`
    ///
    /// Logic Flow:
    /// - Consumes function name and `(`
    /// - Resolves each argument using `walk_operand()`
    /// - Emits `ScrollNode::Call`
    ///
    /// Notes:
    /// - Supports flat arguments only (for now)
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_call(&mut self, function_token: String) -> Option<ScrollNode> {
        let open_paren = self.advance()?; // 🔓 Expect '('

        if open_paren.value != "(" {
            return Some(ScrollNode::Error(
                "Expected '(' after function name.".into(),
            ));
        }

        let mut args = vec![];

        while let Some(token) = self.peek() {
            if token.value == ")" {
                self.advance(); // ✅ Close the argument list
                break;
            }

            if let Some(arg) = self.walk_operand() {
                args.push(arg); // 🎯 Resolve argument via operand logic
            } else {
                return Some(ScrollNode::Error(
                    "Invalid argument in function call.".into(),
                ));
            }

            if let Some(next) = self.peek() {
                if next.value == "," {
                    self.advance(); // Skip comma separator
                }
            }
        }

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let entry = DebugEntry::new(
                "parse_call",
                &function_token,
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

    // -------------------------------
    // 🧾 Assignment Parser
    // -------------------------------

    /// 🧷 Parses a variable assignment into `ScrollNode::Assignment`.
    ///
    /// Pattern:
    /// - `name = value`
    ///
    /// Logic Flow:
    /// - Confirms presence of `=` after identifier
    /// - Resolves right-hand side using `walk_operand()`
    ///
    /// Returns:
    /// - `ScrollNode::Assignment { target, value }`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_assignment(&mut self, target: String) -> Option<ScrollNode> {
        let next = self.advance()?; // 🔍 Expect '='

        if next.value != "=" {
            return Some(ScrollNode::Error(format!(
                "Expected '=' after '{}', got '{}'",
                target, next.value
            )));
        }

        let value = self.walk_operand()?; // 🎯 Parse right-hand side as operand

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let display = format!("{target} = {value}");
            let entry = DebugEntry::new(
                "parse_assignment",
                &display,
                "name = value",
                "Parsed assignment pair",
            )
            .with_location("Parser::parse_assignment")
            .with_suggestion("Ensure variable exists and value is valid expression");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Assignment { target, value })
    }

    // -------------------------------
    // 🧱 Logic Block Parser
    // -------------------------------

    /// 🧱 Parses a grouped logic block delimited by `{ ... }`.
    ///
    /// Pattern:
    /// - `{ instruction_1; instruction_2; ... }`
    ///
    /// Logic Flow:
    /// - Confirms opening `{`
    /// - Repeatedly calls `parse_node()` until `}`
    /// - Collects all valid inner nodes
    ///
    /// Notes:
    /// - Gracefully halts if malformed or EOF is encountered mid-block
    /// - Used for conditional bodies, loops, and nested scroll logic
    ///
    /// Returns:
    /// - `ScrollNode::Block(Vec<ScrollNode>)`
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn parse_block(&mut self) -> Option<ScrollNode> {
        let open = self.advance()?; // 🧩 Expect opening `{`
        if open.value != "{" {
            return Some(ScrollNode::Error(format!(
                "Expected '{{' to open block, found '{}'",
                open.value
            )));
        }

        let mut nodes = vec![];

        // 🌀 Walk through each inner node until `}` is found
        while let Some(token) = self.peek() {
            if token.token_type == TokenType::GroupMarker && token.value == "}" {
                self.advance(); // ✅ Close the block
                break;
            }

            if let Some(node) = self.parse_node() {
                nodes.push(node); // 🧱 Push parsed scroll node
            } else {
                break; // 🚨 Exit on invalid node
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
            .with_suggestion("Ensure matching braces and valid scroll logic inside block");
            println!("{entry:#?}");
        }

        Some(ScrollNode::Block(nodes))
    }

    // ===============================================
    // 🔐 Instruction Decoding & Grammar Checking
    // ===============================================

    // -------------------------------
    // 🧠 Instruction Decoder
    // -------------------------------

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
        use super::instruction_registry::get_instruction_registry;

        let instruction = token.value.clone(); // 🧽 Normalize for consistent lookup

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let found = InstructionRegistry::contains(&instruction);
            let entry = DebugEntry::new(
                "decode_instruction",
                &instruction,
                "Known instruction",
                if found { &instruction } else { "Unknown" },
            )
            .with_location("Parser::decode_instruction")
            .with_suggestion("Verify token is a valid instruction or update registry");
            println!("{entry:#?}");
        }

        get_instruction_registry()
            .contains_key(instruction.as_str())
            .then_some(instruction)
    }

    // -------------------------------
    // 🧪 Scroll Sentence Grammar Validator
    // -------------------------------

    /// 🧪 Validates if a scroll sentence aligns with basic grammar expectations.
    ///
    /// This is a lightweight SVO form validator:
    /// - Ensures non-empty subject and verb
    /// - Allows optional object if non-empty
    ///
    /// 📌 Called during scroll parsing for soft enforcement.
    /// 📊 Debug logs SVO structure.
    ///
    /// 🛠️ Future: Add schema-matching, verb role checking, and preposition handling.
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn is_valid_sentence(&self, subject: &str, verb: &str, object: Option<&str>) -> bool {
        let has_subject = !subject.trim().is_empty();
        let has_verb = !verb.trim().is_empty();
        let has_valid_object = object.map(|o| !o.trim().is_empty()).unwrap_or(true);

        #[cfg(feature = "debug_mode")]
        {
            use crate::debugger::{DebugEntry, Severity};
            let actual = format!("s='{}', v='{}', o='{:?}'", subject, verb, object);
            let entry = DebugEntry::new(
                "is_valid_sentence",
                verb,
                "Non-empty subject and verb",
                &actual,
            )
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
//   - This section defines post-parse behavior for `ScrollTree`,
//     including `.stone` serialization and grammar validation stubs.
//
// ⚙️ Engine Scope:
//   - Converts internal ScrollNode structures into readable `.stone` IR
//   - Prepares structure for spiritual grammar validation
//   - Interfaces softly with Operand Resolver and DebugEntry tracking
//
// ---------------------------------------------------
// 🚨 Version Control Notice:
// ---------------------------------------------------
//   This logic is part of the OmniCode Parser Scroll.
//   Any updates here must be reviewed for downstream effects.
//   ⚠️ Comments marked as contract-bound signal compiler/validator interfaces.
//
// ---------------------------------------------------
// 📅 Scroll Revision Metadata:
// ---------------------------------------------------
//   _version_:       v0.0.3
//   _last updated_:  2025-06-14
//   _author_:        Seanje Lenox-Wise / Nova Dawn
//   _change log_:
//     - Improved `.stone` serializer logic with operand awareness
//     - Replaced validation stub with semi-operational grammar hooks
//     - Integrated debug feedback for sentence and node output
//
// ---------------------------------------------------
// 🪜 Ladder Baton — Flow & Interface Direction:
// ---------------------------------------------------
//   ⬆️ Upstream:
//     - Receives fully parsed `ScrollTree` from Parser logic (incl. node walkers)
//     - Inherits operand-aligned nodes from operand resolution phase
//
//   ⬇️ Downstream:
//     - Feeds `.stone` output into Watchtower (diagnostic scrolls)
//     - Exports nodes into Assembler for IR transformation
//     - Hooks into `.logos` validator (future) for theology and grammar audits
//
//   🔁 Parallel:
//     - DebugEntry tracking feeds logs into OmniDebug channel
//     - Grammar feedback loop will interface with ErrorEmitter sublayer
//
// ---------------------------------------------------
// 🔮 Notes for Next Phase:
// ---------------------------------------------------
// - Extend `to_stone()` to handle nested indentation and metadata fields
// - Implement `.logos` validator and trust-based audit tagging
// - Mirror `.stone` and `.logos` divergence points for truth scoring
// - Start schema propagation through grammar roles and call assignments
//
// ---------------------------------------------------

// ===============================================
// 🧱 ScrollTree Output & Validation Methods
// ===============================================

impl ScrollTree {
    // -------------------------------
    // 🪨 Stone Format Serializer
    // -------------------------------

    /// 🔁 Converts `ScrollTree` into intermediate `.stone` format.
    ///
    /// Serializes all top-level nodes into `.stone`—a linear, readable
    /// intermediate representation for debugging, inspection, or transport.
    ///
    /// 🔮 Future upgrades:
    /// - Prettify block formatting
    /// - Add nested indentation
    /// - Integrate schema-aware emitters
    /// - Resolve operands using `.logos` or grammar walker
    pub fn to_stone(&self) -> String {
        let mut output = String::new();

        for node in &self.nodes {
            match node {
                // ✨ Basic instruction: verb and arguments flattened
                ScrollNode::Instruction { name, args } => {
                    output += &format!("{} {}\n", name, args.join(" "));
                    // 🔍 If operand resolver enriches args in future, update format here
                }

                // 📖 Scroll-style sentence: subject–verb–object grammar
                ScrollNode::ScrollSentence {
                    subject,
                    verb,
                    object,
                } => {
                    output += &format!("{} {} {}\n", subject, verb, object);
                    // 🧠 Could later enrich with operand role types or tags
                }

                // 🧷 Assignment: `x = value`
                ScrollNode::Assignment { target, value } => {
                    output += &format!("{} = {}\n", target, value);
                    // ⚙️ Operand-aware value? Ensure proper spacing or quotes if literal
                }

                // 🔢 Literal node: raw value capture
                ScrollNode::Literal(val) => {
                    output += &format!("literal {}\n", val);
                }

                // 🏷️ Metadata: for tags, titles, or attributes
                ScrollNode::Metadata(data) => {
                    output += &format!("meta {}\n", data);
                }

                // 🧱 Block: nested child nodes, displayed as internal lines
                ScrollNode::Block(inner) => {
                    output += "{\n";
                    for child in inner {
                        // 🚧 TEMP: Debug output — replace with `child.to_stone()` or similar
                        output += &format!("  {:?}\n", child);
                    }
                    output += "}\n";
                }

                // 🚨 Error display
                ScrollNode::Error(err) => {
                    output += &format!("!error {}\n", err);
                }

                // 📝 Declaration: `let name: Type`
                ScrollNode::Declaration { name, dtype } => {
                    let dtype_display = dtype.clone().unwrap_or_else(|| "Unknown".into());
                    output += &format!("let {}: {}\n", name, dtype_display);
                }

                // 🔀 Conditional: just show condition inline
                ScrollNode::Conditional { condition, .. } => {
                    output += &format!("if {}\n", condition);
                    // 🌿 Future: emit body as well (nested blocks)
                }

                // 🔁 Loop: emit as `loop <cond>`
                ScrollNode::Loop { condition, .. } => {
                    output += &format!("loop {}\n", condition);
                    // 🌱 Similar: body emission later
                }

                // 📥 Import statements
                ScrollNode::Import(path) => {
                    output += &format!("import {}\n", path);
                }

                // 🔚 Return value — potentially operand-wrapped
                ScrollNode::Return(value) => {
                    output += &format!("return {}\n", value);
                    // 🧩 Future: value may come from operand tree
                }

                // 📞 Function call
                ScrollNode::Call { function, args } => {
                    // 💡 Function call emits like: `func(arg1, arg2)`
                    output += &format!("{}({})\n", function, args.join(", "));
                    // 🧠 Operand resolver may later format args differently
                }

                // 💬 Comments in scroll
                ScrollNode::Comment(text) => {
                    output += &format!("// {}\n", text);
                }
            }
        }

        output
    }

    // -------------------------------
    // 📖 Scroll Validation (.logos-Aligned)
    // -------------------------------

    /// 📖 Validates the `ScrollTree` against .logos grammar and Scripture alignment.
    ///
    /// Early validation logic now includes:
    /// - Subject–Verb–Object sentence checks
    /// - Instruction name registry checks
    /// - Return statement validity
    ///
    /// 🛐 Future integration:
    /// - Full `.logos` spiritual schema
    /// - Verse-backed alignment walkers
    /// - Drift diagnostics and audit score
    #[cfg_attr(not(any(test, feature = "debug_mode")), allow(dead_code))]
    pub fn validate_with_scripture(&self) -> bool {
        use crate::parser::Parser;

        // 📜 Create a temporary parser instance for access to instruction registry and validators
        let validator = Parser::new(vec![]); // 🧪 Only used to call helper functions

        for node in &self.nodes {
            match node {
                // 🔍 Validate subject–verb–object structure
                ScrollNode::ScrollSentence {
                    subject,
                    verb,
                    object,
                } => {
                    let is_valid = validator.is_valid_sentence(subject, verb, Some(object));
                    if !is_valid {
                        #[cfg(feature = "debug_mode")]
                        {
                            use crate::debugger::{DebugEntry, Severity};
                            let entry = DebugEntry::new(
                                "validate_with_scripture",
                                &format!("{} {} {}", subject, verb, object),
                                "Valid SVO sentence",
                                "Failed validation",
                            )
                            .with_location("ScrollTree::validate_with_scripture")
                            .with_severity(Severity::Warning)
                            .with_suggestion("Review sentence structure or verb roles");
                            println!("{entry:#?}");
                        }
                        return false; // 🚨 Fatal alignment failure
                    }
                }

                // 🔍 Validate instruction name against registry
                ScrollNode::Instruction { name, .. } => {
                    if validator
                        .decode_instruction(&Token::from_value(name))
                        .is_none()
                    {
                        #[cfg(feature = "debug_mode")]
                        {
                            use crate::debugger::{DebugEntry, Severity};
                            let entry = DebugEntry::new(
                                "validate_with_scripture",
                                name,
                                "Known instruction",
                                "Unknown instruction",
                            )
                            .with_location("ScrollTree::validate_with_scripture")
                            .with_severity(Severity::Warning)
                            .with_suggestion("Verify instruction name is part of the registry");
                            println!("{entry:#?}");
                        }
                        return false; // 🚨 Invalid instruction
                    }
                }

                // ⚠️ Return with empty or suspicious value
                ScrollNode::Return(value) => {
                    if value.trim().is_empty() || value == "None" {
                        #[cfg(feature = "debug_mode")]
                        {
                            use crate::debugger::{DebugEntry, Severity};
                            let entry = DebugEntry::new(
                                "validate_with_scripture",
                                value,
                                "Non-empty return",
                                "Empty or invalid return value",
                            )
                            .with_location("ScrollTree::validate_with_scripture")
                            .with_severity(Severity::Warning)
                            .with_suggestion(
                                "Ensure return carries actual meaning or operand value",
                            );
                            println!("{entry:#?}");
                        }
                        return false;
                    }
                }

                _ => {
                    // ✨ Other node types are considered valid by default
                    // May be enriched in future .logos validations
                }
            }
        }

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
            .with_suggestion("Integrate .logos validator hooks");
            println!("{entry:#?}");
        }

        true // ✅ Passed all checks
    }
}
