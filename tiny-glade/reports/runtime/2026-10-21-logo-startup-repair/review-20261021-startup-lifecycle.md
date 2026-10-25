# Startup lifecycle / handoff independent source review — 2026-10-21

Scope: current shared changes in game_loop.rs/event_loop.rs, App handoff, LaunchDisplayOverride. Read-only; no Cargo, GUI, runtime or new RE. Reviewed current source after render agent's three policy fixes.

## Findings

- P2 hardening: run_game.rs GameplayPreparation::poll obtains `LogoAnimation` optionally. If owner is missing, current `if let Some` bypasses `is_finished` and accepts pending gameplay. Actual make_loading_app always initializes it, so no normal-path failure established; nevertheless a missing mandatory producer should fail explicitly rather than turn into an implicit skip-splash. Use required-context lookup for async path; explicit finish_skipped_splash stays distinct. Sent to central.
- Earlier P2 policy integration errors were observed then fixed by implementation agent: ToggleFullscreen no longer toggles under force_windowed; apply_pending_video_settings applies policy before display change; both direct write paths use persistence_config. These findings are resolved in current source, not outstanding failures.

## Confirmed current source behavior

- `LoopLifecycle` Running→Stopping→Stopped. main_loop checks terminal state **before matching any event**. Queued WindowEvent/RedrawRequested, Resumed, Suspended, DeviceEvent, AboutToWait do not touch egui/App/cursor/renderer. LoopExiting can retry shut_down only. queue_os_event/dispatch_os_event/frame also reject terminal use.
- dispatch callback errors/panics request_stop; run wrapper stores originating anyhow Error object before cleanup. Shutdown errors log without replacing it. Returning lifecycle.take_failure after event-loop closure therefore preserves first callback cause and source chain.
- shut_down marks terminal first, clears pending OS batch, consumes preparation/save/screenshot CPU cleanup once, attempts GPU regardless returned CPU failures. On GPU failure phase remains Stopping; LoopExiting/Drop can repeat only renderer.shutdown; successful GPU marks Stopped even if a CPU cleanup error must still be returned.
- Preparation shutdown cancellation joins actual worker before renderer destruction, disposes pending/queued SubApps using canonical save-thread shutdown, and preserves originating producer failure over subsequent cleanup error. No additional gameplay update occurs after cancel observed.
- ProductionRenderer existing shutdown retry path leaves `closed=false` until all acquire/submits/presents/session clear/device idle succeed. Rhapsody swapchain shutdown explicitly retains generations for retry. No new fallback that treats timeout as completion, ignores GPU readiness, or resumes after shutdown.
- LaunchDisplayOverride inserted on gameplay App before GameLoop::new. make_loading_app clones that immutable policy, while prepared original gameplay SubApp retains its own same policy. Six runtime owners (config/input/window/monitor/frame/adaptive) move at handoff; gameplay policy still exists after incoming GameConfig replaces it. No full shader/resource merge; each App keeps independent shader library and actual shared device owners.
- Normal async wait still requires actual LoadingGameComplete + original GPU warmup before publishing; poll only releases wait stage when pending is real; finishing final Logo stage gates handoff when Logo owner exists. Explicit skip-splash path alone moves pending egui and sends StartupRevealFinished.
- No added sleeps, worker spin, fake readiness, extra resource copies per frame from these lifecycle changes. Rc/Cell check adds constant tiny callback overhead. Launch policy cloned only when settings flags!=0; persistence GameConfig clone only on actual write, not idle frame.

## Limits

No real event-loop queued event sequence, device timeout/retry, shutdown screenshot failures, or fullscreen/persist/handoff behavior was executed. Existing new unit tests model lifecycle state, not real winit/WSI queue dispatch; central runtime acceptance remains necessary. CPU cleanup is deliberately one-shot; this review does not claim arbitrary panicking third-party callbacks can be safely replayed.
