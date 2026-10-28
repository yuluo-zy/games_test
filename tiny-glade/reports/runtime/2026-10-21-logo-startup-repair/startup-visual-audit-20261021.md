# Startup visual audit and diagnostic handoff — 2026-10-21

No Cargo/GPU/GUI run by this agent. Production rendering algorithms and rhapsody are unchanged. Modified files: production_renderer.rs and world_postprocess.rs (diagnostic only), original_loading_gpu.rs (fixture export/control only). Frozen hashes: startup-visual-trace-frozen-20261021.json.

## Established current paths

- Logo/loading owns a separate loading App. Its normal path is default/saved loading background through Vermeer -> MEI temp/combine -> loading draw list containing original logo meshes -> exact final target export -> original resolve. It is not supposed to render the actual gameplay world below the Logo; substituting a world/clear image would be unsupported.
- Main color/depth use world_extent. Only the proven worker warmup explicitly supplies32x32; normal frame uses WindowSize times actual world_resolution_scale. Main tail uses FrameRenderConfig.extent_8. The original owner handoff transfers WindowSize and FrameRenderConfig from loading to gameplay, not a warmup-modified size.
- Current rhapsody owned temporal requests compare requested descriptors and replace changed-size resources with invalid history. TAA uses actual color dimensions; TGSR uses existing extent_8. There is no current source evidence that32x32history is permanently reused as960x540history.
- Central executed normal960x540world content test: mean129.3/distinct98159,42.95s; this agent visually inspected logo-startup-world-960.png showing actual grass/trees/low wall and first-frame noise, not a32x32block enlargement. Capture is after final_post before overlays and does not prove final UI, Logo background, WSI source alignment or temporal convergence.
- UI is deliberately hidden while Vermeer active outside photomode. Vermeer first waits2frames, then increases progress by min(dt,1/30)/3; about93 gameplay frames are needed at capped dt. The prior60gameplay-frame window run usually cannot have finished this reveal gate. This is a concrete reason why successful present counts cannot establish menu visibility. Do not skip the gate.

## Composition and alpha boundaries

- MEI temp uses transparent clear, combine loads the target and uses the existing premultiplied overlay state (ONE, ONE_MINUS_SRC_ALPHA). Logo/UI draws preserve target contents. Static source review has not established an unconditional black clear bug.
- Existing reconstructed/lifted resolve source reads integer texels, picks MSAA/non-MSAA from actual source samples and writes output alpha1. A mismatched source/swapchain extent can read outside source and produce black; it does not automatically scale a smaller source to a bigger window. The new diagnostic records actual source and acquired-target dimensions at the same resolve declaration.
- Existing loading GPU test only asserted any visible RGB. One glyph can satisfy it, so it never proved the background or actual time progression.

## Explicit diagnostics now available

TG_STARTUP_VISUAL_TRACE=1 creates only diagnostic snapshot resources and a log controller. Disabled branch never formats logs or repeats environment reads. Loading/main have separate counters; first3, every30 and structural state changes are logged. Progress/dt/frame counters are observations, not per-frame change triggers.

main-export logs are emitted only after real graph execution and native texture export validation. They include graph+receipt identity, actual main/depth/TAA allocations read from that receipt physical mapping and backend image owners, declared dimensions separately, actual tail/samples, WindowSize/FrameRenderConfig extents and scale, MEI available/drawn counts, actual UI-gate snapshot and entity counts, Vermeer phase/hold/freeze-camera/debug-freeze/frame/progress/dt/capped-dt, menu tag, Logo progress and draw-list lengths. Missing/culled actual allocations remainNone. resolve-declare logs source/swapchain actual dimensions/samples and equality for those same selected main receipts; it is labeled declaration and does not assert final present completion.

The diagnostic does not hold any GPU owner or modify dimensions/history/timers/UI flags. Generic T13 feedback capture/report remains excluded.

## Loading image fixture export

original_loading_gpu test now accepts TG_LOADING_SIZE=960x540 (default320x180), TG_LOADING_PIXEL_OUT=absolute.png or .ppm. It reads the actual final loading target after Vermeer+MEI+logo-list composition and exports unchanged pixel bytes. PNG preserves raw alpha; PPM shows RGB. It prints full/top-left/bottom-band mean, black fraction, distinct RGB and alpha range. It retains all prior assertions but does not upgrade any-RGB success to full visual acceptance.

Fixture limits are printed: default Logo progress0, one Time tick1/60, default FrameConstants and manual Tiny Glade glyph input. No actual async handoff/welcome UI is proven. The current genuine no-SavedSessionList background preparation is explicitly polled and asserted ready; no substitute image is made.

## Existing final post-overlay capture seam

Production already calls shared_texture_capture::opportunity(purpose6,tail.msaa,4) after T21Egui and before export. An explicit test/diagnostic can enqueue ScreenshotPurpose::FinalFrame with the existing gather_screenshot_requests Rgba sink on the actual World, then run genuine GameLoop frames and normal finish/hurry. This exercises the existing generic consumer only, adding no screenshot system or T13 feedback producer. Headless FinalFrame includes UI/T20 but has no EguiBackend; actual GameLoop is needed for full T21 coverage. Normal purpose2 capture remains insufficient for overlay acceptance.
