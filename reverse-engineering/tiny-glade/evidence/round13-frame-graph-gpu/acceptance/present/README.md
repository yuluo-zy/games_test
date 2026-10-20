# Actual nonempty gameplay WSI candidate

`gameplay-present.patch` is a four-file candidate. No production file was modified by this worker and no Cargo command was run. The new fixture parses through rustfmt and the patch passed `git apply --check` against its recorded base snapshot; recheck after the central pull/integration.

The executable is `original_gameplay_present`, with `harness = false` so the real winit event loop runs on the process main thread. It uses an isolated account, the full original packaged intro and the existing `run_game_thread` + `GameLoop::new`. All device/runtime setup, the separate loading App, async gameplay preparation, real LoadingGameComplete check, actual warmup and handoff remain in production APIs. It forwards every real OS event through `GameLoop::main_loop`, which calls `GameFrameDriver::frame` with the real `ProductionRenderer` host. There is no manual `App::update` loop, fake RenderHost or manufactured load/reveal completion.

The renderer's existing `any_present_successful` is cumulative across loading/gameplay, so it cannot prove a gameplay present. This candidate adds an opt-in diagnostic with no default game behavior change: only tickets produced by the real present path while MainCamera and ActiveSession exist are tagged; only `PresentTicket::is_complete` **and** `is_present_complete` success increment the world-completion count. In this diagnostic mode unfinished world presentation tickets remain owned until completion. Successful swapchain shutdown also confirms completion of remaining tagged tickets before counting them. Rendering/queue idle alone is never counted as WSI completion.

After the real handoff, the fixture checks the actual three authoritative walls, construction/publication failures, brick allocations/upload status/nonzero ranges/dirty state, seven global proxies, all their physical index owners, and 7/7/4 wall list admissions. It then requires three **additional** completed world presents and matching normal gameplay frame returns. LoopExiting must complete production shutdown before the executable returns success. A panic, renderer/worker error, premature close, 600 seconds or 3600 redraws returns failure. These bounds cannot preempt a blocked driver call; the central process watchdog should therefore also use a bounded timeout (for example 720 seconds).

Run only through the single central Cargo owner, after source freeze, from `D:/game/tiny-glade`:

```text
cargo test -p tiny-glade --test original_gameplay_present --locked --offline -j 2
```

Set `TG_INSTALL_ROOT=D:/game/ljxsj_92385/Tiny Glade` and `RHAPSODY_REQUIRE_VULKAN=1`; do not enable `TG_ALLOW_MINIMAL_WORLD_FRAME`. A successful run establishes actual original-intro content reaching production world WSI completion. It still does not establish pixel equivalence or all tail/interactions, and does not advance excluded T13.
