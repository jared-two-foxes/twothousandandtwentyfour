# 2048 TUI Backlog

## Project Context
- **Status**: Compiling successfully ✅
- **Platform**: Rust TUI using Ratatui + Crossterm
- **Game Model**: Exponents (1=2, 2=4, 3=8, ..., 11=2048)
- **Board Size**: Fixed 4x4 (no expansion planned)
- **Win Condition**: Reach exponent 11 (2048), with option to continue playing
- **Loss Condition**: No empty cells and no adjacent equal tiles

---

## 1. Complete Core Grid Support

**Priority: HIGH** — Unimplemented panic path blocks valid-move detection.

### 1.1 Implement Grid::columns()
- **File**: [src/grid.rs](src/grid.rs#L197)
- **Issue**: `unimplemented!()` blocks column-based adjacent-equal checks
- **Fix**: Return `Vec<Column<'a, T>>` analogous to `rows()`
- **Impact**: Runtime panic in [src/model.rs](src/model.rs#L45) via `adjacent_by_column()`
- **Status**: ✅ Completed

### 1.2 Add bounds assertion in Grid::swap()
- **File**: [src/grid.rs](src/grid.rs#L234)
- **Issue**: TODO comment notes missing validation
- **Fix**: Add asserts for valid row/col indices before swap
- **Impact**: Better error messages on out-of-bounds access
- **Status**: ✅ Completed

---

## 2. Lock In Exponent-Based Game Model

**Priority: HIGH** — Clarifies tile representation; unblocks scoring and rendering.

### 2.1 Confirm tile storage as exponents throughout
- **Files**: [src/actions.rs](src/actions.rs#L106), [src/model.rs](src/model.rs#L73)
- **Behavior**: Tiles are stored as exponents (1 = 2^1, 2 = 2^2, etc.; 0 = empty)
- **Status**: ✅ Completed (validated by unit tests in [src/actions.rs](src/actions.rs) and [src/model.rs](src/model.rs))

### 2.2 Render exponents as visible powers in TUI
- **File**: [src/bin/cli/view.rs](src/bin/cli/view.rs#L33)
- **Fix**: Convert exponent to displayed value: `if exp == 0 { "" } else { format!("{}", 2u32.pow(exp as u32)) }`
- **Impact**: Tiles display correctly as 2, 4, 8, 16, ..., 2048
- **Status**: ✅ Completed

### 2.3 Ensure score reflects *displayed* tile value
- **File**: [src/actions.rs](src/actions.rs#L26)
- **Issue**: Score accumulation likely uses raw exponent, not 2^n
- **Fix**: When merging at exponent k, add `2^k` to score, not k
- **Impact**: Score is correct and matches player expectations
- **Status**: ✅ Completed

---

## 3. Fix Game-State Transitions

**Priority: CRITICAL** — Current logic inverts win/loss; game is unplayable.

### 3.1 Correct done-condition logic
- **File**: [src/actions.rs](src/actions.rs#L31)
- **Current**: `if highest_tile(model) == 11 || model.check_for_valid_moves() { state = Done }`
- **Issue**: Sets Done when valid moves exist (backwards); no distinction between win/loss
- **Fix**:
  - Set state to Won if `highest_tile == 11` (2048 reached)
  - Set state to Lost if `!check_for_valid_moves()` (no moves available)
  - Otherwise stay Running
- **Status**: ✅ Completed

### 3.2 Extend State enum
- **File**: [src/model.rs](src/model.rs#L6)
- **Current**: `enum State { Running, Done }`
- **Change**: `enum State { Running, Won, WonContinue, Lost }`
  - `Won`: Player reached 2048; offer continue or reset
  - `WonContinue`: Player chose to keep playing after winning
  - `Lost`: No moves; game over
- **Impact**: Enables win-screen UX and continue-game flow
- **Status**: ✅ Completed

### 3.3 Wire state into CLI loop
- **File**: [src/bin/cli/main.rs](src/bin/cli/main.rs)
- **Issue**: Model state transitions are not reflected in the app loop
- **Fix**: Check `app.model.state` in main loop; stop input processing if Won/Lost (unless WonContinue)
- **Status**: ✅ Completed

---

## 4. Add Win Screen with Continue Option

**Priority: HIGH** — Core user-facing win/loss experience.

### 4.1 Extend event handling for win/loss states
- **File**: [src/bin/cli/main.rs](src/bin/cli/main.rs#L68)
- **Changes**:
  - If state is Won, listen for 'c' (continue) or 'q' (quit) or 'r' (restart)
  - If state is WonContinue, treat like Running (allow moves)
  - If state is Lost, listen for 'r' (restart) or 'q' (quit)
- **Status**: ✅ Completed

### 4.2 Add win/loss overlay to view
- **File**: [src/bin/cli/view.rs](src/bin/cli/view.rs#L6)
- **Add**:
  - Conditional rendering of game board or win/loss screen
  - Win screen: "You reached 2048! [C]ontinue or [Q]uit?"
  - Loss screen: "Game Over. No moves left. [R]estart or [Q]uit?"
- **Status**: ✅ Completed

### 4.3 Support restart functionality
- **File**: [src/bin/cli/main.rs](src/bin/cli/main.rs)
- **Add**: Handle 'r' key to reset `app.model = Model::new()` and `app.should_quit = false`
- **Status**: ✅ Completed

---

## 5. Enforce Correct Move Semantics

**Priority: HIGH** — Spawn-on-every-input is incorrect 2048 behavior.

### 5.1 Only spawn a new tile if board changed
- **File**: [src/actions.rs](src/actions.rs#L29)
- **Current**: Always calls `model.generate_new_value()` after compress
- **Fix**:
  - Compare board state before/after compress
  - Only spawn if state changed (value > 0 or any cell moved)
  - Return early or skip spawn on no-op move
- **Impact**: No-op moves (e.g., pressing left when all tiles are left-aligned) do not consume a turn
- **Status**: ✅ Completed

### 5.2 Verify fixed 4×4 assumptions are intentional
- **Files**: [src/model.rs](src/model.rs#L35), [src/model.rs](src/model.rs#L36)
- **Current**: Hardcoded `for i in 0..4`
- **Action**: Add a comment confirming board is always 4×4; consider using `self.grid.width()` for clarity
- **Status**: Not started

---

## 6. Finish CLI UX Essentials

**Priority: MEDIUM** — Playable but basic; improves UX significantly.

### 6.1 Display score and high score
- **File**: [src/bin/cli/view.rs](src/bin/cli/view.rs#L6)
- **Add**: Header or footer rendering score, high score, and board info
- **Note**: High score field exists in [src/bin/cli/main.rs](src/bin/cli/main.rs#L10) but is never updated or displayed
- **Status**: ✅ Completed

### 6.2 Update high score on game end
- **File**: [src/bin/cli/main.rs](src/bin/cli/main.rs)
- **Add**: When state becomes Lost/Won, update `app.high_score = app.high_score.max(app.model.score)`
- **Status**: ✅ Completed

### 6.3 Improve tile styling and colors
- **File**: [src/bin/cli/view.rs](src/bin/cli/view.rs#L23)
- **Add**: Color-code tiles by exponent (e.g., red for 2048, yellow for 1024, green for lower)
- **Use**: Ratatui's Style/Color for per-exponent coloring
- **Status**: Not started

### 6.4 Add help footer
- **File**: [src/bin/cli/view.rs](src/bin/cli/view.rs#L6)
- **Add**: Small line showing: "↑↓←→ Move | [Q]uit | [C]ontinue (after win) | [R]estart"
- **Status**: ✅ Completed

---

## 7. Replace Duplicate and Dead Logic

**Priority: MEDIUM** — Code quality and maintainability.

### 7.1 Remove or integrate has_valid_move()
- **File**: [src/model.rs](src/model.rs#L98)
- **Issue**: Duplicate of `check_for_valid_moves()`; contains broken typo and private-field access
- **Fix**: Delete `has_valid_move()` entirely; use `check_for_valid_moves()` as single source of truth
- **Status**: Not started

---

## 8. Rebuild Test Coverage

**Priority: HIGH** — No regression safety; tests are currently broken.

### 8.1 Fix and update existing compress tests
- **File**: [src/actions.rs](src/actions.rs#L177)
- **Task**:
  - Update test setup to use current Model/Grid APIs
  - Verify expected outputs use exponent representation (e.g., 2 = exponent 1, 4 = exponent 2)
  - Run and pass all existing test cases
- **Status**: Not started

### 8.2 Add no-op move test
- **Behavior**: Move in same direction twice; second move should spawn nothing
- **Test**: Board state + score remain same after no-op
- **Status**: Not started

### 8.3 Add merge-once-per-move test
- **Behavior**: A row of [1, 1, 1, 0] compressed left should yield [2, 1, 0, 0], not [2, 0, 0, 0]
- **Test**: Verify only one merge occurs per value per move
- **Status**: Not started

### 8.4 Add win detection test
- **Behavior**: Reaching exponent 11 (2048) sets state to Won
- **Test**: Generate board with exponent 11, perform move, verify state = Won
- **Status**: Not started

### 8.5 Add loss detection test
- **Behavior**: Board full, no adjacent equals, no moves available
- **Test**: Construct full board with no merges, verify state = Lost after attempting move
- **Status**: Not started

### 8.6 Add score correctness test
- **Behavior**: Merging two exponent-2 tiles (4) yields exponent-3 (8) and score += 8
- **Test**: Verify score += 2^exponent, not raw exponent
- **Status**: Not started

### 8.7 Add state transition test
- **Behavior**: Game starts in Running; moves transition to Won or Lost
- **Test**: Verify state machine: Running → Won → WonContinue (or Lost → end)
- **Status**: Not started

---

## 9. Final Hardening

**Priority: LOW** — Polish before release.

### 9.1 Run full test suite
- **Command**: `cargo test`
- **Target**: All tests pass with no warnings
- **Status**: Not started

### 9.2 Manual playthrough script
- **Steps**:
  1. Start game
  2. Play until reaching 2048
  3. Choose continue
  4. Play until loss
  5. Restart and quit
- **Verify**: All features working end-to-end
- **Status**: Not started

### 9.3 Clean up TODO and FIXME comments
- **Files**: All source files
- **Action**: Remove inline TODOs; replace with documented decisions or close resolved items
- **Status**: Not started

### 9.4 Review naming and comments
- **Clarify**:
  - Message enums (currently has TODO about renaming to ApplicaitonMessage)
  - Confusing variable names
  - Ensure exponent-based model is clearly documented in code
- **Status**: Not started

---

## Implementation Order (Recommended)

1. **Week 1: Core Grid & State Logic** (Items 1-3)
   - Items 1.1–1.2 (implement Grid API)
   - Items 2.1–2.3 (exponent consistency)
   - Items 3.1–3.3 (fix state transitions and wire into CLI)

2. **Week 2: Game Mechanics & UX** (Items 4-6)
   - Items 4.1–4.3 (win/continue/loss UI)
   - Items 5.1–5.2 (enforce move semantics)
   - Items 6.1–6.4 (score display, styling, help)

3. **Week 3: Cleanup & Testing** (Items 7-9)
   - Item 7.1 (remove dead code)
   - Items 8.1–8.7 (rebuild tests)
   - Item 9 (hardening & polish)

---

## Notes

- **Exponent Model**: Tiles stored as small integers (1–11); displayed as 2^n; score uses 2^n.
- **Fixed Size**: No dynamic board resizing needed; 4×4 is constant.
- **Continue Flow**: After winning at 2048, player can choose to keep playing or quit/restart.
- **High Score**: Tracked across games; updated when current score exceeds it at loss/end.
