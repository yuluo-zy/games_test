# Startup color ownership audit

Scope: startup → assembly → main loop. T16 is frozen at `../color-menu/t16-freeze-manifest.json`; T13 is excluded. All new files are evidence/candidates. No production source writes or Cargo execution.

## Current conclusion

The actual `tiny-glade` gameplay App already connects both grading producers. No missing grading/LUT registration was found in that App. The concrete remaining color-load correctness blocker is the incorrect production `GladeColorCustomization` schema. Its complete ten-file candidate belongs to `../glade-colors-full/`, not this audit. The original-defined full schema must be integrated before a saved session can prove color continuity.

`country-core::startup::run_game` is an alternate CPU convenience assembler. It still does not register session grading or initialize `ImageFormationConfigs`; its comment claiming that the CPU grading system requires a device is stale. The production gameplay App uses its own complete registration. Do not silently use the convenience assembler as proof of production startup, or duplicate the GPU loader there.

## Real producer and consumer chain

1. `tiny-glade/src/game_loop.rs` installs the actual graphics libraries and calls `gameplay_startup::install_gameplay_device_runtime` before transferring the gameplay SubApp to the asynchronous loader. Both splash and skip-splash paths retain that same gameplay startup schedule.
2. `gameplay_startup.rs` registers a PreStartup Inputs system that calls `initialize_common_assets` followed by `initialize_grading_assets` on the actual HostTables. `GradingLuts::load` scans `assets/luts` once, creates the real texture owners, and pairs metadata with textures. The host publishes `grading_lut:<name>` into the same `AssetTextureLibrary` that final_post/gallery consume; `tony_mc_mapface` and `menu/active_lut` retain their distinct packaged keys. Loading-App common assets are separate; no handoff moves or overwrites gameplay GradingLuts.
3. `tiny-glade/src/run_game.rs` initializes the canonical three-slot `ImageFormationConfigs`; each OnEnter(Game) runs `load_game_session`, then `load_glade_grading`. The latter takes only `ResMut<ImageFormationConfigs>` and `Res<ActiveSession>` and performs no GPU allocation.
4. Theme `grading.json` (or the native I/O-error default / Halloween2024 fixed grade) replaces all three slots and clears all lens/artistic fields. The saved `SessionMeta.image` then replaces slots 1 and 2 completely; slot 0 stays the theme default. The selected kind is preserved. Therefore a second session must reapply CPU configuration, while the process/App LUT assets remain owned and reusable.
5. `world_postprocess` reads the current canonical configuration, flattens bypass, verifies named/default/fixed texture ownership, then binds final_post. Gallery reads slot 0 plus the original loaded LUT metadata. No additional LUT load is required by a session switch.
6. Color customization is independent of grading: wall registration currently initializes GCC; `parse_saved_game_data` reads `Colors` using the canonical GCC type or uses GladeSettings-derived initial colors on missing/invalid Colors; `apply_initial_resources` moves `parsed.colors` into World before replay and captures that same value for saved sections. The complete GCC schema preserves optional windows, named enums, plural fences, flag color and all four stair fields. The current production scalar/bool substitutes cannot establish this chain.

## Original evidence and independent verification

`startup-color-audit.json` binds original RNE SHA256, four complete PDB ranges, six direct calls confirmed at LLVM instruction boundaries, all 28 packaged LUT files, five shipped theme grades and ten source hashes. Full original `load_glade_grading` is 2346 bytes, `ImageFormationConfigs::default` 607 bytes, GCC Default 82 bytes and `GradingLuts::load` 1690 bytes. The original texture-load wrapper at 140a2ce90 calls GradingLuts::load at 140a2cf5f; the native grading wrappers explicitly name the two canonical CPU resource types.

`native_grading_oracle.py` executes the complete original grading function and original configs constructor in a disposable subprocess. Only allocator/deallocator and CRT memory operations are hosted; no game function is replaced and the PE entry point never runs. Nine fixtures cover all three selected kinds through None → Some(full saved image) → None on the same owner. Every original result preserves kind, applies the full saved grading/lens/artistic payload only to slots1/2, clears stale fields on the next no-image session, and sets the actual change tick. `grading-original-lifecycle-fixtures.json` records the results. Non-Halloween file I/O/JSON parsing and GPU execution are outside this oracle.

All five shipped grade names resolve to existing LUTs; the Halloween and fallback default LUTs also exist. Every shipped PNG has exactly 16³ RGBA8 texels, matching the native fixed descriptor. This is a packaging check, not a replacement parser or GPU parity claim.

## Narrow candidates and remaining acceptance

- Integrate the GCC owner's complete schema/caller candidate, then run its original-wire assertions. Do not apply the superseded windows-only patch.
- `grading_lifecycle_original_fixture.rs` is a small test body to append inside the existing grading test module. It checks the actual Rust producer against all nine original fixture outputs, including repeated session state clearing. It has not been compiled or executed; central owns the build window.
- For the actual App, validate first session → asynchronous handoff → Reload → second session with two different themes and distinct saved image/GCC values. Check one retained GradingLuts/texture owner set, changed CPU slot0/1/2, retained selected kind, canonical GCC replacement and real load-complete publication. Existing static registration and the isolated native fixtures do not prove that full schedule/lifecycle.
- Native LUT parser/device behavior and complete final_post mode controls remain bounded by their respective owners. `custom_lut: None` and unknown render mode policy are render-path gaps, not evidence of an absent startup grade. No default value or new fake producer is proposed to hide them.
