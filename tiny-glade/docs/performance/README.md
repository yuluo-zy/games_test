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
