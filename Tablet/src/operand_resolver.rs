// ===============================================
// 📜 Metadata - Bearer v0.0.1 (Tablet Operand Resolver)
// ===============================================
// _author_:        Seanje Lenox-Wise / Nova Dawn
// _version_:       0.0.1
// _status_:        Dev
// _created_:       2025-06-11
// _last updated_:  2025-06-11
// _license_:       CreativeWorkzStudio LLC — Kingdom-First Proprietary Use
// _component_:     Bearer (Operand Resolver for Tablet)
// _project_:       OmniCode / Millennium OS
// _description_:   Resolves operand structures from parsed scroll nodes into structured Assembler-ready forms.
//
// _notes_:
// - Interfaces between Scroll Tree (Parser) and Instruction execution (Assembler phase)
// - Validates operand structure against instruction schema
// - Resolves values, symbols, literals, and bindings to Operand enum variants
// - This is where meaning is carried—before code executes
// - Future support: nested operand resolution, spiritual posture validation, and Watchtower alerts
// ===============================================

// ===============================================
// 🌀 Opening — Imports & Declarations
// ===============================================
// This section declares all dependencies used in the Bearer module.
// These include token types, instruction schemas, parser nodes, and debugging hooks.
// The Bearer serves as the Tablet’s operand resolver—it prepares the body for execution
// and ensures meaning is correctly structured before being handed off to the Assembler.

// === Standard Library Imports ===

use std::collections::HashMap; // 📦 Maps symbolic bindings to resolved operands and confidence tiers
use std::fmt; // 🧾 Enables custom debug output for operand display

// Optionally required for advanced memory or metadata linking across scrolls
use std::rc::Rc; // 🔗 Shared ownership across single-threaded components
// use std::sync::Arc; // 🔗 Shared ownership in multithreaded context (Uncomment if Watchtower multithreads)
// use std::cell::RefCell; // 🧬 Interior mutability for components like scroll-tree node walkers
// use std::sync::Mutex; // 🔐 Needed if multithreaded mutable access becomes required

// === Internal Module Imports ===

use crate::tokenizer::{Token, TokenType};
// 🪙 Tokens are the smallest language units — used during literal extraction or pattern matching

use crate::instruction_registry::{Instruction, OperandSchema};
// 📚 Instruction structures and operand expectations — schema validation and resolution targets

use crate::parser::{ScrollNode, ScrollTree};
// 📜 Nodes and scroll tree — represent parsed sentences and operand containers

use watchtower::debugger::{DebugEntry, DebugResponse, Severity};
// 🪛 Debug events, trace severity, and feedback scaffolding — emitted to Watchtower for trace logging

// === Optional Future Imports ===
// These are proactively included for future operand resolution and trust linking extensions.

// use crate::instruction_registry::InstructionStatus; 
// 📍 Uncomment once operand resolution result mutates Instruction status directly

// use crate::tokenizer::TokenSpan;
// 🧭 Could track source origin for scroll-to-symbol traceability or backtracing

// ===============================================
// 📦 Foundational Declarations — Core Structures
// ===============================================
// This section defines the Operand enum and Bearer container.
// No resolution logic is defined here—only the structures responsible
// for carrying and describing operand types before execution.
//
// Operand is the core resolved unit. The Bearer walks scroll nodes,
// resolves them against instruction expectations, and emits Operand sets.

// ===============================================
// 🔣 Operand — Bearer of Instruction Inputs
// ===============================================
// This enum defines all valid operand forms that can be carried
// into the execution and resolution layers of OmniCode.
// It reflects Dev Log 7’s vision: operands are not mere values,
// but contextual carriers of symbolic, literal, and scroll-level meaning.

/// 🔣 Operand — Bearer of Instruction Inputs  
/// Represents every valid operand form the Bearer may resolve.  
/// Operands are symbolic containers of meaning—not just values.
/// See Dev Log 7 for philosophical and structural context.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// 🧾 A raw literal value such as a number, string, or boolean.
    /// This is the simplest form of operand—it carries immediate meaning without context.
    /// Example: `5`, `"truth"`, `true`
    Literal {
        value: String,
        dtype: Option<OperandType>, // 🧬 A type hint—Integer, String, etc.—if available
    },

    /// 🪶 A symbolic name—referring to a variable, alias, or declared identity in the scroll.
    /// Carries spiritual or contextual meaning, not just technical reference.
    /// Example: `faith`, `covenant`, `localPath`
    Binding {
        name: String,
        alignment: Option<BindingScope>, // 🧭 The level this name lives in: Local, Global, Sacred
    },

    /// 🔁 A group of operands used together—such as arguments to a function or elements of a list.
    /// Treated as a unit, but each operand within it has its own nature.
    /// Example: `(x, y, 5)`
    Group(Vec<Operand>),

    /// 🔧 An instruction that is *called* as part of this operand.
    /// This is not a reference—it is an active sub-expression inside another instruction.
    /// Example: `resolve(x + y)`
    InstructionCall {
        name: String,             // 🏷️ Name of the instruction being invoked
        args: Vec<Operand>,       // 🧩 The operands being passed into that call
    },

    /// 🔍 A passive reference to an instruction by name.
    /// It is not invoked—it just *points to* something known or stored.
    /// Example: `InstructionRef("load_balance")`
    InstructionRef(String),

    /// 🛤 A path through symbolic scopes or fields—used for dotted expressions like `user.name`.
    /// Helps the Bearer know how deep to go when chasing meaning.
    /// Example: `PathAccess(["root", "credentials", "token"])`
    PathAccess {
        path: Vec<String>,
    },

    /// 🔐 A value that has already been evaluated—used when folding has occurred.
    /// This carries no dynamic logic, just a final form.
    /// Example: `ResolvedValue("true")` after processing `1 == 1`
    ResolvedValue(String),

    /// ⛅ A placeholder—a symbolic gap waiting to be filled or ignored.
    /// Used in templates, patterns, or scrolls with missing parts.
    /// Example: `_`, `<insert_here>`, or `{{user_name}}`
    Placeholder(String),

    /// 🌀 A wildcard—an "accept anything" operand used when pattern-matching or abstracting.
    /// It tells the Bearer: “You don’t need to care what this is—just keep going.”
    Wildcard,

    /// ❌ An operand that failed to resolve—caught instead of ignored.
    /// Stored for debugging and Watchtower clarity.
    /// Example: a broken token like `"123abc"` in a numeric field.
    InvalidOperand(String),
}

/// ===============================================
/// 📘 OperandType — Resolved Data Classification
/// ===============================================
/// Represents the inferred or declared type of an operand.
// Used throughout Bearer logic for resolution, validation,
// and assembler interpretation.
//
// Though lighter than the full Operand enum, OperandType
// determines the spiritual and technical identity of the
// value or symbol being passed forward.
//
// Types here align with scroll semantics and type-aware
// assembler decisions (e.g., Integer vs Symbol vs Scroll).
//
// More specialized or nested types (e.g., Path, RefType)
// are handled at the Operand level, not here.

/// 📘 OperandType — Declared or inferred operand data type  
/// Guides resolution semantics and operand classification.  
/// Used by Bearer, Validator, and Watchtower to interpret meaning.  
/// This enum is lean by design—but foundational in execution flow.
/// Expanded to support operand variants and system feedback.
#[derive(Debug, Clone, PartialEq)]
pub enum OperandType {
    Integer,     // 🔢 Whole number value
    Float,       // 🌊 Decimal number value
    Boolean,     // 🚦 true / false
    String,      // 🔤 Quoted textual data
    Symbol,      // 🪶 Binding or variable name
    Instruction, // 🛠 Instruction call or scroll-level operand
    Scroll,      // 📜 Inline or referenced scroll
    Path,        // 🛤 Scoped reference (e.g., module::item)
    Wildcard,    // 🌀 Accepts any operand type
    Placeholder, // ⛅ Awaiting deferred typing
    PreFolded,   // ⚙️ Already resolved literal
    Unknown,     // ❓ Not yet classified or inferred
}

// ===============================================
// 🧭 BindingScope — Posture or Alignment of a Symbolic Binding
// ===============================================
// This enum defines the spiritual or contextual posture of a bound symbol.
// A binding’s scope affects resolution visibility, override behavior, and
// assembler readiness—mirroring its covenantal, local, or external nature.
//
// These scopes are intentionally designed to allow for spiritual clarity,
// closure capture, and multi-scroll interoperability without forcing logic
// until needed. They preserve the semantics laid out in Dev Log 7.
//
// This is **not** a replacement for `TrustTier`, but a complementary axis
// used to qualify *where* a binding originates or resides.

/// 🧭 BindingScope — Posture or alignment of a symbolic binding  
/// Optional for now, but enables future scope-aware operand interpretation.  
/// Will inform assembler constraints, binding visibility, and override protection.
#[derive(Debug, Clone, PartialEq)]
pub enum BindingScope {
    /// 🧍 Defined in the current scroll, block, or local scope.
    Local,

    /// 🌍 Available across scroll boundaries—may be public or namespaced.
    Global,

    /// 🔥 Spiritually sealed or protected—cannot be overridden or redefined.
    Sacred,

    /// 📥 Captured from a parent scope (e.g., lifted into a closure).
    Captured,

    /// 🛠 Injected or reserved by the OmniCode runtime engine.
    Runtime,

    /// 🛰 Defined in an external scroll, library, or imported context.
    Extern,
}

// ===============================================
// 🔐 TrustTier — Operand Resolution Confidence Tier
// ===============================================
// This enum encodes the spiritual clarity, interpretive confidence,
// and fallback grade of a resolved operand.
//
// While `OperandType` tells you *what* the operand is,
// and `BindingScope` tells you *where* it's from,
// `TrustTier` tells you *how sure we are* about its identity.
//
// It is primarily used for Watchtower diagnostics, resolution triage,
// and to help inform phase-specific assembler logic.
//
// These tiers will eventually affect trace detail, assembler response,
// and whether the operand may require re-resolution or fallback logic.


/// 🔐 TrustTier — Operand Resolution Confidence (5-Tier Scale)
/// 🔐 `TrustTier` represents the confidence level assigned to a resolved operand.
/// It captures the spiritual clarity, semantic certainty, or interpretive ambiguity
/// of the operand as understood by the Bearer during resolution.
///
/// These tiers influence:
/// - Whether an operand is accepted for assembly
/// - Whether Watchtower raises trace alerts
/// - Whether re-resolution is triggered in multi-pass flows
///
/// Tiers will eventually interface with debugging alignment (0–100)
/// and may cascade into instruction-wide confidence metrics.
#[derive(Debug, Clone, PartialEq)]
pub enum TrustTier {
    /// 🟢 Tier 4 — Fully confident and schema-aligned
    Certain,        // Score: 100

    /// 🟢 Tier 3 — Valid by context but inferred, not declared
    Trusted,        // Score: ~75

    /// 🟡 Tier 2 — Multiple meanings possible; clarity lacking
    Ambiguous,      // Score: ~50

    /// 🔴 Tier 1 — Fallback or guessed meaning; unstable
    Shadowed,       // Score: ~25

    /// 🔴 Tier 0 — Rejected meaning; structurally invalid
    Invalid,        // Score: 0
}

// ===============================================
// 🧾 OperandMetadata — Scroll Provenance & Origin
// ===============================================
// This structure carries trace-level metadata for each Operand,
// enabling deep debugging, scroll tracking, and symbolic alignment.
// 
// While optional per operand, this data is invaluable for Watchtower,
// log overlays, schema reconciliation, and spiritual auditability.
//
// This struct may be embedded into relevant Operand variants in the future
// via an Option or wrapped context layer, depending on scroll propagation logic.

/// 📎 Optional metadata for debugging, traceability, or scroll alignment
#[derive(Debug, Clone, PartialEq)]
pub struct OperandMetadata {
    /// 🪪 The originating scroll or document name
    pub source_scroll: Option<String>,

    /// 🔢 Line number in the original scroll (if known)
    pub line_number: Option<usize>,

    /// 🧭 A unique trace ID or symbolic tag for provenance
    pub origin_trace: Option<String>,

    /// 📛 Human-readable name or alias of the operand (used for logs and trace trees)
    pub display_name: Option<String>,

    /// 🕊️ Trust tier snapshot at time of construction
    pub trust_tier: Option<TrustTier>,

    /// 🧰 Future extensibility map for additional metadata tags
    pub tags: Option<HashMap<String, String>>,
}

// ===============================================
// 🧱 Struct Definition — Operand Bearer (Tablet Cog)
// ===============================================
// The Bearer holds all active context for resolving operands during scroll interpretation.
// It acts as the bridge between parsed sentence tokens and validated operand form,
// carrying the instruction schema, alignment feedback, and resolution output.
//
// This structure persists between scroll passes and serves as the cog
// responsible for operand clarity, resolution tracking, and instruction alignment.

/// 🧱 `Bearer` — Central structure for operand resolution  
/// The Bearer manages parsed scroll data, current instruction context, operand outputs,  
/// and Watchtower debug traces. It acts as the scroll walker for each instruction clause.  
/// Every operand resolved passes through this structure, receiving classification,  
/// confidence evaluation, and debug-tier feedback.
#[derive(Debug)]
pub struct Bearer {
    /// 📚 Instruction registry reference — for schema lookup and instruction arity rules
    pub instruction_registry: InstructionRegistry,

    /// 🧩 Token stream from the scroll being interpreted
    pub tokens: Vec<Token>,

    /// 📛 Current instruction name being processed (if applicable)
    pub current_instruction: Option<String>,

    /// 🪙 Final resolved operands — output of the Bearer resolution pass
    pub resolved_operands: Vec<Operand>,

    /// 🪛 Trace log entries captured during resolution
    pub debug_trace: Vec<DebugEntry>,

    // 🆕 From skeleton expansion:

    /// 🌳 Full parsed scroll tree passed into the Bearer for node walking
    pub scroll_tree: Option<ScrollTree>,

    /// 🧱 Current scroll node being analyzed
    pub current_node: Option<ScrollNode>,

    /// 📜 Instruction schema used for operand validation and trust tier evaluation
    pub instruction_schema: Option<OperandSchema>,

    /// 🧷 Local operand bindings by symbolic name (used in context tracking)
    pub operand_bindings: HashMap<String, Operand>,

    /// 🛡️ Per-symbol trust tier flag — helps Watchtower understand alignment confidence
    pub trust_flags: HashMap<String, TrustTier>,

    /// ❗ Collection of resolution issues that require developer attention
    pub errors: Vec<DebugEntry>,

    pub context_id: Option<String>, // 🧭 Symbolic tag for nested operand contexts (e.g., scroll phase, scope)
    
    // 🔌 Runtime trace connection — not wired yet, but anticipated in design.
    // Will allow Bearer to emit live updates directly to Watchtower if hook is provided.
    pub watchtower_hook: Option<fn(DebugEntry) -> DebugResponse>,
}

// ===============================================
// 🛠 Constructors & Initializers — Bearer of Operands
// ===============================================
// This section prepares the Bearer for resolving operand intent.
// It initializes internal state, optional caches, and configures
// the groundwork for walking, resolving, and dispatching operand logic.
//
// The Bearer does not execute resolution here—it only becomes ready to.
//
// Future: Support injected Watchtower debugger or instruction context.
// ===============================================

impl Bearer {
    /// 🔨 Constructs a new Bearer instance.
    /// This prepares the resolver with fresh state and optional configuration scaffolding.
    pub fn new() -> Self {
        Self {
            instruction_registry: InstructionRegistry::default(),
            tokens: Vec::new(),
            current_instruction: None,
            resolved_operands: Vec::new(),
            debug_trace: Vec::new(),

            scroll_tree: None,
            current_node: None,
            instruction_schema: None,
            operand_bindings: HashMap::new(),
            trust_flags: HashMap::new(),
            errors: Vec::new(),
            context_id: None,
            watchtower_hook: None,
        }
    }

    /// 🪪 Identifies the component as the Operand Resolver.
    /// Useful for debug, scaffolding, or internal CLI description.
    pub fn identity() -> &'static str {
        "Bearer (Operand Resolver)"
    }
}

// ===============================================
// 🛠️ Body — Operand Resolution System (Bearer)
// ===============================================
// This body houses the Bearer's operand resolution pipeline,
// translating parsed instructions into structured operand forms.
// 
// Dev Log 7 identifies this role as:
// "The Priest of Operand Meaning" — tasked with discerning,
// validating, and preparing operands for assembler readiness.
//
// Each phase is scaffolded and wired to match the mental flow
// of execution while awaiting deeper logic integration.

impl Bearer {
    /// 🎯 `resolve_operands` — Primary entry point for operand resolution  
    /// Orchestrates all phases of scroll operand discernment.  
    /// Operands move from raw sentence form to validated `Operand` structures  
    /// through classification, construction, and debug relay steps.  
    /// This is where the Bearer earns its title as the **Priest of Operand Meaning**.
    /// 🎯 Entry point for operand resolution.
    /// Receives a mutable instruction and orchestrates all phases
    /// of operand interpretation and readiness marking.
    ///
    /// This function aligns with Dev Log 7’s principle that all operands
    /// must pass through clarity, construction, and confidence before
    /// being released to the assembler.
    pub fn resolve_operands(instruction: &mut Instruction) {
        // ===============================================
        // 🪧 Phase 1 — Operand Field Extraction
        // ===============================================
        // Gathers subject, verb, and object for pattern-based classification.
        let (subject, verb, object) = Self::extract_fields(instruction);

        // ➕ Phase 1A — Structural Validation (scaffolded)
        if subject.is_empty() || verb.is_empty() || object.is_empty() {
            instruction.status = InstructionStatus::Invalid;
            return; // Cannot resolve if key fields are missing.
        }

        // ===============================================
        // 🧠 Phase 2 — Pattern Recognition & Classification
        // ===============================================
        // Determines operand type based on subject/verb/object symbolic mapping.
        let operand_type = Self::classify_pattern(&subject, &verb, &object);

        // ➕ Phase 2A — Verb Taxonomy Matching (scaffold)
        let _verb_role_hint = match verb.to_lowercase().as_str() {
            "let" | "set" | "define" => Some("Assignment"),
            "return" | "yield" => Some("Control"),
            "push" | "append" => Some("Mutation"),
            _ => None,
        };

        // ➕ Phase 2B — AI-Based Deduction
        if matches!(operand_type, OperandType::Unknown) {
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: format!("Unrecognized operand form — flagged for AI-based deduction."),
                severity: Severity::Drifted,
            });
        }

        // ===============================================
        // 🧱 Phase 3 — Operand Construction
        // ===============================================
        // Builds concrete Operand structure from pattern insight and object value.
        let operand = Self::build_operand(&object, operand_type.clone());

        // ➕ Phase 3A — Operand Refinement (partial logic)
        // For now, just a basic trust tag and binding fallback if unknown.
        let trust_tier = match operand_type {
            OperandType::Unknown => TrustTier::Shadowed,
            OperandType::Symbol => TrustTier::Trusted,
            OperandType::Literal => TrustTier::Certain,
            _ => TrustTier::Ambiguous,
        };

        // Update bindings and trust flags
        if let Operand::Binding { name, .. } = &operand {
            instruction.operand_bindings.insert(name.clone(), operand.clone());
            instruction.trust_flags.insert(name.clone(), trust_tier);
        }

        // ===============================================
        // 🎯 Phase 4 — Instruction State Update
        // ===============================================
        // Updates instruction readiness for assembler or re-resolution.
        let is_resolved = !matches!(operand_type, OperandType::Unknown);
        Self::update_instruction_state(instruction, is_resolved);

        // ===============================================
        // 🛡 Phase 5 — Debug Trace to Watchtower
        // ===============================================
        // Emits trace status from resolution to Watchtower or logs.
        Self::emit_watchtower_log(instruction);

        // ➕ Phase 5A — TrustTier Cascade (scaffolded)
        let mut highest_tier = TrustTier:: Certain;
        for tier in instruction.trust_flags.values() {
            if tier < &highest_tier {
                highest_tier = tier.clone();
            }
        }
        instruction.trust_summary = Some(highest_tier); // Must exist in struct

        // ===============================================
        // 🌿 Phase 6 — Reconciliation & Operand Rewalk (future)
        // ===============================================
        if instruction.resolved_operands.iter().any(|op| matches!(op, Operand::Placeholder(_))) {
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "Instruction contains placeholders — rewalk may be required.".to_string(),
                severity: Severity::Shadowed,
            });
        }

        // ===============================================
        // 📎 Phase 7 — Operand Metadata Tagging (future)
        // ===============================================
        let meta_note = format!("Origin line: {}", instruction.line);
        instruction.metadata_tags.insert("operand_origin".to_string(), meta_note);

        // ===============================================
        // 🪞 Phase 8 — MetaOperand & Reflective Operand Support (future)
        // ===============================================
        if matches!(operand, Operand::Wildcard | Operand::InstructionRef(_) | Operand::Placeholder(_)) {
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "MetaOperand or reflective operand form detected.".to_string(),
                severity: Severity::Valid,
            });
        }

        // Finally, push the resolved operand (for now, only one) into instruction context
        instruction.resolved_operands.push(operand);
    }

    // ===============================================
    // 🧩 Phase 1 — Field Extraction Logic
    // ===============================================
    /// Extract operand-relevant fields from an instruction.
    fn extract_fields(instruction: &Instruction) -> (String, String, String) {
        let subject = instruction.subject.clone();
        let verb = instruction.verb.clone();
        let object = instruction.object.clone();
        (subject, verb, object)
    }

    // ➕ Phase 1A — Structural Validation (scaffolded inside resolve_operands)
    // No separate method needed—basic field checks are inline for now.

    // ===============================================
    // 🔍 Phase 2 — Pattern Recognition Logic
    // ===============================================
    /// Classify operand type from subject-verb-object form.
    fn classify_pattern(subject: &str, verb: &str, object: &str) -> OperandType {
        match (subject, verb, object) {
            ("Let", _, _) => OperandType::Symbol,
            ("Set", _, _) => OperandType::Symbol,
            ("Return", _, _) => OperandType::Unknown,
            _ => OperandType::Unknown,
        }
    }

    // ➕ Phase 2A — Verb Taxonomy Matching
    fn match_verb_taxonomy(verb: &str) -> Option<&'static str> {
        match verb.to_lowercase().as_str() {
            "let" | "set" | "define" => Some("Assignment"),
            "return" | "yield" => Some("Control"),
            "push" | "append" => Some("Mutation"),
            _ => None,
        }
    }

    // ➕ Phase 2B — AI-Based Deduction (scaffolded)
    fn flag_for_ai_deduction(instruction: &mut Instruction) {
        instruction.debug_trace.push(DebugEntry {
            line: instruction.line,
            message: "Unrecognized operand form — flagged for AI-based deduction.".to_string(),
            severity: Severity::Drifted,
        });
    }

    // ===============================================
    // 🧱 Phase 3 — Operand Construction Logic
    // ===============================================
    fn build_operand(object: &str, operand_type: OperandType) -> Operand {
        match operand_type {
            OperandType::Symbol => Operand::Binding {
                name: object.to_string(),
                alignment: None,
            },
            OperandType::Literal => Operand::Literal {
                value: object.to_string(),
                dtype: None,
            },
            _ => Operand::InvalidOperand(object.to_string()),
        }
    }

    // ➕ Phase 3A — Operand Refinement
    fn refine_operand(
        instruction: &mut Instruction,
        operand: &Operand,
        operand_type: &OperandType,
    ) -> TrustTier {
        let trust = match operand_type {
            OperandType::Unknown => TrustTier::Shadowed,
            OperandType::Symbol => TrustTier::Trusted,
            OperandType::Literal => TrustTier::Certain,
            _ => TrustTier::Ambiguous,
        };

        if let Operand::Binding { name, .. } = operand {
            instruction.operand_bindings.insert(name.clone(), operand.clone());
            instruction.trust_flags.insert(name.clone(), trust.clone());
        }

        trust
    }

    // ===============================================
    // 🛠 Phase 4 — Instruction State Resolution Logic
    // ===============================================
    fn update_instruction_state(instruction: &mut Instruction, resolved: bool) {
        if resolved {
            instruction.status = InstructionStatus::ReadyToAssemble;
        } else {
            instruction.status = InstructionStatus::RequiresResolution;
        }
    }

    // ===============================================
    // 📡 Phase 5 — Debug Emission to Watchtower
    // ===============================================
    fn emit_watchtower_log(instruction: &Instruction) {
        let log = DebugEntry {
            line: instruction.line,
            message: format!(
                "Bearer resolution status: {:?}",
                instruction.status
            ),
            severity: Severity::Valid,
        };

        println!("{:?}", log);
    }

    // ➕ Phase 5A — TrustTier Cascade
    fn cascade_trust_summary(instruction: &mut Instruction) {
        let mut highest = TrustTier::Certain;
        for tier in instruction.trust_flags.values() {
            if tier < &highest {
                highest = tier.clone();
            }
        }
        instruction.trust_summary = Some(highest);
    }

    // ===============================================
    // 🌿 Phase 6 — Reconciliation & Operand Rewalk
    // ===============================================
    fn check_for_rewalk(instruction: &mut Instruction) {
        if instruction
            .resolved_operands
            .iter()
            .any(|op| matches!(op, Operand::Placeholder(_)))
        {
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "Instruction contains placeholders — rewalk may be required.".to_string(),
                severity: Severity::Shadowed,
            });
        }
    }

    // ===============================================
    // 📎 Phase 7 — Operand Metadata Tagging
    // ===============================================
    fn tag_operand_metadata(instruction: &mut Instruction) {
        let note = format!("Origin line: {}", instruction.line);
        instruction
            .metadata_tags
            .insert("operand_origin".to_string(), note);
    }

    // ===============================================
    // 🪞 Phase 8 — MetaOperand & Reflective Operand Support
    // ===============================================
    fn handle_meta_operand(instruction: &mut Instruction, operand: &Operand) {
        if matches!(
            operand,
            Operand::Wildcard | Operand::InstructionRef(_) | Operand::Placeholder(_)
        ) {
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "MetaOperand or reflective operand form detected.".to_string(),
                severity: Severity::Valid,
            });
        }
    }
}

// ===================================================
// 🔚 Closing Block — Post-Resolution Hooks & Outlook
// ===================================================
//
// 🧾 Overview:
//   - This section defines post-resolution behaviors for operand handling,
//     including final validation, status echoing, and debug projection.
//
// ⚙️ Engine Scope:
//   - Confirms resolution validity for each operand
//   - Prepares the resolved instruction for assembler intake
//   - (Eventually) emits detailed traces to the Watchtower for transparency
//
// ---------------------------------------------------
// 🚨 Version Control Notice:
// ---------------------------------------------------
//   This logic is part of the Operand Bearer scroll.
//   Any updates here must preserve operand signature compatibility.
//   Comments marked ⚠️ indicate assembler-bound interface expectations.
//
// ---------------------------------------------------
// 📅 Last Updated:
// ---------------------------------------------------
//   Version       : v0.0.1
//   Last Updated  : 2025-06-11
//   Change Log    : Initial post-logic skeleton and future hook layout
//
// ---------------------------------------------------
// 🔮 Notes for Next Phase:
// ---------------------------------------------------
// - Add direct hooks for operand trust levels (temporary, shadowed, sealed)
// - Integrate operand tracing into Watchtower debug overlays
// - Prepare resolution snapshots for `.logos` and `.stone` flows
// - Bearer may need to hold a weak reference to the instruction_registry
// - Instruct Watchtower to react differently based on resolution tier
//
// ---------------------------------------------------

impl Bearer {
    /// ✅ Final confirmation that all operand fields have been classified and constructed.
    ///
    /// This method walks the operands assigned to an instruction and
    /// checks if all have been resolved to valid types. This is a post-pass
    /// that assumes resolution logic has been attempted prior.
    ///
    /// Returns true if all operands are valid.
    pub fn validate_operands(instruction: &Instruction) -> bool {
        // TODO: Once Instruction contains operands, inspect each for Operand::Unresolved
        // If any unresolved remains, return false; otherwise, return true
        true // 🕊 Temporary grace
    }

    /// 🛰 Emit debug snapshot to the Watchtower after operand resolution.
    ///
    /// This will eventually create a full diagnostic payload per operand,
    /// including origin line, symbol status, and spiritual clarity index.
    /// Designed to trace each Bearer action across phases.
    pub fn report_to_watchtower(instruction: &Instruction) {
        // TODO: Integrate with Watchtower::log once available
        #[cfg(feature = "debug_mode")]
        {
            println!(
                "📡 [Bearer] Instruction resolved: {:?}",
                instruction.status
            );
        }
    }

    /// 🧾 Optional serializer for logging or assembly review
    ///
    /// Converts the resolved operand set into a textual or symbolic form
    /// that can be stored, reviewed, or passed along scroll chains.
    pub fn export_operand_signature(instruction: &Instruction) -> String {
        // TODO: Once operand list is available, format each one with type + value
        "operand signature: [TODO]".to_string()
    }

    /// 🌀 Begins operand resolution from the scroll tree root.
    pub fn begin_resolution(&mut self, scroll_tree: ScrollTree) {
        self.scroll_tree = Some(scroll_tree);
        // TODO: Walk tree and begin operand discovery
    }

    /// 📚 Loads the operand schema for a specific instruction.
    pub fn load_instruction_schema(&mut self, instruction: &Instruction) {
        self.instruction_schema = self.instruction_registry.get_schema(&instruction.name);
        // TODO: Validate existence and arity
    }

    /// 🌿 Walks the scroll tree and processes operand nodes.
    pub fn walk_scroll_tree(&mut self) {
        // TODO: Traverse and trigger classification
    }

    /// 🪞 Validates operand count against expected arity.
    pub fn validate_arity(&self, node: &ScrollNode, schema: &OperandSchema) -> bool {
        // TODO: Return true if arity matches
        true
    }

    /// 🪶 Determines the operand type based on the node.
    pub fn classify_operand_type(&self, node: &ScrollNode) -> OperandType {
        // TODO: Inspect token, pattern, structure
        OperandType::Unknown
    }

    /// 🏗️ Constructs the operand from a scroll node and type.
    pub fn construct_operand(&self, node: &ScrollNode, operand_type: OperandType) -> Operand {
        // TODO: Build variant with placeholder value
        Operand::InvalidOperand("unresolved".to_string())
    }

    /// 🕊️ Assigns a trust tier to a resolved operand.
    pub fn mark_trust_level(&self, operand: &Operand) -> TrustTier {
        // TODO: Later factor in source, certainty, and alignment
        TrustTier::Shadowed
    }

    /// 🛡️ Records a debug trace entry.
    pub fn record_debug_entry(&mut self, entry: DebugEntry) {
        self.debug_trace.push(entry);
    }

    /// 📦 Finalizes all resolved operands for handoff.
    pub fn finalize_operands(&mut self) {
        // TODO: Commit operand set or emit errors
    }
}
