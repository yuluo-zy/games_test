# Event-loop terminal lifecycle fix

Scope: tiny-glade/crates/tiny-glade/src/game_loop.rs and event_loop.rs only.
No applicable AGENTS.md exists at D:/, D:/game, tiny-glade, crates, or tiny-glade crate; country-core AGENTS is outside this edit scope. No Cargo, GUI, fullscreen or GPU run was started by this agent.

Problem: run() preserved the first Result error but shutdown_done was set only on complete cleanup success. Queued RedrawRequested, AboutToWait, Resumed/Suspended/input callbacks continued touching a closed or partly closing renderer; subsequent device_ready assertions could obscure the original error. The original device_ready check is preserved.

Implementation:
- Shared main-thread LoopLifecycle Running/Stopping/Stopped, Rc shared with run() for the final original anyhow error object. Native Result error source chains remain intact; later cleanup errors are logged as secondary.
- main_loop checks terminal state before any event dispatch. Terminal callbacks only request OS exit; LoopExiting may retry unfinished GPU cleanup. No egui/App update, mapped cursor write, resize, resume, suspend or redraw scheduling after stop.
- dispatch latches Stopping immediately on returned Result failure, before cleanup. Only embedding/test hosts whose hook permits unwinding can also convert callback panics into errors. Normal main::launch installs panic_reporter::setup_panic_hook (main.rs:152), whose first-panic branch reports and aborts (panic_reporter/mod.rs:129); production panic does not reach catch_unwind or ordered shutdown. The original panic hook, assertions and crash-report business remain unchanged. No fake ready or skipped normal frame branch.
- shutdown latches stop first, clears unused OS inputs, consumes gameplay preparation/save/screenshot/cursor cleanup once in the existing dependency order, and always attempts GPU cleanup even if earlier cleanup returns an error. Panics can continue through these phases only under a hook permitting unwind; normal launch aborts as specified above. GPU cleanup alone may retry until complete; no repeated screenshot write/save lifecycle replay. Complete GPU shutdown is terminal even when CPU cleanup reported an error, with the error still returned.
- Drop shares the cleanup state and original-cause sink.
- frame(), queue_os_event(), dispatch_os_event() independently reject terminal work.

Five meaningful no-GUI CPU regression cases are added to game_loop::frame_tests:
1. Actual dispatch gate rejects queued redraw/wait/resume/suspend/resize/focus/close/motion actions after failure and preserves typed io::Error cause over cleanup failure.
2. With the default unwinding test hook, panic from an intact device_ready-like assertion latches terminal before cleanup. This does not prove production panic cleanup.
3. Normal callbacks continue until normal close; completed cleanup is idempotent.
4. Pending GPU shutdown allows GPU-only retry, disallows CPU cleanup replay and resume.
5. Under an unwinding embedding/test host, cleanup callback panic is converted to an error without overwriting prior render failure; subsequent cleanup phases still execute. The production abort hook cannot be validated by this in-process case.

Cross-agent seams for explicit launch window policy are also connected as requested by audit_building_terrain/central:
- apply_pending_video_settings reapplies LaunchDisplayOverride to canonical display before native apply_display_config.
- Both settings write paths share persistence_config (including no-repro fallback).
- ToggleFullscreen ignores changes while force_windowed is true.

Validation status: source reviewed by this agent only; no test pass claimed. Central owns Cargo and independent lifecycle/window policy review. Real WSI terminal behavior remains to be run centrally with explicit windowed/no saved config mutation, if authorized.

## Central review correction: panic boundary

The central reviewer correctly identified that normal launch installs an aborting panic hook. The Result-failure terminal guard is the actual production fix, preventing the later queued redraw/device_ready panic after a returned load/render error. It does not recover the first production panic, and no orderly panic-shutdown claim is made. The broad catch boundaries are retained solely for unwinding library/test hosts: they do not change an installed panic hook or prevent abort; removing them is not required to preserve normal launch semantics. If production panic recovery is ever requested, it needs a separately reviewed crash-report contract and out-of-process verification. No such change is authorized or included here. Central reported 14 lifecycle module tests passed before this comment-only correction; this agent ran no tests.
