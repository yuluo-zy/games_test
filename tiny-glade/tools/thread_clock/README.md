# Windows thread CPU calibration

This directory makes the external thread-counter diagnostic reproducible from
a checkout. It does not run Cargo, initialize Vulkan, change timer resolution,
priority or affinity, or certify the production rendering-thread 16 ms target.
The native reader, CNG workload and original 17-test source retain their original
bytes; `source-manifest.json` records lineage and the portable controller changes.

Use the tested Windows x64 Python 3.12 environment and an installed WPT `xperf`
at the location configured by `tools/perf_windows.py`. Elevated ETW permission
is needed for capture. Another NT Kernel Logger must not be running.

## New calibration on a checkout

Run from the repository root, choosing a new series name:

```powershell
python -B tools/thread_clock/predeclare.py clock-new-host-1
```

This writes only a new `.perf/thread-cpu-calibrations/clock-new-host-1.predeclared.json`
and prints `command_argv`. It records a fresh nonce, UTC declaration, current
Python/xperf/source identities and host binding. It hashes one 1 MiB input chunk;
it does not create workers, run CNG hashing, start ETW or begin the six-run series.

Review that declaration and run its exact printed argv separately. For example:

```powershell
$declaration = Get-Content -Raw .perf/thread-cpu-calibrations/clock-new-host-1.predeclared.json | ConvertFrom-Json
$calibrationArgs = @($declaration.command_argv | Select-Object -Skip 1)
& $declaration.command_argv[0] @calibrationArgs
```

The fixed protocol is three alternating pairs, continuous/burst, burst/continuous,
continuous/burst, with four workers hashing 4,096 chunks of 1 MiB each. Burst
includes real waiting/syscall/scheduling work; only native hash input, algorithm,
bytes, chunks and final checksums are required equal. Each run checks the same
declaration/tool/source/host binding before it starts. Existing output directories
and declarations are not reused. Reports retain every coarse clock failure.
Exit success does not mean `all_runs_within_existing_2pct` passed.

After reviewing all six new captures, freeze a new policy evidence file using
the normal tool. Supply the exact chronological AB/BA/AB order:

```powershell
python -B tools/perf_headless.py freeze-clock-policy .perf/clock-new-host-1.json --predeclared .perf/thread-cpu-calibrations/clock-new-host-1.predeclared.json --calibration-run .perf/thread-cpu-diagnostics/clock-new-host-1-1-continuous --calibration-run .perf/thread-cpu-diagnostics/clock-new-host-1-1-burst --calibration-run .perf/thread-cpu-diagnostics/clock-new-host-1-2-burst --calibration-run .perf/thread-cpu-diagnostics/clock-new-host-1-2-continuous --calibration-run .perf/thread-cpu-diagnostics/clock-new-host-1-3-continuous --calibration-run .perf/thread-cpu-diagnostics/clock-new-host-1-3-burst
```

Keep the declaration, reports, native readings, official CSV, raw CSV/ETL and
their referenced identities. `.perf` remains ignored; distribute those artifacts
separately if another person must verify results. Source alone can generate a
new calibration, but cannot reproduce old measured values without old artifacts.

## Boundaries

- This is a new portable helper version. Do not edit/move the original staging
  helpers, rewrite their old SHA records, rebase an old evidence JSON, or call an
  old strict-invalid round passed. Use a new declaration and six new captures.
- Runtime identity keys are repository-relative. The declaration and all six
  captures use the same twelve-module closure, including `perf_build_target.py`
  and `perf_clock_policy.py`. Git commit/dirty state is recorded when available;
  unknown Git state is explicit, and runtime source content is always hashed.
- The portable series rejects a declaration from another observed host using a
  nonce-salted MachineGuid/hostname hash. That is not physical attestation: OS
  clones may share identifiers. The existing `perf_headless` policy itself still
  compares CPU/OS fields; this package does not strengthen that older CLI's
  same-model-machine discrimination or authorize copying old qualifications.
- The formal owner GetThreadTimes 2% gate remains mandatory. Microsoft CSwitch
  compares an independent implementation using the same ETW event stream;
  it is not a second independent clock. No raw counter is corrected.

## Small tests

```powershell
python -B -m unittest discover -s tools/thread_clock -p "test_*.py" -v
```

There are 17 original tests plus seven delivery/path/declaration tests. Four
original tests exercise small Windows API/CNG/child handshakes; none launches
the long calibration, Rust, Vulkan or a real ETW session. Windows-only tests
skipped on another platform do not count as Windows validation.
