# Shader registration worker split independent review — 2026-10-21

Scope: current application_programs.rs, production_renderer.rs, game_loop.rs and run_game.rs plus existing render_bridge / ShaderHost contracts. Read-only source review; no Cargo or GPU. Concurrent compile/type fixes are outside this report. No P1/P2 found in the reviewed registration/owner split.

## Registration and thread ownership

application_programs::register still calls the canonical country_core startup loader, with original COMMON_SHADERS, UI_SHADERS, BLIT_SHADERS and painterly/vermeer startup enumerations. It retains both shader variants via the existing ShaderBackend variant path. final_post remains the actual fifth COMMON_SHADERS handle, rather than a synthesized ID. The wrapper checks cancellation before real compiled-file reads and again before success; real IO/loader errors retain context. This moves registration/read IO to the existing gameplay worker; it does not claim lazy pipeline decoding/prewarm work has disappeared.

RegistrationBackend's Cell counters are local to the worker invocation. The worker closure captures only the canonical Arc<AssetShaderLibrary> and Arc<ShaderHost>; it does not capture ProductionRenderer, its CoreRenderBridge Rc<RefCell<VulkanGraphSession>>, pending_build Rc<Cell<_>>, egui backend or WSI owner. CoreRenderOwners transports the same Arc host tables / raster / library, and warmup creates its Rc session locally with from_owners. Underlying actual ShaderHost retains the same Arc<Device>, bindless heap/page owners and mutex-protected entry collections. No new GPU device/library/handle namespace is substituted.

initialize verifies canonical App library Arc identity, no earlier completion marker and an originally empty deferred raster/compute library; it inserts RegisteredApplicationPrograms only after complete successful registration. A partially registered library after an error is not ready and is not retried as a clean App. Normal failure aborts and cleans that actual App instead.

## Both startup paths and publication

Ordinary splash constructs gameplay ApplicationGpu::new_owners (deferred), installs its actual asset/runtime owners, captures its matching initializer and warmup owners, and moves gameplay SubApp to spawn_with_initializer. Initializer runs after real AppMode/LoadGameManager validation but before any gameplay update. Worker still must observe actual LoadingGameComplete and complete the original full-world warmup seam before publishing the App. RegisteredApplicationPrograms alone cannot release Logo or publish completion.

Loading App gets its separate library/bridge/program registration through activate_application. The original gameplay bridge, egui, library, final_post Option and programs_deferred flag remain parked together. Loading/gameplay share the device, not their same-numbered shader handles. At handoff, all five owners/state fields are swapped together; accept_gameplay_warmup validates buffer/texture/shader/raster/library Arc identities, and finish_deferred_program_registration validates both library/backend Arc identities plus the actual final_post handle before marking programs_deferred=false.

The same-App early return has been checked explicitly: when CompletedGameplayWarmup exists, it first adopts that owner's real named resources and then finalizes deferred registration. Thus same-App activation does not omit final_post. Initial same-App activation before startup intentionally lacks completion and merely permits runtime installation; render_host separately records a hard error if programs_deferred remains true, preventing rendering before actual initialization/warmup publication.

skip_splash still uses ApplicationGpu::new and registers synchronously, with final_post Some and programs_deferred=false; finish_skipped_splash keeps the existing no-Logo branch. It does not incorrectly require the background completion marker. Public/nondeferred renderer construction retains the same synchronous path.

## Cancellation and failure

Worker checks cancellation before initializer, after initializer, each update, before warmup and after real warmup; checks cannot interrupt an individual OS file read or an existing GPU wait, but no additional load/update/submission begins after observed cancellation. Real registration errors pass through the existing typed primary-error branch, save-thread shutdown precedes World drop, and cancellation cleanup failures are not discarded as benign cancellation. Parent cancel_and_join joins the worker before renderer shutdown. Failed/panicked initialization cannot insert a successful marker or publish a prepared App; completion also remains behind the original Logo gates.

## Limits and remaining validation

This proves intended ownership/control flow from source. It does not prove Send bounds/type checking, successful file registration on the real asset installation, absence of driver errors, throughput gain, or new source GPU execution. Central compilation and separate normal-splash/skip-splash runs are needed. Low FPS remains unassigned pending actual phase timings. The frame-pacer wall-clock/real-timestamp mismatch is documented independently in review-20261021-frame-pacer-metric.md and is not sufficient evidence of the performance root cause.
