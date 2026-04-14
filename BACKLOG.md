# 2048 TUI Backlog

## Project Context
- **Platform**: Rust TUI using Ratatui + Crossterm
- **Game Model**: Exponents (1=2, 2=4, 3=8, ..., 11=2048)
- **Board Size**: Fixed 4x4 (no expansion planned)
- **Win Condition**: Reach exponent 11 (2048), with option to continue playing
- **Loss Condition**: No empty cells and no adjacent equal tiles

---

## 9. Final Hardening

**Priority: LOW**

### 9.1 Run full test suite
- **Command**: `cargo test`
- **Target**: All tests pass with no warnings
- **Status**: Not started

### 9.2 Manual playthrough
- **Steps**: Start → play to 2048 → continue → lose → restart → quit
- **Verify**: All features working end-to-end
- **Status**: Not started

### 9.3 Clean up TODO and FIXME comments
- **Files**: All source files
- **Action**: Remove inline TODOs; replace with documented decisions or close resolved items
- **Status**: Completed

### 9.4 Review naming and comments
- **Clarify**: Message enum names (TODO about renaming to `ApplicationMessage`), confusing variable names, exponent-model documentation
- **Status**: Completed

---

## Notes

- **Exponent Model**: Tiles stored as small integers (1–11); displayed as 2^n; score uses 2^n.
- **Fixed Size**: No dynamic board resizing needed; 4×4 is constant.
- **Continue Flow**: After winning at 2048, player can choose to keep playing or quit/restart.
- **High Score**: Tracked across games; updated when current score exceeds it at loss/end.
