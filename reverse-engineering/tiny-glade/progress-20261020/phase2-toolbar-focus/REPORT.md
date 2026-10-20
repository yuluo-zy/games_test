# Focus producer / visibility / event lifecycle recovery

Original B EXE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
Source original identities: evidence/country-core-audit/data/procs.tsv.

Four complete original functions recovered:

- ui_focus_parent_if_self_in_focus 14233dcb0 /2724B. Initial mutable nested query identifies any nonzero keep, inserts self into a per-update visited Entity set and queues its parent. Each breadth-first frontier first updates roots (Without<UiShowIfParentIsInFocus>), then nested ancestors; a newly visited nested ancestor queues its parent. Missing/stale ancestors are skipped, cycle membership terminates ascent. The original initial list may contain duplicates, which are preserved. The set is not a permanent cross-frame cache.
- ui_show_if_parent_in_focus 14233ce00 /2164B. Child state byte is copied **from parent keep**, not parent state/selected. Ordinary items preserve all byte values. ShowOnlyInFpvAndFree restricts to camera_mode1/2 of the canonical PhotomodeState and normalizes nonzero parent keep to bool1. Both missing parent state and missing child state preserve the original unwrap failure. This is independent of AppMode.
- ui_send_focus_begin_events 14233a170 /1652B. Persistent Local<HashSet<Entity>> inserts every nonzero keep and removes queried zero keep. New membership emits only for exact keep1. Existing keep2 -> keep1 therefore does not emit. Despawn/disappearance alone does not clear old generations; no invented retain(current_entities). Canonical UiJustBecameInFocusEvent carries the actual Entity identity.
- ui_notify_hints_if_toolbar_in_focus 14233d680 /1569B. Any nonzero keep emits one zero-sized country_core::resources::toolbar::ToolbarUiIsInFocus event in that update. No payload inferred from the ICF-folded grow_one symbol WriteSettingsFileCmd.

Production: ui_interaction/focus.rs with four exports appended to ui_interaction.rs. Core event owner and configure_app registration added by central. No original business algorithms replaced with noops, single-level propagation, fabricated parent defaults or temporary replacement component identities.

Native verification:

`native_focus_world.py` constructs the actual original Bevy dense-table/read-query protocol including real entity generation metadata, component columns, optional-marker table presence and query match bitsets. The **complete original functions** execute in Unicorn; real original hashbrown insertion/removal/growth and BFS/visibility/event code executes. Only OS allocation (allocator/free/realloc), memcpy/memset, thread-local random seed retrieval and disabled profiling boundaries are hooked. Hooked standard libraries are not the business algorithm under test.

`probe-full-focus.py` produced `full-native-focus-fixtures.json`:
- 2048 original visibility executions: all256 keep bytes * four real camera modes * marker presence2.
- 32 full original ancestor graphs: multiple roots, chains, diamonds, self/other cycles, dangling parent indices, keep0/1/2/255, preservation of unrelated state/selected bytes.
- Nine consecutive original event frames with the same native Local set and Event owner: repeated focus, transitions2->1,255->1, zero removals/re-entry, hints once per focused frame.

Four actual ECS tests (focus_tests.rs) run the production systems against the original recorded outputs, including original visibility panic on absent parent state. At this report's creation these tests are **awaiting central Cargo**, not claimed passed. Exact source hashes and status in manifest.json. The fixture module is original output data; it contains no duplicate implementation of the algorithm.

Full native behavior was executed in dense-query mode; Bevy's sparse/archetype traversal glue is delegated to the actual pinned dependency in production. Interactive game/GPU checks remain separate.
