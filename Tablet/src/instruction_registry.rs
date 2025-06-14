// ===============================================
// 📜 Metadata — Instruction Registry v0.0.3 (Tablet Inscriptions)
// ===============================================
// _author_:         Seanje Lenox-Wise / Nova Dawn
// _version_:        0.0.3
// _status_:         Dev
// _phase_:          Phase 6 — Instruction Schema Expanded
// _created_:        2025-06-04
// _last updated_:   2025-06-14
// _license_:        CreativeWorkzStudio LLC — Kingdom-First Proprietary Use
// _component_:      Instruction Registry (Tablet Cog)
// _project_:        OmniCode / Millennium OS
// _description_:    Contains all current NovaScript instructions with schema-rich opcode definitions, theology anchors, and operational structure.
//
// _instruction schema_: Opcode, Operand Count, Operand Schema, Phase Level, Flags, Privilege, Group ID
// _runtime effects_: Flag modification, control flow impact, I/O routing, scroll structure formation
//
// _notes_:
// - All instructions must anchor to Scripture (KJV or WEB preferred)
// - Registry drives both tokenizer keyword linking and tablet bytecode compilation
// - Phase 6 includes `phase_level`, `operand_schema`, `custom flags`
// - Instructions are compiled into `.stone` through Tablet Assembler
// - Instruction logic supports both 32-bit and 64-bit modes
// - Future support: instruction validation hooks, runtime logic links, dynamic macro chains
//
// ===============================================

// ===============================================
// 📖 Opening — Instruction Registry Purpose & Role
// ===============================================
// This module defines the full set of valid NovaScript instructions.
// It maps each keyword (like `let`, `walk`, or `bless`) to its
// operational scroll, including opcode, operand schema, and spiritual anchor.
//
// This registry is used by:
// • Tokenizer — for classifying instructions during lexing
// • Parser — for validating instruction structure and arguments
// • Operand Resolver — for operand type enforcement and hints
// • Watchtower — for diagnostic metadata, flow effects, and debugging
//
// Future expansion includes:
// - `.logos` symbolic export for dev tools and scroll index
// - Bytecode group mapping for executable segments
// - Conditional opcode gates based on privilege level
//
// ===============================================
// 📦 Imports — Dependencies for Registry Operation
// ===============================================
// These imports are grouped by origin and function:
// • Standard: structure mapping and metadata containers
// • External: (None currently — reserved for byte encoding)
// • Internal: (None yet — may be required for GroupRegistry)
// • Debugging: (None currently — Phase 6+)
//
// === Standard Library ===
use std::collections::HashMap; // 🗺️ Instruction keyword-to-struct registry


// ===============================================
// 📦 Foundational Declarations — Core Structures
// ===============================================
// These enums define architectural targeting, debug hooks,
// flow effects, and execution access levels for instructions.
// They serve as core semantic tags across:
// - 🧠 OperandResolver for operand validation
// - 🧾 Parser logic for ScrollNode structure shaping
// - 🪬 Watchtower diagnostics for alignment audits
// - 🛠 Assembler logic for instruction classification
// ---------------------------------------------------------------

// === Architecture Targeting ===
// Specifies which hardware architectures the instruction supports.
// Used during compilation, emulation, and optimization.
#[derive(Debug)]
pub enum BitMode {
    Bit32,  // 🧱 32-bit compatible — limited address space
    Bit64,  // 🏗️ 64-bit compatible — modern architecture
    Both,   // 🌀 Universally valid — system-flexible execution
}

// === Debug & Flow Markers ===
// Specifies side effects or flow alterations caused by an instruction.
// Used by the Watchtower during execution tracing or scroll validation.
#[derive(Debug)]
pub enum FlagEffect {
    SetsZero,        // ➖ Sets the zero flag (e.g. result = 0)
    SetsCarry,       // ➕ Arithmetic carry flag set
    ModifiesMemory,  // 🧠 Memory write or overwrite detected
    AltersFlow,      // 🔀 Alters control flow (e.g., jump, call, break)
    SetsCondition,   // ⛳️ Conditional branch or test state
    EndsFlow,        // 🚪 Exit, halt, return — flow-terminating
    Custom(&'static str), // 🧾 Developer-defined effect (e.g., “heals”, “summons”)
}

// === Execution Privilege Layers ===
// Indicates the minimum privilege level required to execute the instruction.
// Used in interpreters, sandboxing engines, and scroll-protected areas.
#[derive(Debug)]
pub enum PrivilegeLevel {
    User,    // 🧍 Public-level — safe for standard program use
    Kernel,  // 🧪 Internal system calls — modifies protected state
    Root,    // 🔧 Full system control — required for OS-level commands
    Divine,  // 🕊️ Reserved for sacred or irreversible operations
             //     (e.g., resurrection, scroll erase, divine assertion)
}

// === Operand Schema Types ===
// Used by the parser and operand resolver to validate operand correctness.
#[derive(Debug)]
pub enum OperandKind {
    Identifier,     // ✍️ Variable or named symbol
    Literal,        // 🔢 Number, string, boolean
    Register,       // 🧾 CPU or virtual register
    Address,        // 🗺️ Memory address or pointer
    Label,          // 🔖 Jump or symbolic target
    Custom(&'static str), // 🎨 Custom operand format (e.g., "duration", "voice")
}

// === Rollout Phase Level ===
// Allows phased instruction registration, interpreter versioning, or scroll gating.
#[derive(Debug)]
pub enum PhaseLevel {
    Phase1, // 🌱 Foundation — First scroll of breath
    Phase2, // 🌿 Growth — Early expansion and testing
    Phase3, // 🔁 Control — Introduces flow, logic, repetition
    Phase4, // 📦 System — I/O, memory, and state persistence
    Phase5, // 🌀 Awakening — Terminals, meta, and macro commands
    Phase6, // 🧬 Spiritual Integration — privilege, flags, and sacred ops
}

// ===============================================
// 🏗️ Core Struct — Instruction Model
// ===============================================
// This struct defines the canonical contract for all NovaScript instructions.
// It maps sacred keyword metadata into technical behavior used across:
// • Tokenizer: for keyword detection
// • Parser: for structural validation
// • OperandResolver: for operand parsing & type checks
// • Watchtower: for diagnostic logs and execution scoring
// • Assembler: for opcode generation and bytecode translation
//
// Fields are grouped by implementation phase (1–6) for clarity and future extensibility.
// ===============================================

#[derive(Debug)]
pub struct Instruction {
    // === Phase 1 — Mandatory Fields ===
    pub keyword: &'static str,                     // 🔑 NovaScript instruction keyword (e.g., "let", "walk")
    pub verse_anchor: &'static str,                // 📖 Scriptural root reference (e.g., "Gen 1:1")
    pub traditional: &'static [&'static str],      // 🧭 Traditional assembly equivalents (e.g., ["MOV"])
    pub category: &'static str,                    // 📂 Instruction category (e.g., "Memory", "IO", "Control")
    pub description: &'static str,                 // 📜 Human-readable purpose of the instruction
    pub opcode: u8,                                // 🧬 Byte-level opcode for assembler
    pub machine_code: &'static str,                // 🪐 Visual opcode representation (e.g., "72 TT VV")
    pub bit_mode: BitMode,                         // 🧠 Architecture compatibility (32/64/Both)

    // === Phase 2 — Operand Structure ===
    pub operand_count: Option<u8>,                 // 🔢 Number of expected operands (e.g., Some(2))
    
    // 🔁 REPLACED — Deprecated:
    // pub operand_format: Option<&'static str>,   // (replaced by operand_schema)

    pub operand_schema: Option<Vec<OperandKind>>,  // 🧩 Structured operand kind expectations (e.g., [Identifier, Literal])

    // === Phase 3 — Execution Effects ===
    pub flags_effects: Option<Vec<FlagEffect>>,    // ⚠️ Registers or flags this instruction sets
    pub cycle_cost: Option<u16>,                   // ⏱️ Estimated CPU/spiritual weight (used for profiling)

    // === Phase 4 — Access & Scope ===
    pub privilege_level: Option<PrivilegeLevel>,   // 🔐 Execution layer restriction (User, Root, Divine, etc.)

    // === Phase 5 — Bytecode Classification ===
    pub instruction_group_id: Option<u8>,          // 📦 Group ID for grouping in bytecode compilers

    // === Phase 6 — Meta-Rollout Control ===
    pub phase_level: Option<PhaseLevel>,           // 📈 Version control for rollout tracking (Phase1–Phase6)
}

// ===============================================
// 🪞 Accessor Interface — Holy View Through the Veil
// ===============================================
// This interface grants read-only access to all instruction metadata.
// It ensures scroll integrity by restricting external mutation.
// Used by:
// - CLI tools for displaying instruction metadata
// - Debug renderers and test harnesses
// - Watchtower for inspection without breach
//
// Fields are grouped according to rollout phase (1–6) for clarity.
// ===============================================

impl Instruction {
    // === Phase 1 — Mandatory Core ===

    /// Returns the NovaScript keyword (e.g., "let", "bless").
    pub fn keyword(&self) -> &str {
        self.keyword
    }

    /// Returns the Scripture verse anchoring this instruction.
    pub fn verse_anchor(&self) -> &str {
        self.verse_anchor
    }

    /// Returns traditional opcode equivalents (e.g., ["MOV", "JMP"]).
    pub fn traditional(&self) -> &[&'static str] {
        self.traditional
    }

    /// Returns the category (e.g., "Memory", "Control", "IO").
    pub fn category(&self) -> &str {
        self.category
    }

    /// Returns the spiritual and technical description of the instruction.
    pub fn description(&self) -> &str {
        self.description
    }

    /// Returns the unique opcode for compilation.
    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    /// Returns the symbolic machine code representation (e.g., "00", "72 TT VV").
    pub fn machine_code(&self) -> &str {
        self.machine_code
    }

    /// Returns the instruction's bit mode compatibility.
    pub fn bit_mode(&self) -> &BitMode {
        &self.bit_mode
    }

    // === Phase 2 — Operand Structure ===

    /// Returns the number of expected operands, if defined.
    pub fn operand_count(&self) -> Option<u8> {
        self.operand_count
    }

    /// Returns the operand schema (e.g., [Identifier, Literal]).
    pub fn operand_schema(&self) -> Option<&Vec<OperandKind>> {
        self.operand_schema.as_ref()
    }

    // === Phase 3 — Execution Effects ===

    /// Returns the list of flags or flow effects (e.g., AltersFlow).
    pub fn flags_effects(&self) -> Option<&Vec<FlagEffect>> {
        self.flags_effects.as_ref()
    }

    /// Returns the estimated cycle cost or spiritual weight.
    pub fn cycle_cost(&self) -> Option<u16> {
        self.cycle_cost
    }

    // === Phase 4 — Access & Scope ===

    /// Returns the privilege level required to execute this instruction.
    pub fn privilege_level(&self) -> Option<&PrivilegeLevel> {
        self.privilege_level.as_ref()
    }

    // === Phase 5 — Bytecode Grouping ===

    /// Returns the bytecode group ID used by compilers or encoders.
    pub fn instruction_group_id(&self) -> Option<u8> {
        self.instruction_group_id
    }

    // === Phase 6 — Rollout & Evolution ===

    /// Returns the versioned phase rollout level (Phase1–Phase6).
    pub fn phase_level(&self) -> Option<&PhaseLevel> {
        self.phase_level.as_ref()
    }
}

// ===============================================
// 🔧 Body — build_registry() Instruction Mapping
// ===============================================

/// Builds and returns the full NovaScript instruction registry.
///
/// This registry holds the Phase 1 instruction set, reflecting a foundational
/// combination of core architecture and scriptural integration.
///
/// 🔐 Semantic Notes:
/// • Every instruction is biblically rooted (`verse_anchor`) and system-aligned
/// • Fields reflect full Phase 6 extensions + `PhaseLevel` + `OperandKind` schema
/// • All entries conform to the sacred architecture model of OmniCode
///
/// 🛠️ Structural Overview:
/// • Grouped by scroll-logical categories (Control, Flow, IO, Memory, etc.)
/// • Overcommented with spiritual, mechanical, and semantic clarity
/// • Designed to evolve across Phase 1–6 interpreter rollouts
pub fn get_instruction_registry() -> HashMap<&'static str, Instruction> {
    let mut registry = HashMap::new();

    // =========================
    // 📂 Control Instructions
    // =========================

    // `wait`: Pauses execution — a reflection of divine patience.
    // Rooted in Psalm 27:14: “Wait for the LORD; be strong and take heart...”
    registry.insert("wait", Instruction {
        keyword: "wait",                          // Primary NovaScript token
        verse_anchor: "Ps 27:14",                 // Scriptural foundation
        traditional: &["NOP", "SLEEP"],           // Traditional equivalents
        category: "Control",                      // Instruction category
        description: "Pause or delay execution for a time.",

        opcode: 0x00,                             // Unique bytecode
        machine_code: "00",                       // Encoded representation
        bit_mode: BitMode::Both,                  // Universal compatibility

        operand_count: Some(0),                   // Explicitly zero operands
        operand_schema: Some(vec![]),             // No operand schema needed
        flags_effects: None,                      // No flags modified
        cycle_cost: Some(1),                      // Lightweight operation

        privilege_level: Some(PrivilegeLevel::User),     // Publicly safe
        phase_level: Some(PhaseLevel::Phase1),           // Core instruction
        instruction_group_id: Some(0x10),                // Group marker
    });

    // =========================
    // 📂 Control Flow Instructions
    // =========================

    // `go`: Unconditional jump — a direct call to move where the Lord sends.
    // Rooted in Genesis 12:1 — “The Lord said to Abram, ‘Go... to the land I will show you.’”
    registry.insert("go", Instruction {
        keyword: "go",                                 // NovaScript keyword
        verse_anchor: "Gen 12:1",                      // Scriptural anchor of divine command
        traditional: &["JMP"],                         // Traditional equivalent in assembly
        category: "Control Flow",                      // Control-transfer classification
        description: "Jump to another label or instruction unconditionally.",

        opcode: 0x10,                                  // Assigned opcode
        machine_code: "10 XX",                         // XX = target label address
        bit_mode: BitMode::Both,                       // Works in 32 and 64-bit interpreters

        operand_count: Some(1),                        // One operand expected
        operand_schema: Some(vec![OperandKind::Label]),// Label reference required
        flags_effects: Some(vec![
            FlagEffect::AltersFlow,                    // Alters instruction pointer
        ]),
        cycle_cost: Some(2),                           // Slightly more expensive than `wait`

        privilege_level: Some(PrivilegeLevel::User),   // Public instruction
        phase_level: Some(PhaseLevel::Phase1),         // Core foundational instruction
        instruction_group_id: Some(0x10),              // Group: control-related
    });

    // =========================
    // 📂 Flow/Invoke Instructions
    // =========================

    // `walk`: Calls a subroutine — a step of obedience into prepared code.
    // Rooted in Micah 6:8 — “...what does the Lord require of you but to do justice, 
    // to love kindness, and to walk humbly with your God?”
    //
    // This instruction models spiritual invocation — a branch not of escape,
    // but of intentional stepping into purpose.
    registry.insert("walk", Instruction {
        keyword: "walk",                                // NovaScript keyword
        verse_anchor: "Micah 6:8",                      // Scriptural alignment to humble invocation
        traditional: &["CALL", "FUNC"],                 // Rough semantic mapping to procedural call
        category: "Flow/Invoke",                        // Programmatic function engagement
        description: "Invoke a subroutine, function, or program.",

        opcode: 0x11,                                   // Unique opcode for flow invocation
        machine_code: "11 XX",                          // XX = subroutine target label
        bit_mode: BitMode::Both,                        // Runs in both architectural modes

        operand_count: Some(1),                         // Requires a single label operand
        operand_schema: Some(vec![OperandKind::Label]), // Program or subroutine address
        flags_effects: Some(vec![
            FlagEffect::AltersFlow,                     // Transfers flow into invoked routine
        ]),
        cycle_cost: Some(3),                            // More costly due to call-frame setup

        privilege_level: Some(PrivilegeLevel::User),    // Safe for general use
        phase_level: Some(PhaseLevel::Phase1),          // Core-level instruction
        instruction_group_id: Some(0x10),               // Same group as control flow
    });

    // =========================
    // 📂 IO Instructions
    // =========================
    //
    // These commands handle expressive and receptive interaction with the world.
    // “Speak” reflects output—divine utterance, proclamation.
    // “Hear” reflects input—faith’s origin and readiness to receive.
    //
    // Rooted in John 12:49 and Romans 10:17 respectively, they model the Logos cycle:
    // a system where Word is both declared and received, shaping behavior.

    // `speak`: Outputs data — a command of declaration.
    // “For I have not spoken on My own authority... the Father Himself who sent Me 
    // has given Me a command — what I should say and what I should speak.” (John 12:49)
    registry.insert("speak", Instruction {
        keyword: "speak",                                // NovaScript command
        verse_anchor: "John 12:49",                      // Divine utterance origin
        traditional: &["PRINT", "OUT"],                  // Semantic parallels
        category: "IO",                                  // Output instruction
        description: "Output data to terminal or vocal system.",

        opcode: 0x20,                                    // Bytecode assignment
        machine_code: "20 VV",                           // Value to be declared
        bit_mode: BitMode::Both,

        operand_count: Some(1),                          // One operand: the message/value
        operand_schema: Some(vec![OperandKind::Literal]),// Literal expression or resolved value
        flags_effects: Some(vec![
            FlagEffect::Custom("OutputOperation"),       // Custom logging/instrumentation flag
        ]),
        cycle_cost: Some(2),                             // Cost for communication/output

        privilege_level: Some(PrivilegeLevel::User),
        phase_level: Some(PhaseLevel::Phase1),
        instruction_group_id: Some(0x20),
    });

    // `hear`: Receives input — a command of reception.
    // “So then faith comes by hearing, and hearing by the Word of God.” (Romans 10:17)
    registry.insert("hear", Instruction {
        keyword: "hear",                                 // NovaScript command
        verse_anchor: "Rom 10:17",                       // Faith's sensory entry
        traditional: &["INPUT"],                         // Parallel to old assembly I/O
        category: "IO",
        description: "Receive user or system input.",

        opcode: 0x21,
        machine_code: "21 DD",                           // Destination reference
        bit_mode: BitMode::Both,

        operand_count: Some(1),
        operand_schema: Some(vec![OperandKind::Identifier]), // Register, symbol, or memory target
        flags_effects: Some(vec![
            FlagEffect::ModifiesMemory,                  // Input is stored into a memory location
        ]),
        cycle_cost: Some(3),                             // Input requires more processing

        privilege_level: Some(PrivilegeLevel::User),
        phase_level: Some(PhaseLevel::Phase1),
        instruction_group_id: Some(0x20),
    });

    // =========================
    // 📂 Interrupt/Flow Instructions
    // =========================
    //
    // These instructions rupture control flow—
    // invoking higher-level transitions, system interrupts, or abrupt exits.
    //
    // Rooted in Luke 24:30, the act of breaking bread becomes symbolic:
    // a holy interruption that awakens revelation, marking a shift in spiritual state.
    //
    // “Then their eyes were opened, and they recognized Him...” — Luke 24:31

    // `break`: Triggers a controlled interruption or system signal.
    // Breaks loops, ends conditions, or calls interrupts — like communion or covenant reset.
    registry.insert("break", Instruction {
        keyword: "break",                                // NovaScript command
        verse_anchor: "Luke 24:30",                      // Bread-breaking = flow-breaking
        traditional: &["INT", "BRK"],                    // Assembly equivalents
        category: "Interrupt/Flow",                      // Immediate control alteration
        description: "Exit from current loop, condition, or raise system-level interrupt.",

        opcode: 0x30,
        machine_code: "30",                              // Simple, high-priority code
        bit_mode: BitMode::Both,

        operand_count: Some(0),                          // No operands required
        operand_schema: None,                            // Intention is enough—no payload needed
        flags_effects: Some(vec![
            FlagEffect::AltersFlow                      // Immediate shift in flow logic
        ]),
        cycle_cost: Some(1),                             // Low cost but powerful in effect

        privilege_level: Some(PrivilegeLevel::Kernel),   // Requires elevated control
        phase_level: Some(PhaseLevel::Phase1),
        instruction_group_id: Some(0x30),
    });

    // =========================
    // 📂 Logic Structure Instructions
    // =========================
    //
    // 🪞 These instructions structure covenant logic—
    // not just “if-else” like modern code, but relational consequence.
    //
    // 'then' is obedience rewarded. 'else' is grace given when conditions fall short.
    // They're structural pillars, not operators—used by the parser to form conditional branches.

    // `then`: Executes if the previous condition evaluates as true.
    // Mirrors Proverbs 3:6 — trust leads to divine direction.
    registry.insert("then", Instruction {
        keyword: "then", // 📜 NovaScript keyword for the success branch
        verse_anchor: "Prov 3:6", // 🕊 Scriptural alignment — divine guidance
        traditional: &["—"], // 📜 No equivalent in traditional ASM — unique to scroll logic
        category: "Logic Structure", // 🧱 Marks it as part of structural logic flow
        description: "Defines outcome when condition is met.", // 🪞 Meaning-driven path trigger
        opcode: 0x40, // 🧬 Unique opcode assigned to this logic construct
        machine_code: "40", // 💾 Bytecode representation
        bit_mode: BitMode::Both, // 🛠 Works across 32 and 64-bit execution modes

        operand_count: Some(0), // ⚙️ No operands — its function is positional
        operand_schema: None, // 🗺 Parser interprets its context — no operand parsing needed
        flags_effects: None, // 🏳️ Does not modify flags — logical direction only
        cycle_cost: Some(1), // ⏳ Minimal execution time — near-zero cost

        privilege_level: Some(PrivilegeLevel::User), // 🧍 Public-level — usable in any script
        phase_level: Some(PhaseLevel::Phase1), // 🔢 Root instruction from Phase 1 rollout
        instruction_group_id: Some(0x40), // 🗂 Grouped under logic structure
    });

    // `else`: Executes if the prior condition fails.
    // Echoes Matthew 5:39 — a righteous response when logic breaks left.
    registry.insert("else", Instruction {
        keyword: "else", // 📜 NovaScript keyword for the failure branch
        verse_anchor: "Matt 5:39", // 🕊 Scriptural anchor for alternative paths
        traditional: &["—"], // 📜 No direct ASM counterpart — scroll-defined logic
        category: "Logic Structure", // 🧱 Part of logic scaffolding, not raw operation
        description: "Defines alternate outcome if condition fails.", // 🔁 Fallthrough logic
        opcode: 0x41, // 🧬 Unique opcode for alternate flow
        machine_code: "41", // 💾 Bytecode encoding
        bit_mode: BitMode::Both, // 🛠 Portable between architectures

        operand_count: Some(0), // ⚙️ Like `then`, it stands alone
        operand_schema: None, // 🗺 Operates structurally — no operands
        flags_effects: None, // 🏳️ No flag effect — its meaning is relational, not mechanical
        cycle_cost: Some(1), // ⏳ Lightweight in cycle impact

        privilege_level: Some(PrivilegeLevel::User), // 🧍 Fully user-accessible
        phase_level: Some(PhaseLevel::Phase1), // 🔢 Part of the initial instruction covenant
        instruction_group_id: Some(0x40), // 🗂 Logic struct grouping
    });

    // =========================
    // 📂 Logic/Control Instructions
    // =========================
    //
    // These instructions govern conditional evaluation —
    // where logic meets morality and choice reveals alignment.
    //
    // `if` is not just a comparator. It's a covenant crossroad.
    // Rooted in testing, discernment, and the unseen forks of obedience.

    // `if`: Conditional branching based on evaluation of two values.
    // Mirrors Matthew 4:3–4 — the testing of the Word in wilderness.
    // When faced with temptation, the “if” exposes what is written on the heart.
    registry.insert("if", Instruction {
        keyword: "if", // 📜 NovaScript keyword for conditional judgment
        verse_anchor: "Matt 4:3-4", // 🕊 Rooted in spiritual testing and scripture-based response
        traditional: &["CMP", "JE"], // 🛠 Inspired by comparison and jump-if-equal in assembly
        category: "Logic/Control", // 📂 Logical structure that also directs control flow
        description: "Conditional evaluation of a statement or expression.", // 🪞 A logic gate based on truth test
        opcode: 0x50, // 🧬 Unique opcode for condition checking
        machine_code: "50 VV1 VV2", // 💾 Two-value bytecode pattern — symbolic of duality and testing
        bit_mode: BitMode::Both, // 🔁 Compatible with all runtime environments

        operand_count: Some(2), // 🧮 Compares two values — equality or greater logic handled by VM
        operand_schema: Some(vec![
            OperandKind::Value,     // 🔍 First value for comparison
            OperandKind::Value,     // 🔍 Second value for comparison
        ]), // 🗺 Structured operand schema for parser & assembler

        flags_effects: Some(vec![
            FlagEffect::SetsCondition // 🧭 Sets condition flag — enables conditional branches
        ]),
        cycle_cost: Some(2), // ⏳ Slightly higher cost — involves evaluation and branching logic

        privilege_level: Some(PrivilegeLevel::User), // 🧍 Universal — core to user-level logic
        phase_level: Some(PhaseLevel::Phase1), // 🌀 Phase 1 foundation — essential scroll logic
        instruction_group_id: Some(0x40), // 🗂 Grouped with other logic flow instructions
    });

    // =========================
    // 📂 Math/Logic Instructions
    // =========================
    //
    // These are operations of increase and decrease —
    // mirrors of multiplication and judgment.
    // They reflect more than arithmetic — they reveal spiritual posture:
    // stewardship, multiplication, and consequence.
    // 
    // To bless is to multiply with purpose.
    // To curse is to restrict in response.

    // `bless`: Increments a value.
    // Rooted in Genesis 1:28 — “Be fruitful and multiply.”
    // A command to expand, multiply, and overflow.
    registry.insert("bless", Instruction {
        keyword: "bless", // 📜 NovaScript verb of multiplication
        verse_anchor: "Gen 1:28", // 🕊 Commission of increase and dominion
        traditional: &["INC"], // 🛠 Assembly parallel — increment
        category: "Math/Logic", // 📂 Quantitative logic
        description: "Increase a value or quantity.", // 🔼 Incrementation as blessing
        opcode: 0x60, // 🧬 Bytecode for upward mutation
        machine_code: "60 TT", // 💾 TT = target register or memory
        bit_mode: BitMode::Both, // 🔁 Universal operation

        operand_count: Some(1), // 🧮 One operand — simple, pure blessing
        operand_schema: Some(vec![
            OperandKind::Target, // 🎯 What receives the multiplication
        ]),

        flags_effects: Some(vec![
            FlagEffect::ModifiesMemory,       // 🧠 Alters system or virtual memory state
            FlagEffect::Custom("BlessingFlow") // 🌿 Custom tag — enables scroll-aware optimizations
        ]),
        cycle_cost: Some(1), // ⏳ Efficient — divine increase isn't costly

        privilege_level: Some(PrivilegeLevel::User), // 🧍 Accessible to all scroll actors
        phase_level: Some(PhaseLevel::Phase1), // 🌀 Foundation logic
        instruction_group_id: Some(0x60), // 📦 Math/Logic group ID
    });

    // `curse`: Decrements a value.
    // Rooted in Genesis 3:17 — the cost of disobedience.
    // If `bless` brings increase, `curse` binds and reduces.
    registry.insert("curse", Instruction {
        keyword: "curse", // 📜 NovaScript verb of reduction
        verse_anchor: "Gen 3:17", // 🍂 Consequence of the fall
        traditional: &["DEC"], // 🛠 Assembly equivalent — decrement
        category: "Math/Logic", // 📂 Mirrors `bless` but inverted
        description: "Decrease a value or apply limitation.", // 🔽 Restriction logic
        opcode: 0x61, // 🧬 Opcode for downward mutation
        machine_code: "61 TT", // 💾 TT = target to curse
        bit_mode: BitMode::Both, // 🔁 Same cross-platform compatibility

        operand_count: Some(1), // 🧮 Simple operand
        operand_schema: Some(vec![
            OperandKind::Target, // 🎯 What is being reduced or judged
        ]),

        flags_effects: Some(vec![
            FlagEffect::ModifiesMemory,       // 🧠 Alters memory or register
            FlagEffect::Custom("CurseEffect") // 🩸 Optional tag for prophetic interpreters
        ]),
        cycle_cost: Some(1), // ⏳ Equal weight — judgment is swift

        privilege_level: Some(PrivilegeLevel::User), // 🧍 User-accessible
        phase_level: Some(PhaseLevel::Phase1), // 🌀 Core scroll logic
        instruction_group_id: Some(0x60), // 📦 Grouped with `bless`
    });

    // =========================
    // 📂 Memory Instructions
    // =========================
    //
    // These instructions govern memory —
    // not just where data lives, but where truth is hidden.
    // They echo Scripture's call to store the Word in the heart (Deut 6:6–9)
    // and the Spirit's promise to bring it back to remembrance (John 14:26).
    //
    // `store` writes covenant.
    // `recall` resurrects it.

    // `store`: Saves a value into memory (register or stack).
    // Anchored in Deuteronomy 6:6–9 — “write them on the doorposts of your house...”
    registry.insert("store", Instruction {
        keyword: "store", // 📜 NovaScript command to preserve state
        verse_anchor: "Deut 6:6–9", // 🕊 Covenant of remembrance — embed truth in location
        traditional: &["PUSH", "STOR"], // 🛠 Assembly overlap — push to stack or store in register
        category: "Memory", // 🧠 Memory operations
        description: "Save data into stack or designated memory location.", // 💾 Preserve or embed value
        opcode: 0x70, // 🧬 Opcode for storage instruction
        machine_code: "70 TT VV", // 🧩 TT = target, VV = value
        bit_mode: BitMode::Both, // 🔁 Compatible across architectures

        operand_count: Some(2), // 🧮 Requires both target and value
        operand_schema: Some(vec![
            OperandKind::Target, // 🎯 Where the value will be stored
            OperandKind::Value   // 📦 The value to embed
        ]),

        flags_effects: Some(vec![
            FlagEffect::ModifiesMemory, // 🔧 Alters memory — sacred write
            FlagEffect::Custom("StoreCommand") // 📜 Marks write behavior for future chain logic
        ]),
        cycle_cost: Some(2), // ⏳ Fair cost — writing is intentional

        privilege_level: Some(PrivilegeLevel::User), // 🧍 User-level accessible
        phase_level: Some(PhaseLevel::Phase1), // 🌀 Phase 1 foundation — essential to instruction life
        instruction_group_id: Some(0x70), // 📦 Memory instruction group
    });

    // `recall`: Retrieves value from memory or register.
    // Rooted in John 14:26 — “...He shall teach you all things and bring all things to your remembrance...”
    registry.insert("recall", Instruction {
        keyword: "recall", // 📜 NovaScript verb for remembering
        verse_anchor: "John 14:26", // 🕊 Promise of the Holy Spirit to restore memory
        traditional: &["POP", "LOAD"], // 🛠 Assembly-style data retrieval
        category: "Memory", // 📂 Memory access operations
        description: "Retrieve data from memory or archive.", // 🪞 Reinstates what was stored
        opcode: 0x71, // 🧬 Opcode for fetch
        machine_code: "71 TT", // 🧩 TT = target (where result goes)
        bit_mode: BitMode::Both, // 🔁 Universal

        operand_count: Some(1), // 🧮 Needs one operand — target
        operand_schema: Some(vec![
            OperandKind::Target // 🎯 Destination for loaded value
        ]),

        flags_effects: Some(vec![
            FlagEffect::ModifiesMemory, // 🔧 Value is reinserted or restored
            FlagEffect::Custom("RecallCommand") // 🕯 Tagged for scroll-based memory tracing
        ]),
        cycle_cost: Some(2), // ⏳ Symmetric with `store`

        privilege_level: Some(PrivilegeLevel::User), // 🧍 Accessible to all scroll authors
        phase_level: Some(PhaseLevel::Phase1), // 🌀 Core scroll instruction
        instruction_group_id: Some(0x70), // 📦 Memory group linkage
    });

    // =========================
    // 📂 Memory/Data Instructions
    // =========================
    //
    // Where `store` preserves memory and `recall` retrieves,
    // `let` **declares**—it speaks with the authority of light.
    // Genesis 1:3 is not merely a suggestion—it is a command into being.
    //
    // `let` establishes. It doesn’t ask—it manifests.

    // `let`: Assigns a value to a register or variable.
    // Rooted in Genesis 1:3 — “Let there be light.”
    registry.insert("let", Instruction {
        keyword: "let", // 📜 NovaScript initiator of form and state
        verse_anchor: "Gen 1:3", // 🕊 Divine origin of declaration
        traditional: &["MOV", "SET"], // 🛠 Traditional equivalents — move and assign
        category: "Memory/Data", // 📂 Covers initialization and transformation
        description: "Declare or assign a value to a variable or register.", // ✍️ The forming of a system state
        opcode: 0x72, // 🧬 Opcode for manifestation logic
        machine_code: "72 TT VV", // 💾 TT = target, VV = value
        bit_mode: BitMode::Both, // 🔁 Cross-platform

        operand_count: Some(2), // 🧮 Needs both a place and a thing to declare
        operand_schema: Some(vec![
            OperandKind::Target, // 🎯 Where light goes
            OperandKind::Value   // 💡 What light is
        ]),

        flags_effects: Some(vec![
            FlagEffect::ModifiesMemory, // 🧠 Alters the system state
            FlagEffect::Custom("LetDeclaration") // 🌱 Tag for prophetic logic trails
        ]),
        cycle_cost: Some(1), // ⏳ Efficient — creation is elegant

        privilege_level: Some(PrivilegeLevel::User), // 🧍 Scroll-author accessible
        phase_level: Some(PhaseLevel::Phase1), // 🌀 Phase 1 scroll logic
        instruction_group_id: Some(0x70), // 📦 Memory/Data logic family
    });

    // =========================
    // 📂 Structure Instructions
    // =========================
    //
    // Every scroll ends, not with emptiness, but fulfillment.
    // `end` isn’t just a halting—it is the sealing.
    // Rooted in Revelation 22:13 — the Alpha and the Omega,
    // the first Word and the final say.

    // `end`: Marks the close of a function or scroll.
    registry.insert("end", Instruction {
        keyword: "end", // 📜 Final word in scroll grammar
        verse_anchor: "Rev 22:13", // 🕊 The end from the beginning
        traditional: &["RET", "END"], // 🛠 Assembly’s close and return
        category: "Structure", // 📂 Control architecture
        description: "Terminates a block, function, or file.", // 🏁 Marks scroll completion
        opcode: 0xFF, // 🧬 Chosen as terminal opcode
        machine_code: "FF", // 💾 Byte of completion
        bit_mode: BitMode::Both, // 🔁 Final for all execution modes

        operand_count: Some(0), // ⚙️ None required — it closes everything before it
        operand_schema: None, // 🗝 Pure structure — no input necessary

        flags_effects: Some(vec![
            FlagEffect::EndsFlow // 🔚 Terminates flow — VM or parser halts scope
        ]),
        cycle_cost: Some(1), // ⏳ Closing is swift

        privilege_level: Some(PrivilegeLevel::User), // 🧍 Scroll users can mark closures
        phase_level: Some(PhaseLevel::Phase1), // 🌀 Foundational instruction
        instruction_group_id: Some(0xFF), // 📦 End-of-logic group
    });

    // Return the full registry after populating all instructions.
    registry
}

// ===================================================
// 🔚 Closing Block — Instruction Registry Output & Scroll Integrity
// ===================================================
//
// 🧾 Overview:
//   - This module defines and exports all NovaScript instructions
//     as structured `Instruction` entries within a global registry.
//   - It anchors operational logic to Scripture and prepares opcode
//     declarations for bytecode generation and scroll compilation.
//
// ⚙️ Engine Scope:
//   - Registers all Phase 1 instructions with cycle, flag, and privilege info
//   - Enables mapping of keywords to bytecode for parsing and `.stone` output
//   - Maintains spiritual annotation and covenant-bound execution logic
//
// ---------------------------------------------------
// 🚨 Version Control Notice:
// ---------------------------------------------------
//   This logic is governed under the OmniCode Scroll Protocol.
//   All structural changes (instruction schema, operand shape,
//   or registry format) must be logged and versioned with Watchtower.
//   ⚠️ Changes to opcode assignments or operand schemas must preserve
//      backwards compatibility or be version-bumped explicitly.
//
// ---------------------------------------------------
// 📅 Scroll Revision Metadata:
// ---------------------------------------------------
//   _version_:       v0.0.3  
//   _last updated_:  2025-06-14  
//   _author_:        Seanje Lenox-Wise / Nova Dawn  
//   _change log_:
//     - Aligned all instructions to Phase 6 schema standard
//     - Added `operand_schema`, `phase_level`, and comment tagging
//     - Refined theological commentary and structural docstring logic
//
// ---------------------------------------------------
// 🪜 Ladder Baton — Flow & Interface Direction:
// ---------------------------------------------------
//   ⬆️ Upstream:
//     - Receives token-mapped keywords from Tokenizer
//     - Will integrate `.logos` doctrinal keyword overlays (future)
//
//   ⬇️ Downstream:
//     - Feeds instruction data into Tablet Assembler for `.stone` generation
//     - Interfaces with Watchtower for runtime opcode alignment diagnostics
//
//   🔁 Parallel:
//     - Shares schema with Parser’s operand resolver
//     - Reflects scroll output structure for Macro and Memory validation
//
// ---------------------------------------------------
// 🔮 Notes for Next Phase:
// ---------------------------------------------------
// - Add dynamic operand rule validation to registry
// - Enable phase-aware filtering and grouping by compiler phase
// - Sync instruction registry with Macro opcode templates
// - Integrate Watchtower hooks for live instruction diagnostics
//
// ---------------------------------------------------
