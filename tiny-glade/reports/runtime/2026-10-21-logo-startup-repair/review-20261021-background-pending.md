# Async loading-background pending gate: independent source review

2026-10-21 read-only; no Cargo/GPU/source edits. Source currently being edited; findings refer to the first gate implementation read around16:xx.

## P2 findings sent centrally

1. **Repeated App update before the first deferred render.** game_loop.rs296–312 runs egui.begin/App.update/egui.end and then polls background; Pending returnsOk without a render. Outer frame1000 observes unchanged completed_frames and retains edges. When Ready arrives, GameLoop again calls driver.frame, which repeats App.update/egui. Thus the same suspended render's already-run simulation/input/Logo update is not resumed; it is replaced with an extra update before rendering, while custom input edges were retained. Narrow correction is a driver `awaiting_loading_render_after_update` latch and resume render-only, with one update/egui/consumption per logical frame, not a fake completed frame.

2. **Poll deadline can be postponed by repeated native events.** event_loop.rs270–280 sets WaitUntil(now+16ms) on every AboutToWait while background pending and requests no redraw except ResumeTimeReached. Continuous MouseMove/native events can repeatedly move the deadline before it expires, delaying queued close/resize processing. Keep one fixed next-poll deadline until served or directly dispatch actual pending-mode close through the existing cancellation path. Don't turn all pending ticks into continuous redraw.

3. **A slow optional image suppresses mandatory gameplay failure propagation.** game_loop.rs912 guards the entire preparation.poll with !background_pending. That poll is also the current only immediate receiver of true gameplay producer/warmup Err. A blocked image decoder can therefore indefinitely postpone the primary required startup failure. Split worker completion/error collection from Logo release/handoff; still receive and surface errors during pending, but hold only the Logo/handoff phase.

## Current good properties

- Background worker owns CPU image/path/channel only, no World/device/TextureHost; one-slot result owns image, Drop cancels publication without OS-thread join.
- Pending is distinct from absent; first selected path frozen, corrupt/missing image uses original optional absent branch, upload only after Ready in actual loading graph.
- During established pending outer frame dispatches queued OS event batch before readiness poll, avoids egui/Logo/update/render/frame_count and temporal increments; if an actual close is in that serviced batch it returnsfalse and existing shutdown cancels actual gameplay worker before GPU.
- Suspended path retains existing direct-close behavior. Main lifecycle terminal guard remains above all event handling; no new post-shutdown frame path observed.
- Completed frame count comparison prevents pending from incrementing outer frame_count or clearing edges; however initial update consumption must be resolved with the logical-frame latch described above.

## Remaining review boundary

No real delayed IO, event flood, close during pending, frame/egui invariant test or main process run performed. Central/implementation agent may have fixed these after this snapshot; reread current source before declaring outstanding status.

## Follow-up source review after central fixes

The three earlier findings are resolved by current source:
- GameFrameDriver loading_render_pending_after_update preserves completed App.update/egui result and resumes only readiness→render. Outer input_consumed_this_call clears custom edges exactly on an actual update, while newer pending input survives to next update.
- GameLoop background_poll_deadline stores one absolute deadline; AboutToWait no longer moves it on every event. ResumeTimeReached clears it before next poll. Pending-mode CloseRequested directly executes actual shutdown/cancellation path.
- GameplayPreparation::poll_completion receives real errors/owner without touching Logo or switching App; outer frame calls it before background readiness gate. Normal poll still enforces actual mandatory Logo owner/wait/final stages.

Remaining P2 on new diagnostic capture, sent centrally: GameFrameDriver's TG_STARTUP_UI_CAPTURE currently regards !VermeerState::is_active() alone as “real reveal completion.” Original gameplay startup initializes Vermeer disabled (run_game.rs469), and explicit skipped splash can remain disabled without a natural reveal. This can emit an incorrectly labeled post-reveal capture. Require observed gameplay active→natural Disabled transition with progress>=1 and hold/freeze policy, or observe the true finished event while excluding explicit skip-path event. Reset the0.5s diagnostic timer when reveal activates/restarts; don't alter original reveal state or skip gate. Existing screenshot SceneWithUi allocation/write pipeline is reused correctly, and no explicit progress/ready manipulation was observed.

No runtime test performed after these fixes.
