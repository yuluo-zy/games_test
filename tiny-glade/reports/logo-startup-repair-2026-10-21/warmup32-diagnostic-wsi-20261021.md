# Warmup dimensions and explicit diagnostic WSI policy — frozen handoff

Files: production_renderer.rs; only shipped_session_through_actual_app_and_production_world_graph in original_gameplay_chain.rs. No game_loop/event_loop edits in this task. No Cargo/test/GUI/GPU run by this agent.

## Original dimensions

Evidence: reverse-engineering/tiny-glade/progress-20261021/logo-handoff/REPORT.md, {async-loader,render-outer,compile-main,texture-new-2d}.asm, bounded-native-manifest.json. Original binary SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03, bounded static instruction proof supplied by audit_ui_startup, no new reverse run by this agent.

- async1400d7a51 sends packed width/height0x2000000020 without swapchain.
- render forwards RCX through closure field+18.
- compile_main uses same dimensions for world color/depth and render_world.
- no-swapchain branch retains that pointer for final_post R9 at140162f65.

Production now passes explicit32x32 to the world_extent existing override and an explicit display override to the same full world/tail builder. Ordinary headless/WSI calls pass None, preserving their original display-resource read position and ordinary resolution scale/OOM behavior. No WindowSize/FrameRenderConfig write, pass removal, draw omission, fake ready, or change to compile/execute/export/GPU completion wait.

## Histories and intermediate dimensions

TAA continues deriving its history extent from the actual color target. Existing rhapsody typed temporal allocation replaces a changed descriptor (get_or_create_owned_inner) and invalidates previous contents; completed warmup owners are still transferred/adopted normally. No manually reset history flag or additional history owner.

TGSR history/output currently read FrameRenderConfig.extent_8 in world_graph/world_postprocess. Original internal TGSR/DoF sizing was not established by this outer-argument proof, so those inputs are deliberately untouched. The claim is world targets and final_post output32x32, not that every intermediate texture is32x32. Normal first-frame viewport resources remain actual window dimensions.

## Actual scene content test migration

The old test captured its warmup and required>2000 distinct colors, impossible for1024 pixels. The test now performs the real32x32warmup, accepts its completed owners, executes a genuine App update and a normal-size headless production frame, waits for its actual receipt, and finishes the original screenshot owner. Existing luminance>40 and distinct>2000 assertions remain intact; a new capture-dimensions==realdisplay assertion prevents accidental reduced content acceptance. This is headless content acceptance, not actual WSI or complete startup/playability.

## Explicit diagnostic WSI policy

TG_DIAGNOSTIC_RETAIN_WSI unset or0 selects Strict;1 selects the existing rhapsody::present::SwapchainPolicy::RetainUnproven via Swapchain::new_with_policy. Any other value (including non-Unicode) is a named error before device setup. Startup warns that real presents are enabled but final proof may be absent and shutdown may fail while retaining owners. AcquireOnly is never used. No shutdown, capability, native present, error, timeout, resize, or rhapsody implementation was modified.

RetainUnproven is a diagnostic run mode to expose startup's next real failure on the current device lacking swapchainMaintenance1. Neither enabling it nor headless rendering constitutes strict WSI completion. A native shutdown error must remain an error, not be remapped to success.

## Tests added, not executed by this agent

production_renderer::world_extent_tests:
- warmup_world_and_tail_extents_override_dimensions_without_mutating_scene_inputs
- ordinary_frames_keep_scaled_world_unscaled_display_and_oom_override_contract
- diagnostic_wsi_policy_requires_explicit_opt_in_and_never_uses_acquire_only

Central owns compilation, original-gameplay content execution and independent review, and will preserve exact frozen inputs. No performance claim is made from the32x32argument alone.
