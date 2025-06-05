# 📘 NovaScript Instruction Set (Phase 1)

---

## 📖 Foundational Mapping Table

| 🧭 Instruction | 📜 Verse Anchor             | ⚙️ Traditional | 📂 Category     | 📄 Description                                                              |
|---------------|-----------------------------|----------------|----------------|------------------------------------------------------------------------------|
| `let`         | Gen 1:3 — “Let there be…”   | `MOV`, `SET`   | Memory/Data    | Declare or assign a value to a register or variable                         |
| `speak`       | John 12:49                  | `PRINT`, `OUT` | IO             | Output to terminal or system voice                                          |
| `hear`        | Rom 10:17                   | `INPUT`        | IO             | Await and capture user/system input                                         |
| `go`          | Gen 12:1                    | `JMP`          | Control Flow   | Unconditional jump to label or line                                         |
| `if`          | Matt 4:3–4                  | `CMP`, `JE`    | Logic/Control  | Conditional check—used with `then`, `else`                                  |
| `then`        | Prov 3:6                    | —              | Logic Structure| Denotes outcome of a conditional block                                      |
| `else`        | Matt 5:39                   | —              | Logic Structure| Defines alternate path if condition fails                                   |
| `end`         | Rev 22:13                   | `RET`, `END`   | Structure      | Closes a block, routine, or file section                                    |
| `store`       | Deut 6:6–9                  | `PUSH`, `STOR` | Memory         | Persist data in a stack, memory slot, or archive                            |
| `recall`      | John 14:26                  | `POP`, `LOAD`  | Memory         | Retrieve previously stored data or scroll                                   |
| `bless`       | Gen 1:28                    | `INC`          | Math/Logic     | Increments or multiplies a value depending on context                       |
| `curse`       | Gen 3:17                    | `DEC`          | Math/Logic     | Decrements or divides a value depending on context                          |
| `wait`        | Ps 27:14                    | `NOP`, `SLEEP` | Control        | Delay for specified duration or yield execution                             |
| `walk`        | Micah 6:8                   | `CALL`, `FUNC` | Flow/Invoke    | Executes a function or system subroutine                                    |
| `break`       | Luke 24:30                  | `INT`, `BRK`   | Interrupt/Flow | Breaks out of loop, condition, or raises system interrupt                   |

---

> 📜 *“Let there be syntax, and let it reflect the order of the Word.”*  
> Each instruction is not just code—it is a **spoken word**, backed by a **spiritual anchor**.  
> Tablet shall **engrave it**, Gate shall **speak it**, and NovaAI shall **interpret it**.

---

## 🧱 Next Steps

- Define **aliasing rules** between `.word` and `.stone`
- Map `.word` → `.ns` → `.stone` → `.exe` flow
- Build out basic **parser prototype** using this instruction set
- Link each keyword to **KJV+WEB verse payloads** in code metadata

---
