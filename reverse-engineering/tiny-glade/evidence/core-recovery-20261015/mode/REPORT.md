# AppMode / Toolbar / Mode manager recovery — 2026-10-15

## Implemented

- `systems/mode_manager/types.rs`: real AppMode (16 top-level variants) and EditSubject (6 variants), WallShape, EraserLayer, DecoratorType, all 56 ClutterMeshTy variants. Serde uses original variant and field spelling. Native tag access is an explicit semantic mapping; Rust host ABI equivalence is not claimed.
- `resources/toolbar.rs`: 22 ToolbarCategory variants from serde table; full 11-category constructor for both full/demo editions; remembered selection, wrap/cycle, empty category preservation, full-payload equality, first matching category/item selection, original absent-mode panic.
- `systems/mode_manager.rs`: complete observed ordering: UI early return → last history request → undo then redo → Replay early return → last load completion → last AppMode request → last category request → building/path/decorator/customization/eraser/terrain/garden/tree/water shortcuts → photomode toggle → time-of-day toggle → clear-canvas pressed. Real ECS resources and Events are callable through configure_mode_manager.
- Uses the existing `HistoryTransactions<M,S>` resource itself; is_recording consumes its pending owner, no copied parallel history ledger. AppMode is Resource and roof consumers can use it directly.
- Adds explicit TimeOfDayScreen visibility projection to GUI. Caller supplies initial state; no unverified default is fabricated.

## Corrected old reconstruction errors

1. The first 4-byte event is RequestHistoryCommand; RequestAppModeChange is 16 bytes (24-byte EventInstance).
2. `cmp 0,[History+0x30]; setno` is exactly History::is_recording at 0x140a86970. The None niche is i64::MIN, for which subtraction overflows. The former assertion that this guard is always true was wrong.
3. There are thirteen adjacent 5-entry DigitalActionState jump tables, not a 64-entry AppMode dispatch. Their common pattern is tag0 => true, tag2 => 0.0>=timer, other => false. Reuses the existing just_pressed implementation (including NaN behavior).
4. AppMode native tags 0..5 are Editing's nested discriminants: None/Wall/Roof/DecoClutter/StairPoint/Terrain. tag2 does not mean Timelapse. Wall/Path/Cobblestone/Eraser/Terrain/Decorator/Color/Replay/LoadGame/Water/Tree/Garden/Fences/Photomode/CloneClutter are 6..20.
5. The old ModeEntry split 16-byte record into meaningless payload chunks and compared padding. It has been removed in favor of real enums and derived equality. The tag21 sentinel belongs to Option<AppMode>; it is not a fake toolbar mode.
6. Toolbar category10 is Photomode, category6 is Terrain; the old category labels were incorrect.

## Evidence

Original oracle: `workspace/input/tiny-glade.exe` SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; inspect_modes.py asserts it. B PDB is `D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb`. The existing TPI does not contain complete Rust layouts; no PDB void() signature was treated as a real ABI.

- `procedures.json`: exact PDB S_GPROC/LPROC bounds and names for the targeted functions. Actual mode_manager system generic PDB signature proves all 15 resource/event parameters; x64 prologue maps their locations to stack+c0..110.
- `0x140afc7c0.asm`: AppMode serde; normalized top-level dispatch `0x142ae61ec` produces tags noted above; u64 Cobblestone/Wall/Roof and u32 editing IDs are read at their observed payload locations.
- `0x140afde20.asm`: all AppMode serialized names, 16-entry expected list.
- `0x140afc4a0.asm`, `0x140afbe10.asm`: editing and eraser serialized names.
- `0x140a478f0.asm`, `0x140a48880.asm`: DecoratorType has 8 outer variants; Clutter is a struct variant with field `ty`. Native outer unit variants are bytes56..61 and63; nested ClutterMeshTy occupies0..55.
- `0x14179b870.asm` expected variants `0x142cda7c8`, 56 names; saved as clutter-names.json. Serde names intentionally retain snake_case.
- `0x140afb5b0.asm` expected list `0x142ae5888`, 22 category names saved as category-names.json.
- `0x140af8c50.asm`: complete constructor and edition differences. Full category lengths `[5,4,6,1,6,1,1,1,1,1,1]`, demo `[3,1,2,1,4,0,1,1,0,1,1]`.
- `0x140af89d0.asm`, `0x140af9850.asm`, `0x140a825e0.asm`: category membership, first match and per-variant equality.
- `0x140a86b40.asm`: is_tool_selected mask0x3cfe.
- `0x140a86b60.asm`, `0x140afa2c0.asm`, `0x140afa770.asm`, `0x140afa9f0.asm`: mode-manager and special closures. Loading sets AppMode=Replay and writes the target into RestoreAppTo, then continues; it does not copy the target back into AppMode.
- `history-is-recording.asm`, `history-interrupt.asm`: direct original small function reads.
- `claude-mode.result.json`: configured CLI via updated helper returned ok=true. Used as a second reading of parameter slots and table grouping. Its statement implying the loaded tag was written to AppMode was rejected after tracing the destination ResMut pointer to RestoreAppTo. Its suggestion that History guard might be constant was rejected by the exact is_recording function and Option niche.

## Review and verification

Manually compared every mode-manager basic block, thirteen jump-table patterns, constructor category stores and special category closures to source. Added six ECS/serde tests plus two toolbar tests. Central parent owns compilation/test execution; this report does not assert their result until the parent records it. No database changes, dependency reimplementation, test-only algorithm substitute or no-op callbacks.

## Remaining integration boundaries (not falsely called complete)

- Full UI owner UiState is not restored here; ModeManagerUiContext is an explicit read projection and needs its actual canvas ID. Configurator requires caller-provided state; it does not invent a hash or input.
- TimeOfDayScreen currently exposes only the proven visibility consumer field; full snapshot/editor owner remains GUI work.
- HistoryCommand preserves all four native bytes and exact known constructors. Full history command enum/process ownership is being integrated by the roof/history worker. UiHistoryPlaySelectAnimation and ClearCanvasCmd are real output Events.
- PlayUiSound's full enum/audio playback is not restored here. ModeManagerSound emits the two observed roles with native tags4 and14 for the real audio owner to consume.
- The game bootstrap and original global scheduling remain outside this file. configure_mode_manager installs callable consumers; caller owns standard Bevy event frame updates.
- AppMode nested ID values retain their verified integer payloads; resource-specific ID newtypes can replace these transparently once their owning modules are canonicalized. No binary ABI promise is made.
