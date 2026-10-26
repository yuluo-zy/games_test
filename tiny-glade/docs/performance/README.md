# Windows performance work

Source: codex/render-cpu-cleanup-20261026 at 6957077f9. Independent local repository;
no remote. Source hashes and migrated fixture provenance are adjacent JSON files.

Acceptance: static, many_draws, local_edit, continuous_edit at 1280x720, world
scale 1, fixed 1/60 simulation step, unchanged visual workload. Release builds use
the existing default mimalloc allocator, vsync off, no FPS limit. Warm up 600
observed frames, collect 3000 valid intervals, repeat three times. Final owner CPU
p95 must be <=16 ms in every scene/run; p95/p99 regressions >5% reject a candidate.
Repeat A/B in AB, BA, AB order; expand to five pairs if noise approaches 5%.

Stages: reproducible build; independent CPU probe; Windows baseline; graph query,
projection, scratch and constant optimizations; remaining Collect loops; bounded
retirement/pressure collection; final performance and three 30-minute/10000-frame
stability runs. Every stage has its own commit and evidence; old macOS numbers
are not a Windows baseline. Missing hardware paths and failed/skipped tests remain
explicitly unverified. Sampling, allocation tracing, GPU validation and clean
measurements are separate runs. Do not count clock errors or truncated reports.

Use TG_INSTALL_ROOT to reference the original installation externally. Never copy
original game executables, caches or the full reverse-engineering tree here.
Generated binaries, symbols, JSON and logs belong under ignored .perf/.

## Implemented measurement entry point

Run with a Python 3.11+ interpreter:

    python tools/perf_windows.py calibrate
    python tools/perf_windows.py build win-baseline-v1
    python tools/perf_windows.py run win-baseline-v1 static-r1 --scene static
    python tools/perf_windows.py compare static-r1 candidate-static-r1

Run ETW capture from an elevated terminal. The tool refuses to replace an existing
NT Kernel Logger and stops only the session it successfully started. Every run
retains the raw ETL, scheduling CSV, raw completion points and build identity.
Use --without-etw only for aggregate/overhead diagnostics; it cannot pass p95.

Windows GetThreadTimes on this machine advanced in 15.625ms steps. It is used
only for cumulative crosschecks. Per-frame values come from CSwitch QPC running
intervals, with unioned DPC/interrupt intervals subtracted. A >2% cumulative
crosscheck error, trace loss or missing completion invalidates a measurement.

Current checks: source-only workspace all-target check passed; recorder 10 tests
passed; ETW accounting 5 tests passed; real-window probe target check passed.
Release runtime, probe/ETW overhead, Windows workload baseline, optimization
benefit and final long runs remain unverified until raw run records exist.

## Current gates (2026-10-26)

The production window was actually attempted. It exits with Unsupported because
this UHD 630 driver exposes neither swapchainMaintenance1 nor present_wait/id.
AcquireOnly cannot present and is not a performance substitute. Four-scene
baseline and final performance acceptance therefore remain open. At user direction, source candidates for graph indexing, ownership transfer, Collect convergence and reserved retirement have proceeded with local correctness validation; this does not establish Windows frame-time benefit. A release j2 build also failed
with a country-core rustc allocation failure; the tool now defaults to j1.
System paging, drivers and release optimization settings were not changed.

The scheduler parser was independently compared with Microsoft's xperf cswitch
integrator on a busy/sleep validation workload: relative difference 0.00855%.
GetThreadTimes differs by 7.28% on that workload; both readings are preserved.
The independent check validates the integration, not probe/ETW overhead or game
performance. The official integrator uses the same OS trace, independently of
our interval implementation; it is not a second independent trace capture.

See status.json for exact gates and evidence hashes, and
ownership-audit-20261026.md for the read-only agent's prioritized candidates.

## Source candidates and measured scope

See ownership-implementation-20261026.md, version-index-20261026.md,
collect-migration-20261026.md, pressure-reclaim-20261026.md and
retirement-reservations-20261026.md. The only timing gain established so far is
the targeted debug resource-version query comparison (100,000 queries: 3.793s
scan versus 5.303ms index, about 715x for that operation). No FPS, whole-frame
percentage, release-build improvement or <=16ms result follows from it.
Zero-allocation retirement diagnostics establish specific error-path behavior,
not a frame-time percentage. A real 30-frame original gameplay graph diagnostic
now passes with 720 Collect /90 cross-pass jobs and expected drained descriptor
owners; it is debug, instrumented and headless. Frozen representative Windows
A/B is still needed. See remaining-tasks-20261026.md for the current backlog.
