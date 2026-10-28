# RetainUnproven compatibility policy read-only assessment — 2026-10-21

No production or bottom-library changes, no GPU execution.

## Decision

**Suitable for an explicitly requested, bounded diagnostic run that observes actual Logo→gameplay rendering on a device otherwise capable of the graph pipeline. Not suitable as silent Strict fallback or a clean-shutdown acceptance run.** It preserves real present execution; absent final WSI completion remains a returned error and retained ownership, not fabricated completion.

Current production API has no policy selector: ProductionRenderer::new hardcodes raw Swapchain::new (Strict). Existing bottom API is:

```
unsafe rhapsody::raw::swapchain::Swapchain::new_with_policy(
    device: Arc<Device>, surface: Arc<Surface>, extent: vk::Extent2D,
    vsync: bool, usage: vk::ImageUsageFlags,
    policy: SwapchainPolicy::RetainUnproven,
) -> rhapsody::Result<Swapchain>
```

`rhapsody::present` reexports SwapchainPolicy. Default new remains Strict. Any host selector must be explicit CLI/diagnostic configuration and log the effective policy; normal callers/default remain Strict. `rg::frame_loop::FrameLoop::new` expressly rejects every policy other than Strict (frame_loop.rs192–196); do not weaken that invariant. Current tiny-glade ProductionRenderer manages the real raw swapchain/graph itself, so it can consume the existing raw compatibility API without altering FrameLoop or reintroducing old algorithms.

## Actual guarantees / limits

- swapchain.rs92–108: Strict validates maintenance1; RetainUnproven accepts no-maintenance1 and allow_present only rejects AcquireOnly. RetainUnproven therefore executes actual presentation, not offscreen-only or fake-ready output.
- PresentTicket::wait/is_complete prove **render timeline only**. `is_present_complete` calls actual collect/acquire/present proof. native_present_success_count counts real SUCCESS/SUBOPTIMAL queue results, not actual screen completion. Diagnostic report must keep these categories separate.
- Without maintenance1, subsequent acquisition fences can prove older presentation complete. Latest/final successful presentation may remain unproved once closing stops acquisitions. Shutdown still calls finish_gpu_use then finish_shutdown and returns Unsupported `最终呈现尚无完成证明；保留交换链所有权` when no final proof. Must retain this error. Never turn queue/device-idle into completion.
- raw Swapchain Drop failed shutdown (except DeviceLost) logs failure, increments retained_generation_count, deliberately retains device+surface+lease/presentation owners+retired ancestry; does not vkDestroySwapchain/imageviews. This is safe ownership retention and **failed cleanup**, not success. It can keep the whole device and real Win32 Window Arc alive until process exit.
- ProductionRenderer::shutdown currently tries swapchain.shutdown before cache/session clear/device.wait_idle. Failed compatibility final close stops there and closed remains false. LoopExiting/Drop retries do not re-enable callbacks, but final run can return nonzero even when desired Logo/gameplay frames were actually rendered. Accepting visual diagnostic observations does not accept normal final teardown.
- Win32 PlatformSurface uses Surface::from_win32_retained with actual Arc<Window> (production_renderer.rs1717); retained surface does not outlive destroyed HWND owner. No temporary window/surface substitution required.

## Resource bounds and resize

- MAX_RETIRED_GENERATIONS=4 (raw/swapchain.rs78), **current generation additional**. replacement_for polls proofs then checks recursive retired ancestry count before creating; returned Exhausted/NOT_READY maps to acquire Ok(None), preserving current chain and applying backpressure. It does not evict an unproved generation.
- Completion of a new-generation presentation can retire older generations; old own fences/reacquire proofs can also release them. With repeated resize before any proof, after four ancestors next replacement is refused until real proof progresses. A diagnostic should hold a fixed explicit small window extent and bounded frame count; forced resize tests are separate coverage.
- This is a **generation-count bound**, not byte or time guarantee: each generation holds actual swapchain resources and the device. `retained_generation_count()` is a process-wide monotonically increasing diagnostic count, not a budget/cap. Repeated separate swapchain/device construction+failed destruction could retain more than four total permanently. For a single fresh diagnostic process/one window, report retained count before/after and retire process after run; do not loop new renderers in one process to evade failed shutdown.
- Other graph minimum Vulkan features (timeline/dynamic rendering/full descriptor indexing etc.) still apply. RetainUnproven only relaxes **provable final WSI teardown capability**, not graph/device feature requirements.

## Suggested evidence contract for explicit diagnostic selector

Preserve actual loading App, Logo five-stage update, LoadingGameComplete and gameplay graph warmup/handoff unchanged. Record effective RetainUnproven choice, actual loaded/handed-off App stages, actual native present success counts/frames and any real screenshot capture. Keep final closure error and retained-generation count. Do not describe compatible diagnostic as Strict/native WSI shutdown passed; don't suppress first runtime failure or broaden CPU/GPU timeouts.

## Warmup 32x32 diff source review

Reviewed production_renderer.rs warm_up_gameplay plus callers of build_full_world_stage/world_extent/display_extent. Only worker warmup supplies Some([32,32]) for outer world target and tail display/final_post extent. Ordinary compile_full_world_graph passes None, and headless steady-state render passes None: previous world scaling/OOM overrides and FrameRenderConfig.extent_8 display behavior preserved. No WindowSize/GameConfig/FrameRenderConfig/camera mutation. world_graph history selection still uses configured extent_8 for TGSR, outer color dimensions for TAA; world_postprocess enforces/uses same rule. Therefore source does not assume inner TGSR history is32 based on outer extent evidence. Complete world→tail builder, real compile/execute/export and warmup completion token remain mandatory. No new P1/P2 found by source review; no runtime execution or fresh original1400d7a51 decomp was performed here.
