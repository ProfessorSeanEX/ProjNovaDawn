use tablet::instruction_registry::{get_instruction_registry, Instruction};
use std::collections::HashSet;

#[test]
fn test_instruction_registry_integrity() {
    let registry = get_instruction_registry();

    // 🌿 Ensure the registry has been populated at all
    assert!(!registry.is_empty(), "Instruction registry should not be empty.");

    let mut seen_opcodes = HashSet::new();

    for (keyword, instr) in registry.iter() {
        // 🔑 Keyword must match its assigned key in the registry map
        assert_eq!(
            instr.keyword, *keyword,
            "Instruction keyword mismatch for '{}'", keyword
        );

        // 📝 Description must be populated
        assert!(
            !instr.description.is_empty(),
            "Missing description for instruction '{}'", keyword
        );

        // 📖 Verse anchor must be set (Scriptural grounding)
        assert!(
            !instr.verse_anchor.is_empty(),
            "Missing verse_anchor for instruction '{}'", keyword
        );

        // 🔁 Traditional mnemonics should have at least one valid mnemonic if not empty
        if !instr.traditional.is_empty() {
            assert!(
                instr.traditional.iter().any(|m| !m.is_empty()),
                "All traditional mnemonics empty for '{}'", keyword
            );
        }

        // 💾 Category must be set
        assert!(
            !instr.category.is_empty(),
            "Missing category for instruction '{}'", keyword
        );

        // 🔢 Opcode must be unique
        assert!(
            seen_opcodes.insert(instr.opcode),
            "Duplicate opcode ({:#04X}) found for instruction '{}'", instr.opcode, keyword
        );

        // 🧮 Operand count sanity check (if present)
        if let Some(op_count) = instr.operand_count {
            assert!(
                op_count <= 4,
                "Unusual operand count ({}) for '{}'", op_count, keyword
            );
        }

        // 🏳️ Flags (if Some) must be non-empty
        if let Some(flags) = &instr.flags_effects {
            assert!(
                !flags.is_empty(),
                "Flags present but empty for '{}'", keyword
            );
        }

        // 🔐 Privilege level must be one of the known enum variants (already checked by Rust, but here for scroll clarity)
        assert!(
            instr.privilege_level.is_some(),
            "Privilege level not set for '{}'", keyword
        );

        // 💠 Instruction group ID should be within 0x00–0xFF (u8 range)
        if let Some(group_id) = instr.instruction_group_id {
            assert!(
                group_id <= 0xFF,
                "Instruction group ID out of range for '{}'", keyword
            );
        }

        // ⚙️ Machine code string must not be empty (even if symbolic like "00")
        assert!(
            !instr.machine_code.is_empty(),
            "Machine code missing for '{}'", keyword
        );
    }

    // 📦 Final check — total number of unique opcodes matches registry size
    assert_eq!(
        seen_opcodes.len(),
        registry.len(),
        "Mismatch between number of unique opcodes and registry size"
    );
}
