# Validation boundary

Central owns Cargo. This agent ran no Cargo command.

## Confirmed results

- Original producer-block execution: both Move and Rotate construction tails completed. Actual memory-write tracking identifies inactive padding; storage-address words are real address tag/payload, not positions.
- Original rotate interaction core:90 finite cases, covering roof/terrain/stair; raw cancel0, commit1, hover2, start3, ongoing4; Lantern/Flag/clutter; zero and nonzero cursor displacement.
- Original canonical event assembly:54 emitted112-byte requests passed through original `141276e30`, capturing actual136-byte events.
- Additional original storage-field audit:54 requests executed original `140a20890`, including its hash probe and field update branches. Only explicit map/storage primitives are fixture hooks. Seed changes touch only Info+2c/+45; normal rotation preserves non-destination Info+28..47. Native destination-union padding is distinguished from active fields.
- Central `tmp/phase2-final-decorator-lib.log` confirms production test `ui_systems::native_rotation_goldens::actual_rotation_ecs_matches_all_native_phases_and_preserves_existing_info` **passed**. Its90 cases run the actual canonical request helper → event reader → focused dispatcher → rotate system → typed patch/backend → public deferred-event release. It checks real archive/storage/history/backup/audio state and active native event fields.
- Central `tmp/phase2-final-decorator-core.log`:9/9 core-path integration tests passed.
- Central `tmp/phase2-final-decorator-audio.log`:12/12 audio integration tests passed.

## Narrow fixture correction after the initial full-lib run

The initial full-lib run had215 passed /1 failed. The remaining failure was the old `apply_decorator_op::tests::sentinel_change_updates_in_place_without_removing` fixture: `storage_with` set `u40=0x0800`, placing invalid ExtraInfo tag8 at Info+41. The old accessor had decoded a different, incorrect destination-overlapping location, hiding this fixture mistake.

The actual Stair extra niche is0/1. The fixture now uses tag0, which also passes the original wrapping `(tag-2).min(6)|4 ==6` guard. Only that fixture value and its incorrect8/9 comment were corrected; no production algorithm or assertion was weakened.

Final central rerun confirmed by reading the actual logs:

- `tmp/phase2-final-decorator-lib-retest.log`: **216/216 passed**, including the90-case actual production ECS test.
- `tmp/phase2-final-core-focused-request.log`: **1/1 passed**; real UiState gate and all three authoritative storage addresses.

The host build attempt in `tmp/phase2-final-host.log` fails before host compilation because the configured original installation has no `fmod.dll`. This environmental result does not change the CPU test results and does not establish that the complete game can run.

## Limits

- Each of the90 production cases starts from a fresh world and default Local state. A multi-frame drag → commit → History serialization/replay oracle has not been run. The existing archive function and native `DecoratorArchivalResources` collision/first-snapshot behavior still need independent review.
- Native rotate execution stops at the common signifier-loop entry; the mesh factory/draw tail is reviewed against original instructions, not covered by a native wake or GPU rendering oracle.
- The original library atan2/fmod dependencies are explicit math hooks. Finite yaw/audio comparisons use a narrow float tolerance; the tested control flow, active byte fields and preserved unrelated Info bytes are checked separately.
- Full tag8 focus-widget geometry is not restored or mounted. The request tail helper does not invent a widget identity. The old boolean FocusUi adapter was removed with central authorization and is not counted as restored functionality.
- Move, place, height/stair discovery, remaining Dispatcher variants, default/tool hover scans, legacy unconverted backend changes, broader signifier configuration and complete audio-loop interpretation remain open. No full-game or three-workflow completion claim follows from these passing scoped tests.
