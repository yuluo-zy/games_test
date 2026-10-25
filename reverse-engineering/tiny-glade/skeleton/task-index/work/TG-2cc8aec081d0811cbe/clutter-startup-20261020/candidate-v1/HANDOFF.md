# Clutter Startup companion candidate v1

State: implemented_unverified; five candidate sources only. Root authorized narrow root-owned companion paths in addition to the existing TG-2cc8aec081d0811cbe lib.rs claim. Every before file was copied afresh from current production for this batch; no old before copy was reused. No production/Cargo/Git/database file was changed and no build/CPU/GPU test was executed.

This implements the scoped Startup design reviewed in ../design-delivery-v1.json SHA d538dc095290173aecc74c1eb0fd17cbaad8dd91ed4095bbe70b55062412515f. Independent design review is independent-validation/clutter-startup-20261020/DESIGN_REVIEW_V1.json, reported SHA4e721e52... . Six complete original B procedures and exact Startup tuple/name/parameter evidence remain in the immutable parent design. This batch does not amend that frozen delivery.

Changes:

- lib.rs removes the loader alone from the existing per-frame tuple. Its original two subscription Startup members and the partial full Update plugin remain uninstalled; this candidate does not call configure from the gameplay host.
- resources/mod.rs exports the real country_core canonical clutter resource.
- gameplay_startup registers one idempotent Startup closure using the captured actual installation root, same AssetMeshLibrary, same MeshWatch and actual demo config. It queues canonical ClutterMeshes only after the complete real loader returns. ClutterStartupRegistered identifies registration only and never satisfies asset completion.
- run_game marks the actual gameplay World at construction. The actual GPU gameplay installer also marks its direct-runtime callers after physical owner/resource preconditions; the independent loading App does not receive this role. SubApp move retains the same role. Worker verifies real canonical catalog after successful update and before reading LoadingGameComplete. Existing real event, cancellation and warm-up gates remain unchanged, and every existing save-before-World/GPU guard remains in place.
- game_loop catches actual App::update panics into a reportable error. After successful update, a World with the actual GameplayApplication role must have the canonical catalog regardless of Splash/Game/Reload state or camera presence. Both failure paths return before platform output, rendering, successful-frame counters, temporal advance or FPS limiting. This current-host panic adapter retains original panic text; it does not claim typed std::io::Error identity from the leaf's original unwrap.

Coherent external source dependencies, to be integrated by root with these five sources:

- canonical resource source under TG-5d112ff9638b93a02e/scene-clutter-producer-20261020/source/clutter_meshes.rs SHA5314749ba8a40db1958d72536408ebe0e2f3d200b26ab896cf396e396fd3fcb1.
- loader v2 under TG-b374246e420cab4618/scene-clutter-producer-20261020/v2/source/load_clutter_meshes.rs SHAf3c82bdb831d76a70a0dcf84577633d07397c8d365fc5ba54b2ebc18967a4fa7, API load_clutter_meshes_from(&Path,&AssetMeshLibrary,&MeshWatch,bool)->canonical ClutterMeshes, including original watch-before-add.
- **Correction to the frozen design's Cargo observation:** the workspace declares system-clutter, but tiny-glade app crate does not yet depend on it. required-root-dependencies.json pins current app Cargo SHA and the necessary root-only [dependencies] hunk `system-clutter.workspace = true`. Root acknowledged central insertion and local dependency metadata update without external version upgrades. This owner did not edit Cargo. The five-source candidate is not a standalone typecheck without this dependency and the two leaf sources above.

Validation source and limits:

- Added Startup registration-only, missing-real-file/no-publication, synchronous update panic-text, and all-three-loading-states/no-camera missing-catalog rejection tests. Expanded the existing clutter registration test to reject a per-frame loader.
- Included one explicitly ignored genuine B installation test using TG_INSTALL_ROOT. It calls the same actual Startup closure, expects61 loaded full-game CPU mesh owners and6 flowerbeds, checks true installed source paths, then verifies a second update preserves mesh count and handles. It is a scoped CPU asset-startup test, not full gameplay/Nani/GPU acceptance.
- Corrected an existing save cleanup test: its small App has no genuine catalog, so its formerly named warmup-failure branch now tests missing-catalog rejection and that warmup is never reached. Immediate cancel may race with this observable rejection; both must join the real save worker before World drop. No fake catalog is inserted to preserve the old test's warmup claim.
- Existing direct-channel handoff fixtures still prove only their resource-transfer protocol; they do not prove a successful real loader/warmup path. Positive full-game warmup/present requires genuine startup resources.
- syntax-and-source-check.json records parser-only rustfmt --emit stdout on all five sources; output discarded. Patch apply check and freeze are content/applicability checks only. Root owns Cargo/typecheck/test/GPU execution.

Suggested root validation after coherent integration: system-clutter bush_registration_tests; tiny-glade clutter_startup_tests, the two new frame_tests and save_shutdown_preparation_tests. Run the ignored genuine_clutter_startup test explicitly against the original B installation. No check has been executed by this owner.

Remaining boundaries: original two subscription Startup members and their real collision/raster dependencies, full clutter Update/blueprint physics lifecycle, canonical mesh-to-Nani atlas binding and nonempty draw→submit→present are still open. Presence of this genuine CPU catalog does not replace LoadingGameComplete, successful original graph warm-up or rendering success. B-specific asset paths/counts and demo omission semantics are retained; no C pack-change requirement is imported.
