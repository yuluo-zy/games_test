# Decorator interaction — phase 2, 2026-10-20

## Scope and status

Restored and frozen for the central Cargo run:

- Canonical Move/Rotate request payloads, replacing `{id,delta}` / `{id,amount}` and the decorator crate's shadow zero-sized Rotate event.
- The two original request-construction tails. They require the actual widget context, authoritative decorator address and actual UI intersection point. They do not generate a widget identity.
- Focused `AppMode::Editing(DecoClutter)` event consumption: last unread Move wins, otherwise last unread Rotate; retain full identity/hit during an active drag and refresh its hit through the authoritative `RaycastWorld`.
- Rotate consumer: all original phase branches, seven-f32 persistent local state, roof/terrain/stair destinations, original wall panic, click-to-change-variant threshold, yaw computation, complete-info preservation, archive/history/backup flags, canonical audio, and the two named arrow meshes.
- A typed change-patch path. Map-key identity is separate from mutable `Info+2c`; a patch is applied over the existing record instead of inserting patch bytes as a complete `DecoratorInfo`.
- Stair-height patch construction now targets the real extra low word and does not overwrite destination bytes through the old extra encoding. The complete stair-height UI remains outside the closed scope.

This is **not** completion of all decorator interaction. The full focus widget producer, move, place, height affordance variants, stair discovery and the default/tool hover scan remain open. The old full UI registration tuple must not be used to claim these paths are restored.

## Original evidence

Original B image SHA256:

`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`

`original-ranges.json` records exact original bytes and hashes. PDB procedure sizes include cold tails:

| Procedure | Original range | Bytes | SHA256 |
|---|---|---:|---|
| rotate | `1407a8580` | 3596 | `6cdb66166e8c7626165e3320601c067336a1ac614e3290292c1cee586520f55e` |
| affordance | `1407af340` | 9340 | `337efc52bc73e3bb5a20946aba6a39cc658c434637f67ca4e653f9fd9e729c08` |

`decompile-result-v4.json` / `*-v4.c` reconstruct six computed jump sites from their actual original relative tables. Earlier decompiler versions are retained as failed/incomplete evidence, not authoritative bodies. In particular, v3 had two unbounded tables incorrectly producing cases17/18 and bad instructions.

`capture_producer_blocks.py` executes native `1409aaa78..1409aab69` and `1409aafff..1409ab0d6`; it records actual memory writes. `producer-block-native.json` distinguishes inactive padding from actual writes.

Request field offsets below exclude the event system's eight-byte EventId:

| Field | Move | Rotate |
|---|---:|---:|
| widget | `00` | `00` |
| hit | `08..47` | `08..47` |
| map-key DecoratorId | `48` | `48` |
| rotation height offset | — | `4c` |
| raw DecoratorType | `4c` | `50` |

The producer constructs a **decorator collision variant**. `hit.handle[0] = 4 | id<<32`, and handle words1/2 are the authoritative storage address tag/payload, not a cached position or Rapier index. Position is read from native UiState+4c; normal is Y; toi is zero. Hit+34..3f and variant padding are inactive/unwritten in these producer tails. The owned API normalizes inactive padding; it does not claim those bytes were valid original raycast fields.

`capture_rotate_ops.py` executes the original full-sized rotate body up to its common signifier-loop entry for90 finite cases: three destinations × raw phases0/1/2/3/4 × three decorator types × two cursor distances. Hooks replace only named external dependencies and CRT math; original branch decisions and112-byte request construction execute unchanged. `rotate-native-ops.json` retains request/audio/local outputs.

`capture_change_events.py` feeds the54 actual emitted requests to original `apply_decorator_op` (`141276e30`) and executes its complete event assembly. Archive/storage calls are dependency hooks. `rotate-native-events.json` records the resulting136-byte canonical events. This proves request/event packing; it is not a claim that hooked native storage/archive internals were executed.

The production ECS regression lives in `ui_systems/native_rotation_goldens.rs`. It runs the actual request helper → canonical event → focused dispatcher → rotate → typed storage/archive/history/audio path → public `unleash_decorator_op_events` → canonical Events. It compares active native request/event fields and finite yaw/audio values, and checks preservation of the remaining existing72-byte Info. Central owns Cargo; final results are recorded separately in `VALIDATION.md` after the run.

## Corrected old interpretations

1. Dispatcher header word+8 is zero for Move/Rotate; it is not the widget or decorator ID. Widget is+10, hit+18, ID+58; Rotate offset+5c and type+60.
2. The affordance global gate tests AppMode tags `{0,3,4,11,20}`, not Dispatcher tags.
3. Raw drag3 begins a drag,4 continues it;0 is cancel and1 commit. Both0/1 use the final update branch and arm pending history/backup reset.
4. Rotate's native40-byte EventInstance is `PlayDecoratorSoundCmd`, confirmed by the full PDB FunctionSystem signature. It is not `DeferredDecoratorOpQueue`. Ratio, position-context word, position and flag must survive.
5. Original world-to-screen scale uses `MouseDepthValue.view_depth`, projection Y scale and window height. It does not use FOV as a distance.
6. Native change112B has addresses@0/10, destination@20, patch fields@50..65, actual map key@68 and decorator type@6c. The previous `Info@8` interpretation is incorrect for change variants. Legacy unconverted change callers are not counted as closed.
7. ExtraInfo begins at Info+30 with niche/tag at Info+41. Reading/writing extra at Info+10 damages the destination. Central fixed the canonical core accessors from this evidence.
8. The arrow draw closure uses `corner_arrow` in the world pass and `corner_arrow_outline` in UiPass. Native animation starts with zero scale, flag9=1, value=.3, duration0, t/eased0. Central owns completion of the original zero-duration animation update; no producer workaround pre-fills t=1.

## Remaining gaps, in dependency order

1. **Full focus producer / widget collision volume tag8.** Original `ui_focus_decoclutter1409a92c0` computes clone/rotate/move gizmo geometry, complete transformed volumes, signifiers and identity formatting. Existing `WidgetVolumeShape` cannot yet express the full original tag8 shape. The boolean `DecoClutterFocusUi` adapter was fabricated and emitted incomplete requests; it was removed from registration with central authorization. The restored request tails are callable, but a game input cannot reach them until this real widget producer is restored and registered.
2. **Remaining Dispatcher variants.** Place initialization0/1, Height4, stair-focus5 and other original union payloads are not reconstructed by the Move/Rotate payload enum. `interaction:None` explicitly denotes that absence. The old `state/payload/ty` compatibility fields are not a complete union representation.
3. **Remaining affordance scans.** AppMode0/4/11/20 needs the full all-intersections scan, decorator/stair graph and tool policy. `ui_dispatch_focused_decorator_requests` is deliberately a focused-branch scope, not a replacement claiming all9340B behavior.
4. **Move / place / chimney and stair height UI.** Existing files still have old partial logic, including legacy use of Dispatcher header payload as an ID. They are not restored by fixing the rotate chain. No old partial tuple was mounted as a shortcut.
5. **Legacy backend callers.** The new typed patch is used by rotation and stair-height construction. The old compatibility change path, guessed external command fields, and incomplete chimney request still require migration from original payload evidence; they are not counted complete.
6. **Renderer and audio consumers.** Canonical audio now retains ratio/context/flag; the existing audio-loop consumer does not yet interpret all those fields. Original event-context enum naming is unresolved and retained as a u64 rather than guessed. The broader signifier translation/configuration branches remain open even though the rotation factory has concrete drawable meshes/transforms and the original animation fields.
7. **Wider validation.** The90 cases cover finite inputs, control-flow phases, click variant changes, preserved data and native event outputs. They do not prove all NaN/Inf or irregular rotation states, all tool-mode policies, whole-game input or rendered GPU parity.

## Reuseable reverse-engineering method

- Verify the exact original image and per-function bytes before using a patched analysis database.
- Use PDB full procedure sizes, not only unwind hot fragments. Recover computed jump tables from original table bytes before interpreting a decompiled branch as unreachable or empty.
- Read the full FunctionSystem type before naming a writer or resource. Equal queue strides are not evidence of equal event identities.
- Keep enum headers, active payload, inactive padding and application identity separate. Record native writes, not merely a poisoned byte value, to identify padding.
- Capture real original requests at dependency boundaries; run actual production ECS against those captures. Do not validate a second transcription against itself.
- Preserve the existing record when applying a patch; retain all unaffected business data. Explicitly separate map key, mutable record words, storage address and destination.
- A passing constructor/factory test closes that leaf only. Report the missing producer, consumer, scheduling and render edges separately so the central host never registers a partial tuple as a completed workflow.
