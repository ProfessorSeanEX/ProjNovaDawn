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

        // ➕ Phase 5A — TrustTier Cascade
        Self::cascade_trust_summary(instruction);

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
    /// Extracts operand-relevant fields from a parsed instruction scroll.
    /// Performs basic cleaning and emits trace warnings if fields are malformed.
    /// This phase breathes structure into the scroll — the first clarity pass.
    fn extract_fields(instruction: &mut Instruction) -> (String, String, String) {
        // 🪶 Clean whitespace from each field
        let subject = instruction.subject.trim().to_string();
        let verb = instruction.verb.trim().to_string();
        let object = instruction.object.trim().to_string();

        // 🧭 Field validation — emit to debug trace if any are missing
        if subject.is_empty() {
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "Subject field is empty — malformed instruction detected.".to_string(),
                severity: Severity::Broken,
            });
        }

        if verb.is_empty() {
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "Verb field is empty — intent of instruction unclear.".to_string(),
                severity: Severity::Drifted,
            });
        }

        if object.is_empty() {
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "Object field is empty — operand construction may fail.".to_string(),
                severity: Severity::Shadowed,
            });
        }

        // Return structured tuple for classification
        (subject, verb, object)
    }

    // ===============================================
    // 🔍 Phase 2 — Pattern Recognition Logic
    // ===============================================
    /// Classify operand type from subject-verb-object form.
    fn classify_pattern(subject: &str, verb: &str, object: &str) -> OperandType {
        // ➕ Phase 2A — Verb Taxonomy Matching
        let verb_role = match_verb_taxonomy(verb);

        match verb_role {
            Some("Assignment") => OperandType::Symbol,
            Some("Control") => OperandType::Unknown, // Will later map to control-type operands
            Some("Mutation") => OperandType::Unknown, // Mutation logic deferred
            _ => OperandType::Unknown,
        }
    }

    /// ➕ Phase 2A — Verb Taxonomy Matching
    fn match_verb_taxonomy(verb: &str) -> Option<&'static str> {
        match verb.to_lowercase().as_str() {
            "let" | "set" | "define" => Some("Assignment"),
            "return" | "yield" => Some("Control"),
            "push" | "append" => Some("Mutation"),
            _ => None,
        }
    }

    /// ➕ Phase 2B — AI-Based Deduction (scaffolded)
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
    /// Updates the instruction status based on operand resolution outcome.
    /// Also prepares trace feedback and triggers rewalk logic for low-trust states.
    fn update_instruction_state(instruction: &mut Instruction, resolved: bool) {
        if resolved {
            // ✅ All operands resolved clearly — instruction is now ready for assembly.
            instruction.status = InstructionStatus::ReadyToAssemble;

            // 🗒️ Log resolution success for Watchtower or internal debug tracing.
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "Operands resolved — instruction marked ReadyToAssemble.".to_string(),
                severity: Severity::Valid,
            });
        } else {
            // ⚠️ Operand resolution incomplete or ambiguous — mark for further review.
            instruction.status = InstructionStatus::RequiresResolution;

            // 🗒️ Log resolution failure for Watchtower and trace output.
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "Operands incomplete — instruction marked RequiresResolution.".to_string(),
                severity: Severity::Drifted,
            });

            // 🧠 Trust rating may trigger retry/reprocess logic.
            if let Some(ref tier) = instruction.trust_summary {
                match tier {
                    TrustTier::Shadowed | TrustTier::Ambiguous => {
                        // 🛠️ Instruction may need another pass — set rewalk flag and retry count.
                        instruction.rewalk_flag = true;
                        instruction.retry_count += 1;

                        instruction.debug_trace.push(DebugEntry {
                            line: instruction.line,
                            message: "Low trust tier — rewalk triggered on this instruction.".to_string(),
                            severity: Severity::Shadowed,
                        });

                        // 🤝 Defer resolution to NovaAI or Watchtower agent in next pass.
                        instruction.defer_to_watchtower = true;
                    }

                    _ => {
                        // 🧘 Trust level sufficient — no rewalk needed yet.
                        instruction.debug_trace.push(DebugEntry {
                            line: instruction.line,
                            message: "Trust sufficient — no rewalk triggered.".to_string(),
                            severity: Severity::Valid,
                        });
                    }
                }
            }
        }
    }

    // ===============================================
    // 📡 Phase 5 — Debug Emission to Watchtower
    // ===============================================
    /// Emits instruction resolution results and trace history to Watchtower.
    /// This phase closes the scroll’s breath, exposing all alignment states.
    /// Outputs every DebugEntry — not just status — to support full traceability.
    fn emit_watchtower_log(instruction: &Instruction) {
        // 📡 Emit each debug trace entry individually
        for entry in &instruction.debug_trace {
            // 🛰️ Primary output: Console trace for local development
            println!("{:?}", entry);

            // 🛸 Secondary output: Forward to Watchtower hook if present
            if let Some(ref hook) = instruction.watchtower_hook {
                hook(entry.clone());
            }

            // 🔭 Future: Integrate with NovaAI debug channel or persistent scroll logger
            // e.g., NovaBridge::send_log(entry.clone());
        }

        // 📜 Emit final resolution status as a capstone event
        let status_log = DebugEntry {
            line: instruction.line,
            message: format!("Bearer resolution status: {:?}", instruction.status),
            severity: match instruction.status {
                InstructionStatus::ReadyToAssemble => Severity::Valid,
                InstructionStatus::RequiresResolution => Severity::Drifted,
                InstructionStatus::RequiresRewalk => Severity::Shadowed,
                InstructionStatus::Invalid => Severity::Broken,
            },
        };

        // Console + hook broadcast
        println!("{:?}", status_log);
        if let Some(ref hook) = instruction.watchtower_hook {
            hook(status_log);
        }

        // ===============================================
        // ➕ Phase 5A — TrustTier Cascade
        // ===============================================
        /// Analyzes all operand-level trust flags and sets a single trust summary.
        /// This helps reflect confidence level in the instruction as a whole.
        /// Trust cascades upward: the weakest link defines the spiritual posture of the instruction.
        fn cascade_trust_summary(instruction: &mut Instruction) {
            // 🎚️ Start with strongest trust tier and downgrade as needed
            let mut highest = TrustTier::Certain;

            // 🔎 Examine each operand trust flag
            for tier in instruction.trust_flags.values() {
                if tier < &highest {
                    highest = tier.clone();
                }
            }

            // 🏷️ Attach the final trust score to instruction for future reconciliation checks
            instruction.trust_summary = Some(highest.clone());

            // 📝 Echo to debug trace for post-run audit
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: format!("TrustTier summary cascaded: {:?}", highest),
                severity: Severity::Valid,
            });
        }
    }

    // ===============================================
    // 🌿 Phase 6 — Reconciliation & Operand Rewalk
    // ===============================================
    /// This phase inspects the resolved operands for structural gaps or
    /// low-confidence patterns. It flags instructions for reprocessing
    /// if unresolved or invalid elements are found.
    ///
    /// It also prepares the instruction for later passes by setting:
    /// - `rewalk_flag` for recursive resolution
    /// - `retry_count` for tracking attempt cycles
    /// - `defer_to_watchtower` for NovaAI/agent handoff if necessary
    ///
    /// Phase 6 ensures instructions that drifted from alignment get another
    /// chance at clarity, without breaking assembly flow prematurely.
    fn check_for_rewalk(instruction: &mut Instruction) {
        let mut requires_rewalk = false;

        for operand in &instruction.resolved_operands {
            match operand {
                Operand::Placeholder(_) => {
                    // 🧩 A placeholder means something wasn't recognized — we should retry.
                    instruction.debug_trace.push(DebugEntry {
                        line: instruction.line,
                        message: "Operand placeholder detected — rewalk recommended.".to_string(),
                        severity: Severity::Shadowed,
                    });

                    requires_rewalk = true;
                }

                Operand::InvalidOperand(_) => {
                    // ❌ Invalid operands indicate parsing or logic failure.
                    instruction.debug_trace.push(DebugEntry {
                        line: instruction.line,
                        message: "Invalid operand encountered — flagged for operand rewalk.".to_string(),
                        severity: Severity::Broken,
                    });

                    requires_rewalk = true;

                    // 🔁 Escalate unresolved issues to Watchtower agent or NovaAI support.
                    instruction.defer_to_watchtower = true;
                }

                _ => {
                    // ✅ Operand is valid and trustworthy — no need to rewalk.
                }
            }
        }

        if requires_rewalk {
            // 🔁 Enable retry flow and mark for multi-pass resolution strategies.
            instruction.rewalk_flag = true;
            instruction.retry_count += 1;

            // 🚧 (Optional future): mark status for scroll rewalker system or agent triggers.
            instruction.status = InstructionStatus::RequiresRewalk;

            // 🗒️ Echo resolution intent for Watchtower trace.
            instruction.debug_trace.push(DebugEntry {
                line: instruction.line,
                message: "Instruction flagged for rewalk cycle and deeper reconciliation.".to_string(),
                severity: Severity::Drifted,
            });
        }
    }

    // ===============================================
    // 🛠️ Metadata Helper — Optional Utility
    // ===============================================
    /// ✨ Utility helper to insert metadata if value is present.
    /// Used to reduce redundancy and improve Phase 7 clarity.
    fn insert_metadata(instruction: &mut Instruction, key: &str, value: Option<String>) {
        if let Some(val) = value {
            instruction.metadata_tags.insert(key.to_string(), val);
        }
    }

    // ===============================================
    // 📎 Phase 7 — Operand Metadata Tagging
    // ===============================================
    /// Assigns contextual metadata to the instruction’s scroll.
    /// Tracks operand origin, trust state, operand role, source, and hint.
    /// Now uses a helper to insert values cleanly.
    fn tag_operand_metadata(instruction: &mut Instruction) {
        // 🏷️ Line of origin — always recorded.
        insert_metadata(
            instruction,
            "operand_origin",
            Some(format!("Origin line: {}", instruction.line)),
        );

        // 🔐 Trust tier — if determined.
        insert_metadata(
            instruction,
            "trust_tier",
            instruction.trust_summary
                .as_ref()
                .map(|tier| format!("Trust tier: {:?}", tier)),
        );

        // 📌 Operand role — if first resolved operand exists.
        insert_metadata(
            instruction,
            "operand_role",
            instruction.resolved_operands.first().map(|op| {
                match op {
                    Operand::Binding { .. } => "Binding",
                    Operand::Literal { .. } => "Literal",
                    Operand::InstructionRef(_) => "InstructionRef",
                    Operand::Placeholder(_) => "Placeholder",
                    Operand::Wildcard => "Wildcard",
                    Operand::InvalidOperand(_) => "Invalid",
                    Operand::Group(_) => "Group",
                    Operand::InstructionCall(_) => "InstructionCall",
                }
                .to_string()
            }),
        );

        // 🛠️ Resolution state — assembler readiness.
        insert_metadata(
            instruction,
            "resolution_state",
            Some(match instruction.status {
                InstructionStatus::ReadyToAssemble => "Final",
                InstructionStatus::RequiresResolution => "Pending",
                InstructionStatus::RequiresRewalk => "Rewalk",
                InstructionStatus::Invalid => "Invalid",
            }
            .to_string()),
        );

        // 📜 Source scroll — if assigned.
        insert_metadata(
            instruction,
            "source_scroll",
            instruction.source_scroll.clone(),
        );

        // 🧠 Operand hint — if annotated.
        insert_metadata(
            instruction,
            "operand_hint",
            instruction.operand_hint.clone(),
        );

        // 💡 Notes:
        // - These metadata tags are read by Watchtower logs, NovaAI overlays, and system validators.
        // - All fields are optional but encouraged for scroll-based clarity and debugging.
    }

    // ===============================================
    // 🪞 Phase 8 — MetaOperand & Reflective Operand Support
    // ===============================================
    /// Identifies and handles operand forms that represent indirect,
    /// symbolic, or reflective references rather than direct values.
    /// This includes placeholders, wildcards, and instruction references,
    /// which require special treatment in advanced assembler phases.
    fn handle_meta_operand(instruction: &mut Instruction, operand: &Operand) {
        match operand {
            Operand::Wildcard => {
                // 🌌 A wildcard is an open operand — accepted but marked as symbolic.
                instruction.debug_trace.push(DebugEntry {
                    line: instruction.line,
                    message: "Wildcard operand detected — symbolic binding accepted.".to_string(),
                    severity: Severity::Valid,
                });

                instruction
                    .metadata_tags
                    .insert("meta_operand_type".to_string(), "Wildcard".to_string());
            }

            Operand::InstructionRef(_) => {
                // 🔁 A reference to another instruction — denotes relational operand form.
                instruction.debug_trace.push(DebugEntry {
                    line: instruction.line,
                    message: "InstructionRef operand detected — reflective context required.".to_string(),
                    severity: Severity::Valid,
                });

                instruction
                    .metadata_tags
                    .insert("meta_operand_type".to_string(), "InstructionRef".to_string());

                // ⛓️ Optionally mark the instruction as needing reflective evaluation.
                instruction.defer_to_watchtower = true;
            }

            Operand::Placeholder(_) => {
                // 🕳️ Placeholder detected — symbolic and unresolved.
                instruction.debug_trace.push(DebugEntry {
                    line: instruction.line,
                    message: "Placeholder operand detected — operand remains unresolved.".to_string(),
                    severity: Severity::Shadowed,
                });

                instruction
                    .metadata_tags
                    .insert("meta_operand_type".to_string(), "Placeholder".to_string());

                // ⚠️ Signal potential rewalk if not already triggered.
                instruction.rewalk_flag = true;
            }

            _ => {
                // ✅ Not a meta operand — nothing to handle here.
            }
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

// ===================================================
// 🧭 Bearer — Operand Resolution Engine
// ===================================================
// This `impl Bearer` block defines the full behavioral logic
// for managing operand resolution from scroll parsing to
// Watchtower reporting. All functions are grouped into themed
// regions for clarity, maintainability, and spiritual tracing.
// ===================================================

impl Bearer {

    // ===================================================
    // ✅ POST-RESOLUTION CONFIRMATION
    // ===================================================

    /// ✅ Final confirmation that all operand fields have been classified and constructed.
    ///
    /// This method walks the operands assigned to an instruction and
    /// checks if all have been resolved to valid types. It ensures no
    /// placeholders, invalid stubs, or unresolved entries remain.
    ///
    /// This is a **post-pass sanity check** to confirm that all operands
    /// are spiritually and structurally aligned before proceeding to assembly.
    ///
    /// Returns `true` if all operands are valid and ready.
    pub fn validate_operands(instruction: &Instruction) -> bool {
        for operand in &instruction.resolved_operands {
            match operand {
                Operand::InvalidOperand(_) | Operand::Placeholder(_) => {
                    // 🧾 Record warning trace (optional in later Watchtower logging)
                    #[cfg(feature = "debug_mode")]
                    println!(
                        "⚠️ [Validate] Operand not fully resolved: {:?} (line {})",
                        operand, instruction.line
                    );

                    // 🚨 If any operand is incomplete, resolution is not valid
                    return false;
                }
                _ => {
                    // ✅ Operand is valid — continue checking others
                }
            }
        }

        // 🎯 All operands passed validation
        true
    }

    // ===================================================
    // 📡 WATCHTOWER & TRACE EMISSION
    // ===================================================

    /// 🛰 Emit debug snapshot to the Watchtower after operand resolution.
    ///
    /// This function creates a diagnostic payload from the instruction state
    /// and emits it to the central Watchtower system. It allows deeper
    /// system introspection and alignment checks across components.
    pub fn report_to_watchtower(instruction: &Instruction) {
        // Construct a basic debug payload based on the current instruction state
        let payload = DebugEntry {
            line: instruction.line,
            message: format!(
                "Resolution status: {:?} | Trust summary: {:?}",
                instruction.status,
                instruction.trust_summary.as_ref().unwrap_or(&TrustTier::Shadowed)
            ),
            severity: match instruction.status {
                InstructionStatus::ReadyToAssemble => Severity::Valid,
                InstructionStatus::RequiresResolution => Severity::Drifted,
                InstructionStatus::Invalid => Severity::Broken,
                InstructionStatus::RequiresRewalk => Severity::Shadowed,
            },
        };

        // Send the payload to the Watchtower if a hook exists
        if let Some(ref hook) = instruction.watchtower_hook {
            hook(payload.clone()); // Pass a clone if ownership is taken
        }

        // Always emit to CLI trace in debug mode for local inspection
        #[cfg(feature = "debug_mode")]
        {
            println!("📡 [Watchtower Emission] {:?}", payload);
        }

        // 📬 Future: Relay to NovaBridge (for AI-assisted commentary or remote logging)
        // if let Some(bridge) = NovaBridge::current() {
        //     let signature = Self::export_operand_signature(instruction);
        //     let nova_payload = NovaPayload::from_debug_entry(payload, signature);
        //     bridge.send(nova_payload);
        // }
    }


    /// 🧾 Optional serializer for logging or assembly review.
    ///
    /// Converts the resolved operand set into a readable signature format,
    /// useful for trace logs, scroll metadata, or assembler inspection.
    /// This acts as a compressed summary of operand resolution results.
    pub fn export_operand_signature(instruction: &Instruction) -> String {
        let mut signature = vec![];

        for operand in &instruction.resolved_operands {
            let kind = match operand {
                Operand::Literal { .. } => "Literal",
                Operand::Binding { .. } => "Symbol",
                Operand::Wildcard => "Wildcard",
                Operand::InstructionRef(_) => "InstructionRef",
                Operand::Placeholder(_) => "Placeholder",
                Operand::InvalidOperand(_) => "Invalid",
            };

            let value = format!("{:?}", operand);
            signature.push(format!("{}: {}", kind, value));
        }

        format!("[{}]", signature.join(" | "))
    }

    // ===================================================
    // 🌿 RESOLUTION ENTRY & SCHEMA LOADING
    // ===================================================

    /// 🌀 Begins operand resolution from the scroll tree root.
    ///
    /// This method plants the scroll tree into the Bearer and
    /// immediately initiates tree traversal to extract and classify operands.
    pub fn begin_resolution(&mut self, scroll_tree: ScrollTree) {
        self.scroll_tree = Some(scroll_tree);

        // 🌿 Begin operand discovery immediately
        self.walk_scroll_tree();
    }

    /// 📚 Loads the operand schema for a specific instruction.
    ///
    /// Retrieves the operand schema (arity and expected operand structure)
    /// from the instruction registry based on the instruction’s name.
    /// Logs a warning if the schema is missing, malformed, or mismatched.
    pub fn load_instruction_schema(&mut self, instruction: &Instruction) {
        self.instruction_schema = self
            .instruction_registry
            .get_schema(&instruction.name);

        if self.instruction_schema.is_none() {
            self.record_debug_entry(DebugEntry {
                line: instruction.line,
                message: format!("Missing schema for instruction '{}'", instruction.name),
                severity: Severity::Broken,
            });
        }
    }

    // ===================================================
    // 🔍 SCROLL TREE PROCESSING & ARITY VALIDATION
    // ===================================================

    /// 🌿 Walks the scroll tree and processes operand nodes.
    ///
    /// This function iterates through the children of the scroll tree root,
    /// classifies operand types, validates arity, and constructs resolved operands.
    /// It assumes a schema has been loaded prior to invocation.
    pub fn walk_scroll_tree(&mut self) {
        if self.scroll_tree.is_none() || self.instruction_schema.is_none() {
            eprintln!("⚠️ Cannot walk tree — scroll or schema missing.");
            return;
        }

        let tree = self.scroll_tree.as_ref().unwrap();
        let schema = self.instruction_schema.as_ref().unwrap();

        // Only process top-level children for now
        let operand_nodes = &tree.root.children;

        // 🔍 Validate operand count (arity)
        if !self.validate_arity(&tree.root, schema) {
            self.record_debug_entry(DebugEntry {
                line: 0,
                message: format!(
                    "Arity mismatch: expected {}, found {}.",
                    schema.arity,
                    operand_nodes.len()
                ),
                severity: Severity::Broken,
            });
            return;
        }

        // 🌱 Walk each operand node, classify, construct, and store
        for node in operand_nodes {
            let operand_type = self.classify_operand_type(node);
            let operand = self.construct_operand(node, operand_type);
            let trust = self.mark_trust_level(&operand);

            self.operands.push(operand.clone());

            self.record_debug_entry(DebugEntry {
                line: node.line,
                message: format!("Resolved operand: {:?} with trust {:?}", operand, trust),
                severity: Severity::Valid,
            });
        }
    }

    /// 🪞 Validates operand count against expected arity.
    ///
    /// Returns true if the number of operand nodes matches the schema arity.
    pub fn validate_arity(&self, node: &ScrollNode, schema: &OperandSchema) -> bool {
        node.children.len() == schema.arity
    }

    // ===================================================
    // 🛠 OPERAND CONSTRUCTION & TYPE LOGIC
    // ===================================================

    /// 🪶 Determines the operand type based on the node.
    ///
    /// This logic checks the structure and token contents of a scroll node
    /// to determine if it’s a literal, binding, or symbolic reference.
    /// For now, it's simple — but it's structured for evolution.
    pub fn classify_operand_type(&self, node: &ScrollNode) -> OperandType {
        if node.token.starts_with('"') && node.token.ends_with('"') {
            OperandType::Literal
        } else if node.token.starts_with('$') {
            OperandType::Binding
        } else if node.token == "*" {
            OperandType::Wildcard
        } else if node.token.starts_with("ref:") {
            OperandType::InstructionRef
        } else if node.token == "_" {
            OperandType::Placeholder
        } else {
            OperandType::Unknown
        }
    }

    /// 🏗️ Constructs the operand from a scroll node and type.
    ///
    /// This function builds the appropriate operand variant
    /// based on parsed operand type and the node's token contents.
    pub fn construct_operand(&self, node: &ScrollNode, operand_type: OperandType) -> Operand {
        match operand_type {
            OperandType::Literal => Operand::Literal {
                value: node.token.trim_matches('"').to_string(),
            },
            OperandType::Binding => Operand::Binding {
                symbol: node.token.trim_start_matches('$').to_string(),
            },
            OperandType::Wildcard => Operand::Wildcard,
            OperandType::InstructionRef => Operand::InstructionRef(
                node.token.trim_start_matches("ref:").to_string(),
            ),
            OperandType::Placeholder => Operand::Placeholder("_".to_string()),
            OperandType::Unknown => Operand::InvalidOperand(node.token.clone()),
        }
    }

    /// 🕊️ Assigns a trust tier to a resolved operand.
    ///
    /// This scoring system is temporary. It provides a rudimentary
    /// mapping of operand clarity for now — designed for future depth.
    pub fn mark_trust_level(&self, operand: &Operand) -> TrustTier {
        match operand {
            Operand::Literal { .. } | Operand::Binding { .. } => TrustTier::Sealed,
            Operand::Wildcard | Operand::InstructionRef(_) => TrustTier::Ambiguous,
            Operand::Placeholder(_) => TrustTier::Shadowed,
            Operand::InvalidOperand(_) => TrustTier::Broken,
        }
    }

    // ===================================================
    // 🧾 DEBUGGING & FINALIZATION HOOKS
    // ===================================================

    /// 🛡️ Records a debug trace entry.
    ///
    /// This method allows the Bearer to log significant events or status
    /// changes in the operand lifecycle. These entries are picked up by
    /// Watchtower or dev logs downstream for reflection and error tracing.
    pub fn record_debug_entry(&mut self, entry: DebugEntry) {
        self.debug_trace.push(entry);
    }

    /// 📦 Finalizes all resolved operands for handoff.
    ///
    /// This step marks the Bearer's resolution phase as complete.
    /// It verifies that all operands are resolved and adjusts the
    /// instruction status accordingly.
    ///
    /// Future hooks may emit diagnostics to `.logos` or Watchtower overlays.
    pub fn finalize_operands(&mut self) {
        if let Some(ref mut instruction) = self.current_instruction {
            let all_resolved = instruction
                .resolved_operands
                .iter()
                .all(|op| !matches!(op, Operand::InvalidOperand(_) | Operand::Placeholder(_)));

            if all_resolved {
                instruction.status = InstructionStatus::ReadyToAssemble;
            } else {
                instruction.status = InstructionStatus::RequiresResolution;

                // 🧾 Push debug trace for post-resolution awareness
                instruction.debug_trace.push(DebugEntry {
                    line: instruction.line,
                    message: "Finalization failed — unresolved or invalid operand detected.".to_string(),
                    severity: Severity::Broken,
                });

                // 🚨 Optional: Emit Watchtower trace
                Self::report_to_watchtower(instruction);
            }
        }
    }
}
