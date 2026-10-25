# Async startup diagnosis review (2026-10-18)

Scope: read-only audit of the current standalone macOS startup host and v30 failure. No production edits, no Cargo/GPU run, no country-core systems/resources changes, and no shared analysis-database mutation.

## Proven source and original behavior

The retained B loader `0x1400d7440.c` performs `SubApp::update`, resolves the canonical `Events<LoadingGameComplete>` owner and checks both event buffers (the two nonzero lengths at +0x10/+0x30). Once an actual completion exists it logs the loading-complete state, calls `tiny_glade::render_loop::render`, and only then sends the SubApp. Current `GameplayPreparation` preserves this producer → completion event → successful warm-up → channel handoff order. The diagnostic catch does not fabricate completion or bypass warm-up.

Current `production_renderer::warm_up_gameplay` unconditionally returns `world_consumer_missing` (`original compile_main 0x140161cd0: world FramePasses consumer is not connected`). This is a real explicit integration blocker. It must not be suppressed or turned into success to show gameplay.

The worker owns the sole sender; `GameplayPreparation` owns the receiver. Dropping the spawn `JoinHandle` detaches the thread, not the sender or work. A normal validation/load/warm-up `Err` is stored as the result and passed into `sender.send(result)` with the receiver still present. The logo state can hold a successful SubApp pending but does not consume/drop the receiver, and the cancellation flag is only set when `GameplayPreparation` drops. There is no source-proven normal-error swallowing branch that explains v30's `Disconnected`.

## What v30 proves and does not prove

`/private/tmp/tg-original-window-startup-v30.json` records exit 1, no timeout, SHA-256 `dc80d71a311c135561ffad4cdb0e1324bc47cdd94a7367d4837bdd082d0c0f3d`. The log ends with `original async gameplay loader stopped before returning its SubApp`. The root confirmed the CLI had only `--override-save-root /private/tmp/tg-main-chain-v30-host-save`, with original asset root, strict Vulkan, and an eight-frame bound; no `--repro-panic` was supplied.

The channel failure establishes that the sender was gone before a result was queued. The trace does not establish whether an earlier worker update reached LoadingGameComplete, reached graph warm-up, or panicked. It contains no worker panic message. No specific gameplay root cause is accepted from v30 alone.

The existing new catch boundary reports phase and completed-update count for Rust unwinds; this is suitable narrow host diagnostics. However `main.rs` installs the recovered global panic hook in the ordinary CLI path, and this hook calls `std::process::abort`. A hook which aborts runs before Rust unwinding, so `catch_unwind` cannot intercept such a panic. Since v30 exited normally with an anyhow backtrace rather than an abort and no panic-hook text, a conventional Rust panic under that hook is not proven. Do not change the global crash policy or invent a missing resource until the rebuilt host exposes a concrete error.

## Definite diagnostic noise

`rhapsody::Error::invalid_usage` logs when the error object is constructed. Seven existing `Option::ok_or(Error::invalid_usage(...))` call sites construct and log the error even on a successful `Some` path: allocator.rs six sites and suballoc.rs one. They explain spurious allocation-statistics/ID/suballocation messages and the successful `records.remove` path's “unknown or released memory ID” message. These messages alone do not prove real failed allocation or double free.

A bounded seven-site `ok_or_else(|| Error::invalid_usage(...))` patch and exact before/candidate hashes were prepared under `TG-2e9762ec80651e1568/main-chain-frame-20261018/startup-diagnostic-lazy-errors`. It changes diagnostic evaluation timing only, retaining allocator state, defaults, result values and error category. Root is the production integrator; no production files were changed here.

## Next decision point

Run the rebuilt standalone host with the current catch boundary and a new isolated save root. Preserve its exact binary hash and failure before choosing an implementation change. If it returns the known world-graph Unsupported, the loading producer gate was reached and the next work is the real world consumer. If it reports a specific update/producer failure, fix that owner in coordination with the separate country-core systems/resources refactor agent. If the channel still disconnects without any catch message, add one-shot worker entry/result/send diagnostics or retain its JoinHandle for failure observation; do not weaken loading/ready gates.
