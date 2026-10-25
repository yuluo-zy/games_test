# Real original B grass startup producer

Legally claimed startup_grass.rs TG-df846677de4b2a2a75 (targetB), owner boot-terrain-20261018. This is work-only candidate Rust; no shared production edits, no Cargo/GPU run. candidate-manifest.json binds source target, candidateSHA, original RNE SHA and every retained PDB-bounded native body.

The complete @140a21620/1027B creates the missing W9 producer and two later grass drawables:
1. load_shader_into_library(`/grass_object_push.raster.hlsl`, `grass_object_push`) @140a2168c;
2. `0u32..=5` tuple_windows -> each `(a,b)` maps to `[2a,2a+1,2b,2b,2a+1,2b+1]` -> Take27; native complete iterator specialization @1412fff70/1690B has original map stores @141300503..548 and take-count supplied27 @140a216ba. Reuse current startup_quad::quad_window_indices rather than write another quad algorithm. Actual27 indices end `[8,9,10]`, not all30. No vertex attributes are present; shader uses vertex IDs.
3. Mesh label exactly `grass blade`, library name `grass_blade`; source_file None and empty attribute HashMap are native values. Common loader @140b1aca0/456B calls MeshWatch::watch, then adds that actual Mesh to AssetMeshLibrary; it does not build a second GPU library/VAO directly. Candidate preserves this order and current architecture.
4. load `/grass.raster.hlsl` as `grass_shader`, queue actual DrawableMeshBundle with Transform::identity(), DontCastShadow and DesignatedRenderPass(7).
5. load `/grass_lod1.raster.hlsl` as `grass_lod1_shader`, queue the same mesh handle and second actual shader with the same components. Component tuple identity is proven via original command function1412dc470, referenced @140a217ff/@140a218ec. No KeepOnLevelUnload marker is present in its canonical tuple. Transform rotation original bytes14293e0c0 are `(0,0,0,1)`, translation0,scale1 proved native stores. No fake positions/normals, fallback shader or extra device is added.

Critical schedule proof, no signature-only inference:
- original direct-call scan startup140a21620 → run_unsafe140a91110 callsite140a913b6 (also run1415014a0 / run_without1414ba880), signatures prove six original SystemParams Device,MeshLibrary,MeshWatch,ShaderLibrary,ShaderWatch,Commands;
- exact original pointer run_unsafe at142ad9e60 is inside actual System vtable142ad9e10;
- into_configs140a91810 @140a9195b selects that exact vtable, not merely a similarly typed function;
- 11-item inner tuple into_configs141736760 @14173695d invokes that config helper. This is its fifth item by actual assembled output order; no `.chain()` was proven for the inner tuple, so do not invent per-item serial order.
- outer chain141452870 @141452940 invokes that inner tuple; add_game_startup_systems140941110 @140941150 invokes outer chain; @140941158 DL=1 then @14094115d invokes named SubApp::add_systems<OnEnter<AppLoadingState>>140b66830. Canonical AppLoadingState::Game=1.
Thus this belongs to **OnEnter(Game)** after real session loading in the outer chain, **not** Startup's unrelated8tuple or COMMON_SHADERS13. No extra run_once/once-per-App gate is justified. Actual session exit/removal must precede recreating game entities on the next OnEnter; do not mask missing lifecycle by making this producer idempotent.

The candidate startup_grass.rs takes actual same-App library references, current ShaderBackend and Commands, so parent host can wrap it within real OnEnter execution. Root needs `pub mod startup_grass;` in shared startup/mod.rs. It intentionally uses the existing current loader adaptation (same as other accepted shader startup functions); shader hot-watch support is not independently implemented or claimed completed here. The original ShaderWatch input remains an integration boundary for watch-enabled runs, no dummy ShaderWatch is supplied.

Two candidate tests exercise actual canonical libraries+Commands (TestDevice only at CPU shader boundary), exact27indices/noattributes, actualtwoDrawableMeshBundle identities/pass7/DontCastShadow, and registration of all3paths/names. They have **not been compiled/run** by this worker; root runs unified checks. A later real GPU grass compute/draw chain also needs original Blade SSBO/indirect producers and scene passes and is not claimed verified by these tests.
