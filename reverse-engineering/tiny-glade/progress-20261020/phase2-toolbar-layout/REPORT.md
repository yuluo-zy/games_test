# Phase 2 toolbar layout / normal toolbar setup

Binary: B original `workspace/input/tiny-glade.exe`, SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
Procedure identities: `evidence/country-core-audit/data/procs.tsv` (original PDB procedures, not mismatched patched DB bounds).

## Recovered production behavior

- `toolbar_size_mult_update`: original run_unsafe wrapper `140feff70/673B`, arithmetic `140ff0052..140ff00be`. Width for Bottom/Top, height for Left/Right. Denominator `1282.5*icon_size+30`; Left alone adds `icon_size*(-192.5)` then 22, preserving rounding/order. Native MINSS upper cap, NaN=>1, no invented lower clamp. Startup default remains original zero; host must run updater before sizing.
- `add_toolbar_item` `141e4dc40`: replace fabricated raw64B ABI, unsafe Vec handling and empty kind_handler with owned complete eight-variant tree. Row/Column use real Taffy styles, recursively consume children and preserve original focus parent Entity. Leaf base dimensions are item.size*82.5, one-frame scale1, animation state2, actual .011f32 mouse bytes. All original Custom FnOnce/AppMode/Slider/SliderReset/Spacer/Locked commands are performed. Custom callbacks execute, and Rust ownership drops children/closures exactly once on error/unwind.
- Slider constructor min/max/default tables `142f8f6d8/142f8f70c/142f8f740`, inverse_lerp `140c904a0`; all thirteen initial normalized defaults preserved. Slider nav tags `[2,2,0,0]`. Other navigable leaf defaults `[0,0,0,0]`. Locked has Expand without Nav. Spacer has UiCustomSize `{kind=1,value=1.0}` without Nav or Expand. Core shared type migration performed by central agent.
- `add_contextual_menu` `141e4d1e0`: empty list returns before row lookup; nonempty missing tag preserves native panic. Definition row has relative Node@0, **parent Entity@8**, menu Node@16, tag@24. Parent is loaded at `141e4d501` and passed as sixth add_toolbar_item argument. Anchor is order1/gap10/Above; menu style is Center/Center with zero gap. Original false boolean-sentinel/raw allocation model removed.
- `setup_screen_and_toolbar_nodes` `141e4ccb0`: actual root styles restored. Screen padding30 on all sides, FlexEnd cross alignment and Center justification; toolbar Center/Center with 10-point gap. Full native function executed with only Taffy primitive results stubbed to capture both styles.
- Generic `add_category_button` `1426235d0/1756B`: root base size110 or original optional multiplicative override, appear1/state2, interaction[1,0,0], .011 mouse bytes; callback follows cooldown0, then Expand and HideIfMenuIsOpen. Duplicate-category assert retains its original late location after command emission. Automatically registers actual button Entity into contextual table.
- `toolbar_startup::setup_toolbar`: **all eleven original categories and full descendant trees**, both demo/full editions, captured by executing actual original construction block `142345968..142346e68`. No business payload inferred from label names. Every AppMode tag/payload mapped to current canonical core enum, including native cobblestone qword and decorator tag. Original demo locks and repeated lock labels preserved. Original category roots: TimeOfDay component for tag11; demo Tree/Water lock; all others SwitchToolbarCategoryButton. Real cyclic nav uses the registered Entity identities. No synthetic subset setup inserted.

## Executed independent checks

- `probe-size-mult.py` + `actual-source-size-mult.rs/.exe`: **704/704 exact f32 bit matches**, all4 alignments,11 resolutions (including extreme u32),16 user scales including signed0/negative/Inf/NaN. Also checked original change tick write42. Rust function extracted from actual production source.
- `probe-slider-initial.py` + `actual-source-slider-initial.rs/.exe`: **13/13 exact f32 bit matches** against executing original `141e4e817..141e4e85b` including its original inverse_lerp helper. Canonical production slider_math source compiled directly.
- `probe-toolbar-setup.py`: full original allocation/constant/tree logic executed in both editions with allocator and memcpy modeled only. Captures all11 root categories and54/43 flattened original records. Original nested item trees remain in `native-toolbar-setup-trees.json`.
- `probe-category-constructors.py`: **48 original executions** (demo2 * tag12 * optional size2), capture original bundle and complete four command records. Original variants independently identified using PDB generic command identities. No fake replacement of algorithm under test.
- `probe-item-styles.py`: actual original leaf/row/column/contextual and both root Styles captured before original Taffy new_leaf. Actual pinned taffy field offsets checked via rustc dependency rlib. Old Rust empty-Vec memory order differs: never transmute an old entire Style into the rebuilt dependency.

## Production ECS tests awaiting central runner

`toolbar_layout_tests.rs` four tests: all leaf dispatch/components/82.5 scaling/parent identity, nested Row/Column tree and no container entity, contextual empty/anchor/parent branch, optional category scale plus Custom FnOnce and panic-drop ownership.

`toolbar_startup_tests.rs` two tests: full native54/43 record snapshots including kind/label/exact size bits/FPV flag/slider index/AppMode tag+payload; real Commands setup11 roots/full descendants/menu parents/cyclic navigation/demo locks/time-of-day dispatch.

Root agent alone runs Cargo. Prior root Cargo10/10 passed the then-current factory baseline, but the six new tests and complete setup need the next central run. Source hashes in `freeze-sha256.json`.

## Integration / honest limits

Central must export `pub mod toolbar_startup`, run the outer original OnEnterGame assembly (reset Taffy once, normal setup, photo setup, undo/redo setup, insert all three real resources), and schedule the recovered size updater. Photo setup owned by toolbar_photo_menu; undo/redo canonical owner/setup and host wiring owned by central. Those cannot be declared closed by the normal-tree evidence alone. Final interactive in-game rendering/input remains a separate verification gate; bounded instruction oracles and ECS tests do not prove a complete interactive GPU session.

One-time generate/implement scripts are retained as reconstruction provenance; they write production paths and should not be rerun over later collaborative edits. Probe scripts read production and write only evidence artifacts.

## Follow-up freeze and schedule evidence

Normal/photo category constructors now share one safe ordered implementation: normal cooldown before callback, photo action+ShowOnlyPhoto before cooldown. All original leaf component appends are independent Commands inserts, preserving original lifecycle order rather than combining inserts into tuples. Final toolbar_layout SHA in freeze-sha256.json.

Original schedule constructors were executed (140fee0e0/set7,140fed8a0/set44,140fef7c0/set36,140ff0be0/set8). Each constructed System vtable::name was then executed to obtain its canonical name, avoiding false names from stale unused stack bytes. Complete38 original leaf memberships retained in native-schedule-config-names.json. Nested 3/5 member set7 groups and3/3/2/2 member set8 groups retain unchained tuple semantics inside the outer chain_ignore_deferred. Root owns actual host wiring.
