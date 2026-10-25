# Main-frame pipeline prewarm: independent read-only design assessment

2026-10-21. No Cargo/GPU or production/bottom-library changes.

## Decision

Do not directly remove the three main-frame `ShaderHost::prewarm` calls or treat generic Timeout as successful skip. A safe nonblocking pipeline-pending protocol is not present in the current host. It requires an explicit suspended-frame continuation; this is larger than a local call removal and should remain a separate bounded change.

## Exact current seams

- production_renderer.rs compile_full_world_graph ~455–490: declare world/tail (mutates World), global-host prewarm, compile_built.
- production_renderer.rs compile_loading_graph ~1169–1205: declare loading/Logo graph, global-host prewarm, compile_built.
- production_renderer.rs execute_presentation_graph_inner ~1055–1090: takes actual acquired image into graph, declares resolve, global prewarm, compile, execute.
- render_host.rs1594–1668: prewarm clones **all** compute/raster registered entries plus indexed-count adapter, sequentially ticket.wait until one180s deadline. It waits unrelated old/new pipelines too; no graph-scoped required-ticket set exists.
- render_host.rs2941–2954: independent `with_compile_backpressure` sleeps2ms on a full compiler request queue up to180s **during declaration**. Removing later waits alone does not remove all main-thread stalls.
- rhapsody pipeline.rs226–265 already provides PipelineTicket::poll→Pending/Ready/Failed and try_get→Timeout on pending. poll is the suitable existing API; generic Error::Timeout could instead be a terminal ticket failure, resource/device timeout or queue pressure and must not be globally swallowed.
- compute callback render_host.rs3653 and raster_host.rs1493–1501 try_get at actual record time. Pending returns errors, not silently valid partial output.
- country-core render_frame.rs316/319–325 returns every compile error except explicit OOM recovery; render()272–279 aborts any error. No Pending outcome in RenderHost's compile/execute contract.
- render_bridge.rs257–295 build_frame has one pending declaration per session. BuiltCoreGraph438–466 owns original Rc session and cancels on Drop. It can park safely, but no other build is allowed until compile/cancel.
- rhapsody rg/vulkan.rs3001–3120 consumes PreparedSessionFrame. Callback errors can be PartialSubmission; original callbacks/resources must retire, the consumed graph is not retryable.

## Narrow viable implementation design (future scope)

1. Attach an owned, deduplicated **required PipelineTicket set** to BuiltCoreGraph via current tiny-glade compute/raster declaration seams; include indirect/indexed helper tickets. Do not scan the whole historical ShaderHost catalog. Use poll's Pending/Failed distinction and retain original terminal error category. Failed means existing terminal abort/cleanup, not fallback success.
2. Add tiny-glade host `PendingMainGraph` holding the same BuiltCoreGraph, output ID/format/sample expectation, App/World and shader owner identity, one-shot screenshot graph ID and current render phase. No repeated graph declaration after Pending: Vermeer/painterly/capture/readback and dynamic constants already changed during the original declaration.
3. Add explicit `FrameOutcome::{Pending,Complete}`/continuation at GameFrameDriver and render-host adapter boundaries. Pending returns control to OS, holds the same App update/egui result/graph exactly once, freezes further App.update/event-consumption/reveal/temporal advancement/frame counters until this exact frame executes or is cancelled. Native input/close continues; input waits for next real update. A Pending frame is not counted Complete.
4. Keep last actually published `Arc<Texture>` and its real owner/synchronization evidence. Simplest pending behavior leaves the last OS presentation visible and makes no new GPU submit; do not replay prior command graph or fabricate a frame receipt. A future repeated-image present must import retained texture and real dependencies into a new resolve graph, not reuse a consumed callback graph.
5. Resolve/presentation should have required variant readiness checked before acquiring when possible. If its graph is parked after acquisition, retain original acquired lease, BuiltCoreGraph, source texture and App owner; resize/handoff/shutdown must cancel through originating bridge once. No app swap/new graph while pending. Never assume present complete because a render point completed.
6. Address compile-request queue admission separately. Changing with_compile_backpressure to return pending mid-declaration without transactional ownership/CPU rollback is unsafe. A nonblocking admission continuation or staged bounded request manifest needs explicit design; do not simply replay whole world declaration.
7. Startup/background warmup may continue using explicit bounded prewarm where thread ownership permits it. Removing a global blocking wait is not a license to move arbitrary World/renderer graph execution to an unsafe worker or restore old rhapsody implementation.

## What a safe small interim change can establish

Instrument prewarm pending-count/duration separately from declare/compile/record/submit/GPU waits before attributing the observed ~1.4s gameplay frame to shader prewarm. All-ready prewarm still scans/clones catalog but performs no waits; no evidence reviewed here establishes it dominates steady-state latency. A graph-specific readiness fast path can reduce unnecessary polling but does not itself create the pending continuation required above.

## Reveal / black or blurry observations

- vermeer.rs42/44/46: max dt=1/30, reveal duration3s, warmup2calls.
-599–608: only after warmup and !hold, progress += min(dt,1/30)/3; at most1/90 per effective call.
-626–633: only progress>=1 && !freeze && !hold transitions Disabled and publishes actual StartupRevealFinished.
- This is **not a hardcoded92-frame gate**: approx90–91 growth calls (float accumulation), plus2warmup calls only if not already passed; loading World may have already consumed warmup while hold kept progress0. Slow actual FPS turns the original clipped-delta animation into much longer wall time.
- world_postprocess.rs1208–1231 derives ui_visible = !vermeer_active || photomode. Session previews, LUT gallery, UI2D/MEI/cursor stay gated while normal reveal is active. Vermeer uses real blur pyramid/stroke raster/combine, vermeer.rs362–452; expected stylized blur must be distinguished from missing output/pixel errors.
- Audited60-gameplay diagnostic has no `Game reveal complete.` line. Starting from loading progress0 it can have advanced at most~2/3, so no complete normal interactive UI acceptance follows. Do not force progress/frame count or emit Finished to bypass. Observe actual reveal state + Finished + post-reveal pixels with the original gate unchanged.
