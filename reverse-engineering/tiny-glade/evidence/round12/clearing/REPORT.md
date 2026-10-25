# Complete clearing producer candidate

Production is unchanged. No Cargo command ran. Central must combine this candidate with the forest consumer/animation and Olden/Flowery helper candidates before a source-build checkpoint.

## Original authority

Original RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. Full init_clearing140a78aa0/6913B and forest consumer141bf16c0/5342B are retained in `../history/reveal-forest-original.{json,asm}`. Earlier `round10` text is historical: the original two branches call startup_olden1415539d0/3588B and startup_flowery141553120/2224B, and these cannot be replaced by a mesh inventory.

The live Windows `color-menu-full` worker independently bounded/exported init_clearing, both theme helpers, the particle transform140a123a0/521B and the forest initializer closure inline in1412fa010/500B. `overlay-revision.json` records sizes, bytes and corrected analysis prototypes. The worker uses B patched EXE; original RNE bytes remain the machine-code authority. The original database and executable were not modified, and this overlay is not claimed as synchronized with them. The external library Vec implementation is reused as an execution carrier for its inlined game closure, not rewritten as game source.

PDB initialize140a0dd70 establishes the original11-parameter system: GraphicsDeviceResource, MeshLibrary, MeshWatch, ActiveSession, ShaderLibrary, ShaderWatch, Commands, mutable PlacementLibrary, mutable SsboLibrary, LoadedParticleEmitterCount and ForestPlacements. Current host adapters retain the actual device/library owners and use the existing backend for shader creation.

## Canonical owners

ForestTree is80B: PrefabName16B@0, complete PlacementDataSsbo48B@10, entity8B@40 initially absent, billboard expansion f32@48 and final4B alignment padding. The original initializer clones the Arc and every placement byte. Only records failing `abs(x)<65 && abs(z)<65` enter this owner; exact boundary, infinity and NaN are included. Bush groups are skipped before loading their placement file.

InteractableForestTree is a separate24B animation component: placement_index@0, amplitude@8, progress@c, delay@10. It does not own position or prefab identity. The previous shared proxy lost those roles. `forest_types.rs` preserves the existing core PrefabName identity used by SpawnPrefabCmd, avoiding a parallel asset/core name type. The forest consumer owner migrates manage_tree_prefabs and forest_tree_animation against these exact fields.

## Complete main behavior

The candidate creates Winter ice-cracks mesh/shader/drawable, invokes the real Olden or Flowery helper, publishes Olden's whole PreplacedAutoClutter resource and removes it for every other theme. It parses trees.json, clears prior forest records/processing state, loads every accepted placement file through the current AssetPlacementDataLibrary and retains its actual upload owner. Exterior records preserve prefab names, all placement channels, entity state and expansion.

Optional particle-emitter meshes are resolved through the original RelativePath path under assets/meshes without inventing an extension. Every placement/mesh vertex pair runs the original policy: radius strictly below45 and scale strictly above0.7, followed by stored-angle quaternion/affine transformation. Position/scale NaNs are rejected; a NaN angle still produces a tagged emitter with NaN coordinates. All resulting16B records are uploaded when the canonical ParticleEmitter buffer exists and the vector is nonempty. Only the published count is capped at1100; the upload retains the full vector and original buffer-size failure policy. Finally the producer creates the common `/clearing/distant_billboard.raster.hlsl`, named `distant_billboard_shader`, and the real billboard drawable.

`production-review.patch` removes only the old clearing mesh entry from the run_once group and installs the complete producer in the actual gameplay OnEnter(Game), after load_game_session. The other mesh entries and their run_once gate remain; their separate full-producer owners and central will replace that tuple. The old generic clearing helper is explicitly named clearing_asset_inventory so it cannot be mistaken for this complete producer. No second resource/device or default SpawnPrefab command is introduced. Theme helpers own their additional canonical components/resources. The alternate core CPU assembler remains an asset-only convenience path and is not claimed as equivalent to the full production App.

The optional ParticleEmitter buffer branch is original, not a recovery fallback:140a796a4 calls get_mut;140a796b0 tests the returned pointer, and140a796b3 jumps on null directly to count publication140a79739.140a796bc likewise branches there for an empty vector. Only a present buffer and nonempty vector execute the size check and140a79723 upload. `emitter-upload-branch-evidence.json` and its complete original LLVM block bind this explicitly. The real GPU acceptance must still require that the original full SSBO startup allocated the buffer and observe a nonempty upload; the original None branch cannot be used to claim that acceptance.

## Independent verification and candidate files

`native_placement_oracle.py` executes original game policy instructions in an isolated process, without running the PE entry point. Only allocator/CRT boundaries are hosted; sinf/cosf use ucrtbase. Sixty boundary/NaN/infinity input placements yield54 native ForestTree outputs. Their original prefab clone count, full48-byte placement copy, absent entity and expansion are checked. Eight particle fixtures cover filtering, quaternion rotation, nontrivial scale, exact thresholds and NaNs. Results are in `original-placement-fixtures.json`.

`leaf_comparison.rs` compiles the actual candidate policy module against the already-built workspace glam0.29.3 (no dependency reimplementation). Its standalone execution reported `60 original forest filter + 8 original particle transform fixtures matched`. An initial attempt selected an unrelated existing glam0.20.5 build and failed before compilation; the successful command explicitly selects `libglam-6103396c150735c2.rlib`, matching the production version. No wrong-version result is treated as validation.

Six candidate files are bound by `candidate-files.json`: tree_storage, clearing producer, policy helper, the audio unload nonempty fixture, actual gameplay registration and legacy registration removal. Rustfmt parsed all six with `--emit stdout` so unrelated existing formatting is preserved. `git apply --check --ignore-space-change` passed; the working tree contains CRLF source, so plain apply checking cannot match three LF context blocks. Apply narrow reviewed hunks against the source hashes, not whole-file replacements.

Remaining verification belongs to central: compile the combined canonical types/consumers/theme helpers, execute the real nonempty producer on shipped themes through the actual buffer/mesh/shader owners, then unload and load a different theme. Confirm old entities/forest Arcs/placements are released, new records and particle buffer bytes are populated, optional PreplacedAutoClutter is removed outside Olden, and second-session drawables do not disappear due to run_once. The leaf fixtures do not establish that full App/GPU lifecycle.
