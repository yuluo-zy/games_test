# Current complete-world focused validation template

Prepared only; no Cargo, compilation or binary execution by this agent. Root integrated production world919bfa6980ff3de6b2d19d394fc1d4b0833dd0ff3e1f40d5096a9fd6fef3d481; template gates that source and28 explicitly audited dependency/lock files. No standalone Cargo crate/lock, source projection, type stand-in, function body replacement, precompiled country-core rlib or shader fixture is created.

The safe concrete route is an actual `cargo test -p country-core --lib --features native-physics --locked --offline --no-run --message-format=json-render-diagnostics` from the existing workspace. It compiles the complete current crate/source and real pinned Rapier. The runner picks only the Cargo-emitted unit-test executable whose target is the current country-core/src/lib.rs; it lists tests, requires exactly overlap5 + bounded-intersects4 + finite-simple5, then executes only those three actual module filters and checks all14 ran without failure/ignore. It records binary hash andlogs and rechecks source inputs before/after. Default does not build; only root may pass --execute with a new root-owned output directory. The template itself has not been executed.

## Why an isolated small full-world harness is not currently claimed

The complete world.rs references TerrainHeightsData/TerrainHeightSsbo, calculate_hash, DirtyTiles, canonical WallId/DecoratorId/DecoratorStorageAddr and collision/query_policy. Compiling all its current test modules additionally needs actual intersect_meshes and DecoratorPhysicsColliders. Keeping full canonical source files expands through:

- terrain_height.rs actual readback-SSBO macro -> StorageElement/SsboType -> current render/ssbo.rs + asset_libraries/ssbo_library.rs -> graphics contract, mapped/shadowed buffers, import interfaces;
- decorator_storage.rs -> actual decorator/history types, state diff events and curve utility owners; decorator.rs also uses actual history module;
- DirtyTiles -> existing conservative_line_raster; previous complete world tests -> real mesh-intersection pipeline and companion collider owner;
- full canonical walls/mod.rs declares broader building/publication owners. Omitting them by rewriting WallId/StorageAddr or replacing terrain/hash functions would change the compilation subject.

Thus there is no claimed standalone small closure without either compiling the current actual complete package or a separately authorized/reviewed mechanical source-item extraction plan. No such extraction is in this template. These are concrete transitive source dependencies, not an observed compiler error; root has not yet compiled this prepared closure. If complete country-core reports source/API gaps in unrelated current owners, preserve exact diagnostics and mark this validation blocked by those owners. Do not substitute default terrain/alwaysfalse functions, new registry/World, a prior core rlib or stripped business logic to turn this green.

The audited input list is intentionally a direct-edge evidence list, not a false claim that28 files form the full crate/build closure. Cargo determines actual full dependencies. DefaultHasher in current render/utils::calculate_hash is an existing explicitly unverified current behavior; using that unmodified source is not native hash equivalence. These focused tests address query/content/owner correctness under the accepted current implementation, not full original mesh-hash/caching parity.

## Dependency provenance

lock-provenance.json records full current/original B lock hashes and selected exact package source/checksum identities. Rapier0.22.0 PounceLight7b5ece73..., parry0.17.0, nalgebra0.33.0, BevyECS0.16.0 and all recorded glam identities match those selected original entries. **Current canonical world uses slotmap1.1.1 while original B lock has1.0.6.** Preserve current accepted dependency choice; no silent downgrade or pristine original dependency claim. Path utils/tabloid are actual current workspace source and full file hashes are listed; equal package version is not original source equivalence. Lock equality does not prove fork features or runtime behavior.

## Root invocation after review

`python3 /absolute/path/to/root-run-template.py --execute --output-dir /a/new/root-owned/validation/output`

No app/main-loop/terrain/GPU acceptance follows from a pass. Source review, actual build, exact14 tests and original target execution remain separately stated. Currently only source integration is known; test results are pending.
