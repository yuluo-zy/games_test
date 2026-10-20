# Original reveal event producers — fresh native identity

The previous `WriteSettingsFileCmd` attribution was incorrect. This is established by complete 128-bit resource TypeIds, separately anchored by `EventRegistry::register_event<T>` and `Res<Events<T>>::init_state`, then matched against the original writer instructions. No load-game completion event or request flag is substituted.

| Event | Registration / independent init_state | Native Events resource TypeId | Actual producer |
|---|---|---|---|
| StartupRevealStarted | 140578b80 / 141343b80 | 76eef710ca5ee797,804a5169b0f39613 | main_loop1404d9580: lookup1404da0b5, write1404da14b..15a |
| StartupRevealFinished | 1405734e0 / 141343510 | d8f93d064ea9d61c,6447617066d9832f | vermeer_pass1408a82f0: lookup1408a89e9, write1408a8a14..a8a; run_game_thread1405a42b0 lookup, write1405a4371..80 |

Started follows the real startup/worker SubApp handoff gate: LogoAnimation.finish_wait140b19d30 and is_finished140b19d20, take actual worker result, move six real resources, send Started, move VermeerState through into_disable_hold1408a81e0, move EguiContext, then replace SubApp. Existing `transfer_loading_resources` has exactly this position but the wrong unit-event type; the independent one-hunk patch replaces that type only. It does not wait for a fabricated request_reveal.

Finished in Vermeer follows the existing native progress>=1, !freeze and !hold condition. It drops background, changes phase to Disabled, appends one unit event and increments the Events counter, then logs completion. Current `run_vermeer` has no Events owner wired at all and the old adapter's callback returns None; a real completion would panic. The three-file candidate gives the actual Events<StartupRevealFinished> owner to EffectsHost through resource_scope and renames the incorrect trait method/type constant. Existing nonempty threshold/hold/freeze/missing-resource tests are retained, corrected to the actual event, and assert no second completion event after Disabled.

The original process static143634c9c is read at1408a8448 and set true at1408a844f; the disabled/no-restart path clears it at1408a88d0. Current host resets its projection to false every frame, breaking the original `previous_active & target_reused` conjunction. The candidate retains the canonical VermeerPassLocals owner across frames, initially false, preserving the original algorithm.

`run_game_thread1405a42b0..4380` also writes a real Finished before moving the prepared App into its final host object. This separate original writer has been identified after freezing the three-file patch. Its complete prerequisite/control path is not yet restored by this patch; central integration must audit it rather than emit an unconditional extra event during arbitrary load completion.

Artifacts: `reveal-finished-candidate.patch` (three files), `reveal-finished-candidate-manifest.json`, `reveal-started-handoff-hunk.patch`, `verify-reveal-forest-original.py`, `reveal-forest-original.{json,asm}`. The original JSON retains raw bytes, full function hashes and all native references to both TypeId words. Original RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.

Validation performed: three full candidate Rust files parsed with rustfmt; git apply --check passed against current production; no production writes and no Cargo/runtime claim. Central nonempty regression must exercise the actual handoff, actual active Vermeer frame through completion with real event resource and recording audio consumer, repeated disabled frame, hold, debug freeze and a second session. First startup alone cannot establish the second-session lifetime.

Forest lifecycle rechecked against new raw bytes: init_clearing140a79034..bd clears existing80B records including their Arc drops, then sets the same placement owner+18=false. manage141bf1f30 reads it;141bf2588 writes true. Unload clears it. Canonical processed ownership is already integrated, but the init producer and actual80B payload/default-prefab/entity writeback still require complete recovery; clearing the map by itself does not recover those branches.
