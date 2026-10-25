# B input consumers implemented

Original RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. PDB signatures are in signatures.json; original instruction slices in delta-first-slots.asm and cursor.asm.

- calculate_input_delta: ActionSetValues owns previous/current bool-valued sets. All six analog slots copy directly into the existing MainActionSet owner. Forty digital slots calculate press/hold/release/repeat. 0.1/0.05 replenishes repeat timers; it is not an analog deadzone. Real clock is bevy_time::Time<()>, not paused GameTime. Inactive enum payloads normalize to zero and do not claim native padding bits.
- translate_input_to_cursor_moved: actual EventReader<InputCursorMoved> and EventWriter<InputCursorDelta>; free cursor applies relative events with per-event window clamping and absolute events directly, ignores tag2. Captured cursor sums tags0/2 and ignores tag1. One output event is emitted even with no input before the two ActionSetValues accumulators update. No integer cursor cast or window-size-minus-one behavior was introduced. Explicit comparison preserves the native signed-zero clamp.
- Independent roof agent review confirmed the saved digital branches and cursor branches/event order. After rebuilding, all 5 focused input tests passed, including the signed-zero/NaN clamp regression.
- These are real CPU/ECS consumers, not the complete device pipeline. Keyboard/Steam mapping, reset/replay integration, update_cursor_position and the original global ordering remain open. mod-key.asm is evidence only; no stub mod_key module was added.
