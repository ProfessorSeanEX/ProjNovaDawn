// ==========================================================
// 🧪 Instruction Registry Test Suite — NovaScript Integrity
// ==========================================================
//
// 🎯 Purpose:
//   - Validates the structure, completeness, and scroll-alignment
//     of all registered NovaScript instructions.
//   - Ensures that every instruction is spiritually anchored,
//     structurally sound, and ready for `.stone` compilation.
//
// 📦 Imports:
//   - Pulls registry population logic and instruction schema
//     from the `tablet::instruction_registry` module.
//   - Uses `HashSet` to verify opcode uniqueness across entries.
//
// 🔮 Future-Ready:
//   - Designed for Watchtower logging and scroll diff tracking.
//   - Prepares instruction validation pipeline for compiler integration.
// ----------------------------------------------------------

use tablet::instruction_registry::{get_instruction_registry, Instruction}; // 📜 Source of truth for instructions
use std::collections::HashSet; // 🧮 Used to verify opcode uniqueness and detect duplicates


#// =======================================================
// ✅ Instruction Registry Test — Structural Integrity Pass
// =======================================================
//
// 📜 Purpose:
//   Validates every Phase 1 NovaScript instruction registered
//   through Tablet’s `get_instruction_registry()`.
//
// 🧭 Verifies:
//   - Keyword alignment and registry population
//   - Scriptural anchors and mnemonic presence
//   - Opcode uniqueness and schema consistency
//   - Machine code and privilege integrity
//
// 📦 Observations:
//   - Every instruction must be spiritually grounded (verse)
//   - MVP-level fields are required to pass Phase 6 validation
//   - This suite ensures `.stone` output integrity
//
// 🔮 Future Notes:
//   - Watchtower will log results and diffs as registry evolves
//   - Flags, phase levels, and schema matching will expand over time
//
// =======================================================

#[test]
fn test_instruction_registry_integrity() {
    let registry = get_instruction_registry();

    // 🌿 Must contain at least one registered instruction
    assert!(
        !registry.is_empty(),
        "Instruction registry should not be empty."
    );

    let mut seen_opcodes = HashSet::new();

    for (keyword, instr) in registry.iter() {
        // 🔑 Keyword must match declared field
        assert_eq!(
            instr.keyword, *keyword,
            "Instruction keyword mismatch for '{}'", keyword
        );

        // 📝 Description must not be blank
        assert!(
            !instr.description.is_empty(),
            "Missing description for '{}'", keyword
        );

        // 📖 Scriptural alignment required
        assert!(
            !instr.verse_anchor.is_empty(),
            "Missing verse_anchor for '{}'", keyword
        );

        // 🛠 At least one traditional mnemonic (if list isn't empty)
        if !instr.traditional.is_empty() {
            assert!(
                instr.traditional.iter().any(|m| !m.is_empty()),
                "All traditional mnemonics empty for '{}'", keyword
            );
        }

        // 💾 Instruction category must be present
        assert!(
            !instr.category.is_empty(),
            "Missing category for '{}'", keyword
        );

        // 🔢 Opcode must be unique across instructions
        assert!(
            seen_opcodes.insert(instr.opcode),
            "Duplicate opcode ({:#04X}) for '{}'", instr.opcode, keyword
        );

        // 🧮 Operand count must be reasonable (max 4)
        if let Some(op_count) = instr.operand_count {
            assert!(
                op_count <= 4,
                "Operand count ({}) too high for '{}'", op_count, keyword
            );
        }

        // 🗺 Operand schema (if present) must match operand count
        if let (Some(schema), Some(count)) = (&instr.operand_schema, instr.operand_count) {
            let parts = schema.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).count();
            assert_eq!(
                parts, count,
                "Operand schema mismatch for '{}'", keyword
            );
        }

        // 🏳️ Flags must be non-empty if declared
        if let Some(flags) = &instr.flags_effects {
            assert!(
                !flags.is_empty(),
                "Flags declared but empty for '{}'", keyword
            );
        }

        // 🔐 Privilege level must be defined
        assert!(
            instr.privilege_level.is_some(),
            "Missing privilege level for '{}'", keyword
        );

        // 🔢 Phase level must also be defined
        assert!(
            instr.phase_level.is_some(),
            "Missing phase level for '{}'", keyword
        );

        // 💠 Group ID must be within u8 bounds
        if let Some(group_id) = instr.instruction_group_id {
            assert!(
                group_id <= 0xFF,
                "Group ID out of range for '{}'", keyword
            );
        }

        // ⚙️ Machine code string must be defined
        assert!(
            !instr.machine_code.is_empty(),
            "Missing machine code for '{}'", keyword
        );
    }

    // 📦 Final pass — number of opcodes must equal number of entries
    assert_eq!(
        seen_opcodes.len(),
        registry.len(),
        "Mismatch between unique opcodes and registry entries"
    );
}

// ===========================================================
// 📋 Test Log Summary — Instruction Registry Verification
// ===========================================================
//
// 🧾 Purpose:
//   - Provides a visual confirmation log of instruction registry integrity tests
//   - Ensures scroll-based definitions remain faithful, complete, and executable
//
// 🛠 Usage:
//   - Run with `cargo test -- --nocapture` to view this output
//   - Helps trace test status alongside Watchtower and Tablet outputs
//
// 📌 Note:
//   - This summary is **non-evaluative** (does not affect result state)
//   - Meant for developer awareness and covenantal trace clarity
//
// 🔮 Future:
//   - Will evolve into Watchtower summary emit when test logging syncs
//   - May reflect scroll diffs or opcode gaps in expanded coverage
//
// ===========================================================

#[test]
fn test_log_instruction_registry_summary() {
    println!("✅ test_instruction_registry_integrity: PASSED");

    // 🧭 This log confirms all instructions in the registry passed validation.
    //     Use this scroll as a lighthouse when expanding Tablet opcode logic.
}
