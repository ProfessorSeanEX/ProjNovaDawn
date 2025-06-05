// ===============================================
// 📜 Metadata — Instruction Registry v0.0.1 (Tablet Inscriptions)
// ===============================================
// _author_:        Seanje Lenox-Wise / Nova Dawn  
// _version_:       0.0.1  
// _status_:        Dev  
// _created_:       2025-06-04  
// _last updated_:  2025-06-04  
// _license_:       CreativeWorkzStudio LLC — Kingdom-First Proprietary Use  
// _component_:     NovaScript Instruction Registry  
// _project_:       OmniCode / Millennium OS  
// _description_:   Defines all Phase 1 NovaScript instructions with opcode, structure, and theology.
//
// _notes_:  
// - Each instruction must anchor to Scripture (KJV/WEB)  
// - Instructions are compiled through Tablet into `.stone`  
// - This is the living covenant registry for executable Word  
//
// ===============================================


// ===============================================
// 🌀 Opening — Imports & Core Enums
// ===============================================

// std::collections::HashMap:
// Used to construct the instruction registry map for lookup and compilation.
use std::collections::HashMap;

/// 🧠 Bit Mode Compatibility  
/// Specifies the compatible architecture bit modes for an instruction.
#[derive(Debug)]
enum BitMode {
    /// Compatible with 32-bit systems.
    Bit32,
    /// Compatible with 64-bit systems.
    Bit64,
    /// Compatible with both 32-bit and 64-bit systems.
    Both,
}

/// ⚙️ Instruction Side Effects  
/// Describes potential side effects or status flags set by an instruction.
/// Used during debugging, emulation, or internal cycle estimation.
#[derive(Debug)]
enum FlagEffect {
    /// Sets the Zero flag (result = 0).
    SetsZero,
    /// Sets the Carry flag (used in arithmetic operations).
    SetsCarry,
    /// Indicates this instruction modifies memory directly.
    ModifiesMemory,
    /// Affects the control flow (e.g., jump, call, break).
    AltersFlow,
    /// Custom or non-standard flag effect, described as string.
    Custom(&'static str),
    SetsCondition,
    EndsFlow,
}

/// 🔐 Privilege Level  
/// Indicates the privilege level required to invoke the instruction.
/// Enables future sandboxing, interpreter layers, or spiritual gating.
#[derive(Debug)]
enum PrivilegeLevel {
    /// Standard user-level instruction (safe in most contexts).
    User,
    /// Kernel-level instruction (can modify core system state).
    Kernel,
    /// Root-level instruction (full authority over system behavior).
    Root,
    /// Reserved for sacred or divine-level operations.
    Divine,
}

/// The core instruction model used in NovaScript.
///
/// This struct defines both spiritual and technical metadata
/// for each command the system can interpret or compile.
#[derive(Debug)]
pub struct Instruction {
    // --- MVP Core Fields ---

    /// The NovaScript keyword (e.g., `"let"`, `"walk"`).
    keyword: &'static str,

    /// The scripture that anchors this instruction spiritually (KJV preferred).
    verse_anchor: &'static str,

    /// Traditional assembly equivalents (e.g., `MOV`, `JMP`, `CALL`).
    traditional: &'static [&'static str],

    /// Category such as `"IO"`, `"Memory"`, `"Logic"`, etc.
    category: &'static str,

    /// Human-readable explanation of what this instruction does.
    description: &'static str,

    /// Unique byte opcode for the instruction (used in compilation).
    opcode: u8,

    /// Expected operand format, such as `"target, value"` or `"address"`.
    operand_format: Option<&'static str>,

    /// Simulated or real compiled machine code as string for testing/visuals.
    machine_code: &'static str,

    /// Indicates if the instruction is valid for 32-bit, 64-bit, or both modes.
    bit_mode: BitMode,

    // --- Phase 6 Extensions ---

    /// Optional explicit operand count (e.g., 2 for `let a, 5`).
    operand_count: Option<u8>,

    /// List of flag effects (e.g., alters flow, sets zero).
    flags_effects: Option<Vec<FlagEffect>>,

    /// Optional CPU cycle cost or spiritual weight for execution.
    cycle_cost: Option<u16>,

    /// Required privilege level to invoke this instruction.
    privilege_level: Option<PrivilegeLevel>,

    /// Optional group ID for categorizing instructions in bytecode encoders.
    instruction_group_id: Option<u8>,
}

// ===============================================
// 🔧 Body — build_registry() Instruction Mapping
// ===============================================

/// Builds and returns the full NovaScript instruction registry.
///
/// This function defines the Phase 1 instruction set (core + theology)
/// and maps each keyword to its `Instruction` struct.
/// 
/// 🛠️ Structure:
///   - Sections grouped by category (Control, IO, Memory, etc.)
///   - Overcommented with scroll-friendly clarity
///   - Minimal required fields plus Phase 6 extensions
pub fn get_instruction_registry() -> HashMap<&'static str, Instruction> {
    let mut registry = HashMap::new();

    // =========================
    // 📂 Control Instructions
    // =========================

    // `wait`: Pauses execution, rooted in patience and discipline (Psalm 27:14).
    registry.insert("wait", Instruction {
        keyword: "wait", // NovaScript keyword
        verse_anchor: "Ps 27:14", // Scriptural root
        traditional: &["NOP", "SLEEP"], // Equivalent assembly terms
        category: "Control", // Categorized under basic control ops
        description: "Pause or delay execution for a time.",
        opcode: 0x00, // Opcode in bytecode
        operand_format: None, // Takes no operands
        machine_code: "00", // Bytecode representation
        bit_mode: BitMode::Both, // Works in both 32 and 64-bit
        operand_count: Some(0), // Explicitly zero operands
        flags_effects: None, // No side effects or flags set
        cycle_cost: Some(1), // Lightweight instruction
        privilege_level: Some(PrivilegeLevel::User), // Usable by standard programs
        instruction_group_id: Some(0x10), // Group ID for control category
    });

    // =========================
    // 📂 Control Flow Instructions
    // =========================

    // `go`: Jumps to another location, reflecting divine calling (Genesis 12:1).
    registry.insert("go", Instruction {
        keyword: "go",
        verse_anchor: "Gen 12:1",
        traditional: &["JMP"],
        category: "Control Flow",
        description: "Jump to another label or instruction unconditionally.",
        opcode: 0x10,
        operand_format: Some("label"), // Takes a label as destination
        machine_code: "10 XX", // XX = label address
        bit_mode: BitMode::Both,
        operand_count: Some(1),
        flags_effects: Some(vec![FlagEffect::AltersFlow]), // Alters program control flow
        cycle_cost: Some(2), // Slightly more intensive
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x10),
    });

    // =========================
    // 📂 Flow/Invoke Instructions
    // =========================

    // `walk`: Calls a function or routine, in step with Micah 6:8’s call to walk humbly.
    registry.insert("walk", Instruction {
        keyword: "walk",
        verse_anchor: "Micah 6:8",
        traditional: &["CALL", "FUNC"],
        category: "Flow/Invoke",
        description: "Invoke a subroutine, function, or program.",
        opcode: 0x11,
        operand_format: Some("label"),
        machine_code: "11 XX",
        bit_mode: BitMode::Both,
        operand_count: Some(1),
        flags_effects: Some(vec![FlagEffect::AltersFlow]),
        cycle_cost: Some(3),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x10),
    });

    // =========================
    // 📂 IO Instructions
    // =========================

    // `speak`: Outputs data, a reflection of divine utterance (John 12:49).
    registry.insert("speak", Instruction {
        keyword: "speak",
        verse_anchor: "John 12:49",
        traditional: &["PRINT", "OUT"],
        category: "IO",
        description: "Output data to terminal or vocal system.",
        opcode: 0x20,
        operand_format: Some("value"), // Takes a value to print
        machine_code: "20 VV", // VV = value byte
        bit_mode: BitMode::Both,
        operand_count: Some(1),
        flags_effects: Some(vec![FlagEffect::Custom("OutputOperation")]), // Custom I/O effect
        cycle_cost: Some(2),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x20),
    });

    // `hear`: Accepts user or system input, echoing faith’s origin in hearing (Romans 10:17).
    registry.insert("hear", Instruction {
        keyword: "hear",
        verse_anchor: "Rom 10:17",
        traditional: &["INPUT"],
        category: "IO",
        description: "Receive user or system input.",
        opcode: 0x21,
        operand_format: Some("destination"), // Input destination (register or memory)
        machine_code: "21 DD", // DD = destination
        bit_mode: BitMode::Both,
        operand_count: Some(1),
        flags_effects: Some(vec![FlagEffect::ModifiesMemory]), // Writes input into memory
        cycle_cost: Some(3),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x20),
    });

    // =========================
    // 📂 Interrupt/Flow Instructions
    // =========================

    // `break`: Interrupts flow—symbolic of breaking bread and system cycles (Luke 24:30).
    registry.insert("break", Instruction {
        keyword: "break",
        verse_anchor: "Luke 24:30",
        traditional: &["INT", "BRK"],
        category: "Interrupt/Flow",
        description: "Exit from current loop, condition, or raise system-level interrupt.",
        opcode: 0x30,
        operand_format: None,
        machine_code: "30",
        bit_mode: BitMode::Both,
        operand_count: Some(0),
        flags_effects: Some(vec![FlagEffect::AltersFlow]),
        cycle_cost: Some(1),
        privilege_level: Some(PrivilegeLevel::Kernel), // Elevated instruction
        instruction_group_id: Some(0x30),
    });

    // =========================
    // 📂 Logic Structure Instructions
    // =========================

    // `then`: Defines the outcome when a condition is met.
    // Mirrors Proverbs 3:6 — "He shall direct thy paths."
    registry.insert("then", Instruction {
        keyword: "then", // Trigger for success path execution
        verse_anchor: "Prov 3:6", // Aligns logic to direction and obedience
        traditional: &["—"], // No traditional 1:1 equivalent
        category: "Logic Structure", // Syntax-level construct
        description: "Defines outcome when condition is met.",
        opcode: 0x40, // Assigned logic struct ID
        operand_format: None, // Instruction has no direct operand
        machine_code: "40", // Simple one-byte op
        bit_mode: BitMode::Both, // Universal in 32/64-bit
        operand_count: Some(0),
        flags_effects: None, // No flags or flow alterations on its own
        cycle_cost: Some(1),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x40),
    });

    // `else`: Defines alternate path when condition fails.
    // Anchored in Matthew 5:39 — “...resist not evil: but whosoever shall smite thee...”
    registry.insert("else", Instruction {
        keyword: "else",
        verse_anchor: "Matt 5:39",
        traditional: &["—"],
        category: "Logic Structure",
        description: "Defines alternate outcome if condition fails.",
        opcode: 0x41,
        operand_format: None,
        machine_code: "41",
        bit_mode: BitMode::Both,
        operand_count: Some(0),
        flags_effects: None,
        cycle_cost: Some(1),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x40),
    });

    // =========================
    // 📂 Logic/Control Instructions
    // =========================

    // `if`: Conditional logic instruction — compares two values.
    // Rooted in Matthew 4:3-4, testing response and decision.
    registry.insert("if", Instruction {
        keyword: "if",
        verse_anchor: "Matt 4:3-4",
        traditional: &["CMP", "JE"], // Similar to comparison or jump-if-equal
        category: "Logic/Control",
        description: "Conditional evaluation of a statement or expression.",
        opcode: 0x50,
        operand_format: Some("value1, value2"),
        machine_code: "50 VV1 VV2", // Comparison operands
        bit_mode: BitMode::Both,
        operand_count: Some(2),
        flags_effects: Some(vec![FlagEffect::SetsCondition]), // Implicit flag set for branching
        cycle_cost: Some(2),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x40),
    });

    // =========================
    // 📂 Math/Logic Instructions
    // =========================

    // `bless`: Increments a given value.
    // Scriptural root: Genesis 1:28 — “Be fruitful and multiply...”
    registry.insert("bless", Instruction {
        keyword: "bless",
        verse_anchor: "Gen 1:28",
        traditional: &["INC"], // Equivalent to increment
        category: "Math/Logic",
        description: "Increase a value or quantity.",
        opcode: 0x60,
        operand_format: Some("target"), // Target to increment
        machine_code: "60 TT", // TT = target
        bit_mode: BitMode::Both,
        operand_count: Some(1),
        flags_effects: Some(vec![FlagEffect::ModifiesMemory]),
        cycle_cost: Some(1),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x60),
    });

    // `curse`: Decrements a given value.
    // Tied to Genesis 3:17 — the consequence of disobedience.
    registry.insert("curse", Instruction {
        keyword: "curse",
        verse_anchor: "Gen 3:17",
        traditional: &["DEC"],
        category: "Math/Logic",
        description: "Decrease a value or apply limitation.",
        opcode: 0x61,
        operand_format: Some("target"),
        machine_code: "61 TT",
        bit_mode: BitMode::Both,
        operand_count: Some(1),
        flags_effects: Some(vec![FlagEffect::ModifiesMemory]),
        cycle_cost: Some(1),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x60),
    });

    // =========================
    // 📂 Memory Instructions
    // =========================

    // `store`: Saves a value into memory (register or stack).
    // Anchored in Deuteronomy 6:6–9 — storing the Word within.
    registry.insert("store", Instruction {
        keyword: "store", // Instruction keyword
        verse_anchor: "Deut 6:6–9", // Scriptural call to store truth
        traditional: &["PUSH", "STOR"], // Traditional mnemonic relatives
        category: "Memory",
        description: "Save data into stack or designated memory location.",
        opcode: 0x70, // Unique opcode ID
        operand_format: Some("target, value"), // Two operands: target location, value to store
        machine_code: "70 TT VV", // Target and value encoding
        bit_mode: BitMode::Both,
        operand_count: Some(2),
        flags_effects: Some(vec![FlagEffect::ModifiesMemory]),
        cycle_cost: Some(2),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x70),
    });

    // `recall`: Retrieves value from memory.
    // John 14:26 — “...bring all things to your remembrance...”
    registry.insert("recall", Instruction {
        keyword: "recall",
        verse_anchor: "John 14:26",
        traditional: &["POP", "LOAD"],
        category: "Memory",
        description: "Retrieve data from memory or archive.",
        opcode: 0x71,
        operand_format: Some("target"),
        machine_code: "71 TT",
        bit_mode: BitMode::Both,
        operand_count: Some(1),
        flags_effects: Some(vec![FlagEffect::ModifiesMemory]),
        cycle_cost: Some(2),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x70),
    });

    // =========================
    // 📂 Memory/Data Instructions
    // =========================

    // `let`: Assigns a value to a register or variable.
    // Rooted in Genesis 1:3 — “Let there be light.”
    registry.insert("let", Instruction {
        keyword: "let",
        verse_anchor: "Gen 1:3",
        traditional: &["MOV", "SET"],
        category: "Memory/Data",
        description: "Declare or assign a value to a variable or register.",
        opcode: 0x72,
        operand_format: Some("target, value"),
        machine_code: "72 TT VV",
        bit_mode: BitMode::Both,
        operand_count: Some(2),
        flags_effects: Some(vec![FlagEffect::ModifiesMemory]),
        cycle_cost: Some(1),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0x70),
    });

    // =========================
    // 📂 Structure Instructions
    // =========================

    // `end`: Marks the close of a function or scroll.
    // Rooted in Revelation 22:13 — the Alpha and Omega.
    registry.insert("end", Instruction {
        keyword: "end",
        verse_anchor: "Rev 22:13",
        traditional: &["RET", "END"],
        category: "Structure",
        description: "Terminates a block, function, or file.",
        opcode: 0xFF,
        operand_format: None,
        machine_code: "FF",
        bit_mode: BitMode::Both,
        operand_count: Some(0),
        flags_effects: Some(vec![FlagEffect::EndsFlow]),
        cycle_cost: Some(1),
        privilege_level: Some(PrivilegeLevel::User),
        instruction_group_id: Some(0xFF),
    });

    // Return the full registry after populating all instructions.
    registry

}

// ===================================================
// 🔚 Closing — Return & Scroll Integrity
// ===================================================
//
// ✅ Registry return ensures all instruction definitions
//    are gathered into a single system-wide HashMap.
//    This will feed into `Tablet` for parsing, compiling,
//    and future `.stone` generation.
//
// ⚠️ As of Phase 6, only MVP-level fields are fully used.
//    Extra fields are included for future extensibility
//    but not yet active in the compiler logic.
//
// 📌 This registry will become the backbone for NovaScript's
//    first operational pass—enabling true code-to-stone flow.
//
// ---------------------------------------------------
// 🧾 Change Policy Notice:
// ---------------------------------------------------
//   This file is governed by the OmniCode Scroll Protocol.
//   All structural changes (instruction shape, format rules,
//   or registry behavior) must be versioned and logged
//   with Watchtower if affecting output integrity.
//
//   Comments marked with ⚠️ or 📌 denote pivotal logic areas.
//
// ---------------------------------------------------
// 📅 Last Known Version
// ---------------------------------------------------
//   Version       : v0.0.1
//   Last Updated  : 2025-06-04
//   Change Log    :
//     - Phase 6 structure defined
//     - Flag, cycle, privilege, and group ID support added
//     - Overcommenting finalized for all MVP instructions
//
// ---------------------------------------------------
// 🪧 Notes
// ---------------------------------------------------
// - `.scroll` and `.scribe` output logic will use this registry.
// - Each instruction is spiritually aligned to biblical purpose.
// - Designed for clarity, expansion, and compiler faithfulness.
// - Compatible with `Gate`, `Tablet`, and `Watchtower` subsystems.
//
// ---------------------------------------------------
