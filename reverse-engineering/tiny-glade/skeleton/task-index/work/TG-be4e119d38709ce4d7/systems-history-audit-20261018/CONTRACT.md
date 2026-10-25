# B history replay mode contract (2026-10-18)

Current target is B only under the 2026-10-18 user scope. Claim families include C for ownership isolation, but no C address was submitted to the B database.

Observed: B PDB FunctionSystem type declares the 0x28-byte reader as `ModeJustChanged` (current AppMode 16B + previous AppMode 16B + event metadata), and the 0x10-byte writer as HistoryCmd, not WindowFocusEvent. Current workspace equivalent is AppModeChanged / HistoryCommand.

0x140a7c613..6c2: only last unread mode event matters; current Replay selects restoration mode from previous: tools 6..12/15..19 preserve full payload, Replay/Load keep existing RestoreAppTo, EditingTerrain(5) chooses Terrain(10), other editing/CloneClutter choose Editing(None). It resets ReplayModeElapsedTime to Some(0), replay speed=1, HoldToFastReplay=(false,0).

0x140a7c98b loads GameTime+0x28 as lerp argument3, 6.0 as argument2 and 0.2 as argument4. Existing code's replay.speed rate is wrong. Fast input writes four-byte HistoryCommand 0x03000000/0x03000100/0x03000200; fourth byte3 suppresses processed notification, commands still drive cursor.

Input ActionSetStates offsets +0x80/+0x88 are undo/redo slots16/17 (8B per digital state). Use current MainActionSet owner rather than independent stale counters; ReplayFastHoldInput remains override settings/compatibility projection only. Hold is a Resource as shown in original PDB, not Local.

Original UI signifier owns the 8B ReplayModeElapsedTime Option<f32> (separate exclusive claim TG-3830...). This delivery exposes/reset the observed resource; rendering/query effect remains outside this change. No binary runtime oracle was executed, only raw B RNE/patched byte identity and fresh Ghidra static review. Original fork/global schedule remains unverified.
