# Independent startup / reload acceptance

Status: **candidate prepared, not compiled or executed**. No Cargo, production source mutation, target writes, test-system replacement, or large codegen was performed by this owner. Root owns compilation and runtime acceptance. T13 is excluded.

## Evidence collected independently

`native_oracle.py` reads the original RNE, verifies its known SHA256, extracts eleven complete PDB-bounded functions, and records direct callees in `native-evidence.json`. `.asm` files come directly from LLVM on that RNE, not an implementation author's report. All addresses below refer to this original image.

Observed:

- `140b66480` has the PDB OnExit<AppLoadingState> builder; `140b66830` is OnEnter. The unload builder `140b3f550` chains the first closure before the inner pair. The exact query in `140a04980` is Entity / Without<KeepOnLevelUnload>, Commands. No extra ordering between the two inner systems is claimed.
- `140b3ca2b` writes false into ClearCanvas payload, then `140b3ca42` calls AutoSaveState.close_file. The uninit routine clears the two placement owners, audio triggers, RT meshes and owned GPU instances; its forest processed byte is zeroed at `140a7a774`, then optional water activity is cleared. The audio snapshot empty/Disabled distinction is retained.
- Full photomode session initialization at `140839820..82e` writes scalar, sentinel index, cleared flags, and mode/focus bytes. This is a different operation from the selective on-enable reset.
- Loading audio `140b1b477` tests flag at +30, adds GameTime delta into +34, uses a **strict >1.0** threshold, and clears the flag at `140b1b53a` while retaining timer. `140b1b5a8` is an allocation error cold block, not that clear. The current candidate algorithm matches the timer policy; its documentary address is wrong.
- Original tree producer `140862270` compares the literal eighteen-byte `clearing/tree/bush` name at `1408626b0..6d2`. Its 48-byte placement loop at `140862870..89b` accepts strictly abs(x),abs(z)<65, and `1408628db` forms ID100001+accepted. Raw constants independently read: name prefix at142a95d80, suffix at142a95d90; f32 at142a83814 is65.0. Applying this to original autumn assets yields281 initial trees, recorded with source hashes in `intro-legacy-asset-oracle.json`.
- Original nonempty meadow fixtures: Flowery1 batch/66 records, Summer4/197, Winter0, recorded with per-file hashes in `nonempty-theme-oracle.json`.

## Candidate delivered

Use only `frozen-v2`; v1 has been superseded because it incorrectly required every initial tree to remain alive after the original collision filtering phase. The v2 test observes real intermediate owners each App frame and allows the actual filter/delete lifecycle.

`frozen-v2/startup-acceptance.patch` adds `tiny-glade/crates/tiny-glade/tests/startup_session_lifecycle.rs`. The manifest records new-file absence, proposed source SHA, patch SHA, and fourteen production source hashes at the audit point. Rustfmt parsed the source; patch-to-source byte roundtrip was verified. Neither is compilation or runtime evidence.

The test installs the complete real `run_game_thread`, physical headless Device, CoreRenderBridge asset libraries, production `install_gameplay_device_runtime` tuple and GameFrameDriver. It does not remove systems, substitute resource owners, manually construct prefab output, repair startup resources, or synthesize reveal events.

Acceptance assertions:

1. Empty-account native boot chooses the unchanged original v16 intro. Completion requires real Loaded state, request ownership consumed, exactly one LoadingGameComplete, nonempty authoritative walls and AppMode restored to Editing(None). Failure is explicitly rejected; an absent request alone is insufficient.
2. Exactly one NewSessionStarted and one legacy SpawnPreplacedTrees. Across real frames, observe all281 expected IDs/names/positions through live TreeStorage, real48-byte placement, real88-byte prefab contents, nonempty GPU instance ownership and valid actual RtWorld handles. Original collision-based deletion is allowed afterwards.
3. Ninety real App updates preserve restored AppMode/Loaded state, advance canonical GameTime and do not replay session/completion/legacy events.
4. Flowery (with saved grading override) → Flowery (without it) → Summer → Winter, through actual NextSession + NextState(Reload), actual OnExit and subsequent OnEnter. Every exit emits exactly one false ClearCanvas, removes temporary and prior meadow/tree entities, preserves KeepOnLevelUnload sentinel, clears both placement maps and retains their capacity.
5. Every theme entry creates the original nonempty placement buffers at48-byte stride and real meadow entities, uses new live handles, restores theme grading slot0, and applies/removes saved overrides only in slots1/2. TreeStorage may contain only live entities.
6. Deliberately dirty prior photomode flags and Free camera pose reset on a new session. Enter the installed photomode transition consumer and verify the actual Free Dolly derives from the current loaded camera rather than the old pose.
7. First Winter entry must resolve ice_cracks. After reload and real clock progression the loading state expires while timer remains>1.0. No mock audio or manufactured reveal event is used.

Root runtime command after integrating the new target: set `TG_INSTALL_ROOT=D:/game/ljxsj_92385/Tiny Glade`, then run the `tiny-glade` integration target `startup_session_lifecycle`, single test thread. The physical headless device is intentionally required. Existing v16 original_gameplay_chain coverage and new tests should not run in the same process because paths are process globals.

## Current source findings and limits

Observed in the manifest-stamped production:

- Original unload begin/eight-owner/despawn lifecycle and per-session photo reset are now registered. Parsed request bypasses the absent current history check. Earlier observations that these were missing are obsolete after central integration.
- The whole OnEnter mesh tuple still has `run_once`, including the seasonal init_clearing_meshes. A first nonwinter→Winter transition therefore cannot load ice_cracks through that tuple. The proposed test will expose this unless the full per-session clearing producer supplies it.
- clearing::init_clearing remains a mesh/shader projection with an explicit unresolved two-spawn comment. Its forest placement, billboard/seasonal drawable, emitter/capacity lifecycle is not proven complete by the new meadow test. This is the largest remaining source-level startup gap visible in this audit.
- Real StartupRevealStarted now has a writer in the six-owner loading handoff. StartupRevealFinished now routes from EffectsHost through world_postprocess's actual Vermeer owner. The former claim of absent writers is obsolete.

Inference: the continuous full-App test should expose owner aliasing, missing event reader state, duplicate placement loads, lifecycle ordering and theme-specific run_once mistakes that an empty/first-load-only smoke cannot expose. Until compilation and execution, this remains an expectation, not a passing result.

Unknown / deliberately not claimed:

- The candidate starts at run_game_thread and drives complete App updates; it does **not** execute GameFrameDriver.frame's physical render callback, loading worker→six-owner handoff, final GPU screenshot, Vermeer progress or the audible reveal timeline. These need the actual host-loop acceptance, not fabricated Started/Finished events.
- Keyboard/OS input session reset was source-read but has no runtime assertion here. Direct AppModeChanged in the photo test exercises the production transition consumer, not the full user-input UI path.
- Clock continuity is observed from production initialization and tested as a process-owner invariant; no exhaustive original-program negative proof of every possible clock replacement is claimed.
- Initial281 trees is a transient spawn population, not an assertion that native collision filtering must retain281. Native filter acceptance and resulting inserted starting-edit contents remain separate algorithm acceptance.
- KeepOnLevelUnload's original query filter is proven; exhaustive correctness of every production marker producer is not proven by preserving one sentinel.

## Efficiency / correctness lesson

Use nonempty, sequential fixtures and capture intermediate owned outputs before valid later algorithms delete them. Tie completion to the target request, terminal state, mode restoration and actual side effects. Keep the original binary oracle independent from author reports. Freeze candidates with hashes before central integration. State parse, compile, runtime, and rendered-interaction evidence separately.
