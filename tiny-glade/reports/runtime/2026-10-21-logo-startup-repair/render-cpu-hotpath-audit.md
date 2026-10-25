# Render CPU hot-path audit

2026-10-21. Inspection of render_host/raster_host/render_bridge and their production callers, followed by centrally authorized diagnostic counters only in raster_host.rs. No rhapsody, production_renderer or cache behavior changed; no Cargo, GPU run, original execution or profiler performed by this agent. CPU costs below are source-proven operations, **not measured frame-time percentages**. Central TG_FRAME_TIMING measurements must determine actual priority. Source line numbers below refer to the audit snapshot before the instrumentation insertion; symbol names remain authoritative.

The user's acceptance standard is recorded in `D:/game/PERFORMANCE-STANDARD.md`: 60 FPS minimum, 120 FPS qualified, with full algorithms and actual visible960x540 Release interaction. This audit does not qualify current performance.

## Implemented diagnostic handoff

`RasterShared::vao_refresh_timing() -> VaoRefreshTiming` returns a Copy cumulative snapshot specific to that App/owner. `after.delta_since(before)` returns same-owner deltas with u64 fields `calls`, `list_entries`, `queried_entities`, `unique_vaos`, `elapsed_ns`. These are sums over all full-list refresh calls in the interval, not a frame-wide deduplicated entity count. Direct subset refreshes are excluded.

Only `game_loop::frame_timing_enabled()` enables sampling. Disabled runs construct no Instant and increment no counters. One RAII measurement covers a full refresh including physical-owner validation; early errors retain their original return/cleanup while still publishing timing. One aggregate mutex update occurs on completion, with no per-call logging. The elapsed CPU wall sample includes time in nested refresh_vao_handles but excludes the final diagnostic aggregation lock. It is not GPU time or additional workload to be added to a parent declaration duration.

Central production-renderer diagnostics should take before/after snapshots from the same live RasterShared and log one delta per frame. Do not subtract across an App switch. This agent informed audit_building_terrain of the exact API; runtime integration and validation remain central work. No snapshot-reuse optimization has been implemented.

## Findings

| Finding | Proven work | Impact still to measure |
|---|---|---|
| Repeated complete VAO snapshot rebuild | One refresh in ProductionRenderHost::build_draw_lists, then seven/eight SceneRasterHost::new calls per rendered world; each scans all21 lists plus ECS VAO entities and reconstructs all wanted snapshots. An earlier direct W5 refresh also overwrites the table. | Total refresh milliseconds, wanted/list/ECS counts; compare to graph declaration duration. Safe reduction boundaries are established below. |
| Whole-host pipeline prewarm twice per frame | Main/Loading graph and presentation graph each call ShaderHost::prewarm. It clones all registered compute+raster entries, computes a deadline and calls wait on every ticket, including already Ready ones. | Ticket count, ready/pending count, scan/wait milliseconds. Ready wait returns immediately after a mutex and Arc clone; do not describe all calls as GPU stalls. |
| Declaration-side pipeline miss does synchronous preparation | A genuine new raster key decodes bytecode, lowers the bindless shader package, constructs layouts/modules/reflection, and only then submits pipeline compilation. Compiler-queue Timeout retries sleep2ms, bounded180s. | Per-frame miss count and cold/new-variant declaration time. Existing-key steady frames do not execute this work. |
| LOAD preservation queries rescan earlier graph accesses | raster_has_previous_contents scans earlier passes/accesses until an overlapping write is found; called per writable LOAD attachment. | Earlier pass/access counts and cost. This scan enforces real range semantics; it cannot simply be removed. |
| Global texture table declaration | Each raster pass that reflects set2/b0 clones the frame sampled-view vector, imports each view, and declares every potential read. | Table length and affected-pass count. These dependencies are required for dynamically indexed access; deleting unused-looking entries is not justified. |

No proven per-frame shader-file reread or pipeline-key instability was found in the inspected normal path. The local per-pass render-pass cache starts empty, but a second persistent descriptor cache stabilizes its IDs. Mistaking that local miss for global pipeline recreation would lead to the wrong repair.

## VAO refresh: exact cost and safe intervals

`raster_host.rs:349 refresh_draw_vaos`:

1. Walks all `DRAW_LIST_COUNT` lists; constructs a BTreeSet of VAO ids.
2. Creates an ECS query over all `Handle<Vao>` entities, filters wanted ids, builds a HashMap.
3. Resolves every wanted id or errors; constructs a Vec of typed handles.
4. `refresh_vao_handles` resolves each live AssetVAOLibrary entry, validates its physical index and vertex buffers through this BufferHost, clones names/Arc buffers, builds another HashMap.
5. Replaces `assets.vaos` wholesale.

`scene_raster.rs:67..72` performs that operation unconditionally in every constructor. Current caller locations are:

| Call | Location | What intervenes since prior refresh |
|---|---|---|
| W23 shadow | world_graph.rs:123 | W1–W22, including direct W5 terrain refresh. Keep a full fresh snapshot here. |
| W25 main | world_graph.rs:132 | W24 terrain-lighting compute, new GBuffer/velocity graph textures. |
| W30 OC2 | world_graph.rs:140 | HZB; W27 wall pass2; W28 Nani pass2; W29 grass compute. |
| W34 late | world_graph.rs:144 | W31 visibility query, W32 autofocus, W33 mouse-depth compute. |
| W43 deferred | world_lighting.rs:234 | W35–42 lighting compute plus temporal texture allocation. Deferred fullscreen draws do not themselves use a VAO. |
| W44 refraction (unless skipped) | world_surface.rs:58 | New refraction graph texture. |
| W46 pre-TAA | world_surface.rs:80 | Water/ice preparation, raster and finish. |
| W53 post-TAA | world_postprocess.rs:212 | DoF/TAA/TGSR graph declarations. |

The initial `production_renderer.rs::build_draw_lists` also refreshes. **It cannot simply be reused for all W stages**: `world_terrain.rs:411` invokes `refresh_vao_handles(world,[floor_vao,rocky_vao])` during W5 terrain visual-height readback. Because that helper replaces the full snapshot map, the original full draw-list VAOs are no longer present. Any optimization retaining only the initial refresh would introduce missing/stale owner errors.

The following mutation checks support reusing the W23 snapshot through W53 **in the current synchronous declaration call chain**:

- `world_geometry.rs::calc_terrain_lighting_texture`, grass_compute, run_visibility_queries, calculate_dof_autofocus and mouse_depth_compute take `&World`. Their PassEnv obtains only shared AssetVAOLibrary access. They declare GPU writes to SSBOs/textures/indirect commands, not CPU VAO-owner replacement.
- `world_walls.rs::batches` takes `&mut World` but only marks AssetSsboLibrary changed and temporarily hides/shows the highlighted sparse allocation. It does not mutate AssetVAOLibrary, entity VAO handles or the VAO's physical vertex/index buffers.
- `scene_compute.rs::generate_nani_draw_lists` marks GpuWorld changed and passes a shared reference into the real cull/list producer. Cull/indirection buffers are separate from the CPU VAO snapshot map; the function does not rebuild VAOs. GPU content writes to an already retained buffer do not require replacing its owner snapshot.
- `world_lighting.rs` uses SceneComputeHost with shared World and allocates graph/temporal textures; its resources remain separate from VAO entries.
- `world_surface.rs` uses shared World in SurfaceEnv for both water and ice. Current list callback production bodies in draw_callbacks_a/b, draw_lists::bodies and their draw_shaders helpers do not obtain mutable AssetVAOLibrary or alter entity VAO handles. Test/setup mutations were excluded from this conclusion.
- `world_postprocess.rs` up through W53 uses shared compute views; W54 updates ShaderRefactoringState after the final listed raster constructor, not VAO ownership.

Thus W25, W30 and W34 refreshes are the narrowest confirmed reducible interval after W23; the same source check supports reducing W43/W44/W46/W53 refreshes too (seven duplicate refreshes in the normal non-demo world path, six if W44 skipped). This is a repeated owner-validation/allocation reduction, not deletion of original draw/compute algorithms.

### Preserve dynamic semantics

Recommended first implementation, subject to measured benefit: retain a full fresh snapshot immediately before W23 and reuse it through a **scoped world-render operation**. Keep the normal validating constructor for unrelated callers. A reused snapshot must be tied to the same App/RasterShared, graph and frozen DrawLists; direct `refresh_vao_handles` must invalidate the reuse condition. Do not silently extend it across another frame, OOM rebuild or App transition.

For a generalized cache, an owner version must advance on successful `AssetVAOLibrary::add/rebuild_vao/remove`, and list membership/typed handles must be tracked separately. `vao_library.rs::rebuild_vao` intentionally retains the same VAO id while releasing/replacing physical buffers and updating index_count. A cache keyed only by raw id, list pointer or frame dimensions would be incorrect. Merely checking a Bevy change tick is also weaker than an explicit mutation revision for repeated modifications within one tick. Preserve stale/wrong-device validation and never drop generation checks for a faster lookup.

The shaders/SSBOs themselves still need every existing per-draw binding and graph dependency. These facts authorize neither freezing their contents nor skipping grass/wall/vegetation GPU producers.

## Shader and pipeline cache audit

| Layer | Source | Result |
|---|---|---|
| Program lifetime | render_bridge.rs stores Arc<AssetShaderLibrary>, ShaderHost and RasterShared; RasterPrograms::Core resolves actual library entries. | Per-pass hosts reuse owners. No program cloning/re-registration loop found. |
| Raster pipeline key | raster_host.rs:891 builds `{handle:render_pass.0, selector:packet.state.0}`. | Frame graph IDs, attachment resource IDs and dimensions are not injected into this key. State changes correctly distinguish pipelines. |
| Render-pass id | render_host.rs:721 RenderPassHost::create_from_desc checks persistent by_desc map before registering. | Fresh per-pass local cache does not produce a new handle for an unchanged descriptor. LOAD/CLEAR/depth-access differences intentionally remain distinct. Do not collapse them without proving compatibility. |
| Core variant cache | country-core/render/shader.rs:138 get_or_generate; native provenance 0x140938b00 retained in source. | Read-lock hit returns the cached boxed pipeline; write-lock entry rechecks on miss. Cache persists in program variants and has no per-frame clear. |
| Core reflection | raster_host.rs:669 `(shader-id,variant)` in RasterShared.reflections. | RhCompiled::decode in core_reflection runs only on a missing key, not every draw. |
| Disk shader read | render_host.rs:1787 read, load_compiled_compute/raster. country-core/render/shader.rs compiles program variants once. | Filesystem read is on program construction/load. Raster pipeline misses use the already owned Arc<CoreCompiled>; no filesystem read in submit_raster_pass or command recording. Dynamic later program loads remain possible and must be measured separately. |
| Pipeline cold miss | render_host.rs:1881 create_raster_pipeline. | Decode, BindlessShaderPackage creation, shader modules and reflected inputs are synchronous before compiler request. Repeats across genuinely different native state/pass keys, not an established cache failure. |
| Record raster | raster_host.rs:1491 per prepared draw `try_get`; compute render_host.rs:3651 similarly. | Nonblocking ticket access; no per-draw condition-variable wait or file read found here. |

`RenderPassHost::create_from_desc` has bounded table256; core per-variant pipeline cache256; ShaderHost compute and raster registries each4096. These are limits, not observed live counts. No actual count is asserted without runtime evidence.

## Prewarm and waits

`ShaderHost::prewarm` (`render_host.rs:1595`) unconditionally snapshots every registered compute and raster entry plus the optional indexed-count adapter. Let N=C+R+adapter. Each call performs O(N) Arc clones, allocates a target Vec, and iterates all N tickets. Calls occur after full-world graph build (`production_renderer.rs:528`), Loading build (`1350`), and presentation build (`1239`); the minimal graph fallback has the same pattern (`622`). Therefore a normal main+presentation frame repeats the host-wide scan twice.

`rhapsody/src/pipeline.rs:245 PipelineTicket::wait` was read without modification: Ready locks status and clones the pipeline immediately; Pending waits on a condition variable until completion or deadline; Failed returns its error. This proves two separate concerns:

- Warm frames do repeated CPU bookkeeping even if no new pipeline exists.
- If a newly discovered variant is pending, the production caller can block before graph compilation. The actual incidence and elapsed time require runtime counters. Do not infer that ready-ticket scans explain the user's low FPS.

Safe future direction is pending/new-request tracking or a per-frame exact required-ticket set, preserving all terminal errors and original no-fake-ready behavior. Simply deleting prewarm changes error/cancellation behavior: record currently returns Timeout when a ticket is pending. Any asynchronous continuation must preserve the last valid frame and retry through the real transaction, not skip Logo or claim warmup complete.

Additional waits:

- `with_compile_backpressure` (`render_host.rs:2941`) sleeps2ms on compiler queue Timeout, max180s. It is only reached on a new create request; do not remove bounded compiler backpressure or blindly call it a per-draw wait on cache hits.
- `BufferHost::update_buffer_bytes` (`render_host.rs:164..221`) waits for device-local sparse transfer batches with `upload_buffer_ranges_wait(...u64::MAX)`. Mapped host buffers take the direct update path. This is a synchronous completion contract, not redundant idle waiting. Its invocation count on the user's slow frame is unmeasured; CPU Update/Upload timing must separate it from raster recording.
- `render_bridge.rs:430 wait_submits_done` is an explicit wrapper. It is not called per draw. Production resize/shutdown/recycle and configured pacing need separate measurement before any synchronization change.

## Additional declaration work to measure, not erase

`raster_host.rs:2353 raster_has_previous_contents` performs a linear search of all earlier passes/accesses unless a resource is already imported with valid initial access. Called once per writable LOAD attachment, the worst aggregate grows with declared-pass/access count. Exact mip/layer overlap decides whether the attachment preserves existing contents. A host-local per-resource interval index could reduce searching, but it must be kept in sync with every compute/raster producer; a global written boolean would break partially initialized images.

`raster_host.rs:1332..1388` imports/declares the exact global texture-table views once per participating pass; `render_host.rs:1417 frame_sampled_table` clones all views each time. This is required conservative access coverage for dynamic shader indexing. The frame's immutable view snapshot could be borrowed/shared to reduce allocation; omitting graph reads or guessing which texture a shader will access is not equivalent.

## Suggested measurements and decision order

1. Capture graph-declare/compile/execute/present durations separately. If declaration dominates, measure refresh count/time plus list entries, unique VAOs and queried ECS handles.
2. Log shader registry sizes, per-frame raster/compute registration deltas, core pipeline misses and prewarm ready/pending/error counts. Stable registry sizes refute continual recompilation; nonzero deltas locate the new-key path.
3. For high prewarm duration, distinguish ready scanning from actual pending-ticket waits and compiler backpressure. Keep correctness/error propagation while optimizing either case.
4. If refresh is material, first reduce W23→W34 redundant scans with the verified scope, then extend the same scope to W53. Validate W5 readback before it, VAO rebuild on the next frame, App handoff, error/retry, and direct-pass refresh invalidation.
5. Only profile previous-write scans and sampled-table cloning if costs remain after the first measured bottleneck. These are correctness-bearing declarations and must retain exact resource/view coverage.

No numeric FPS gain, original-performance parity or global hot-path completion is claimed by this source audit.
