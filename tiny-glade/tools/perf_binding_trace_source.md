# Frozen OFF/ON trace and Wall source investigation

This independent controller reuses `perf_binding_scope_diagnostic.capture_mode`
and `parse_native_pass_trace.parse_trace`. It does not modify either tool or the
six frozen CPU runtime policy tools. It has not run Cargo, GPU work, or an actual
test executable. Its stdlib tests use synthetic captures only.

After the coordinator reviews and installs the new tool, invoke it against a
frozen **rich** fixture that contains the typed Wall source bill and the reviewed
native pass trace instrumentation:

```powershell
python C:\game\tools\perf_binding_trace_source.py <frozen-build-label> <fresh-series-label>
```

The protocol is fixed: static scene, warmup 60, samples 30, OFF then ON, same
frozen EXE. The existing count controller still performs cleanup and requests
`TG_CPU_BILL=1`, the explicit binding mode, and the validation layer. This new
wrapper adds only `RHAPSODY_SUBMIT_TRACE=1` after cleanup; it leaves phase logs,
GPU timestamps, and ETW disabled. CPU_BILL automatically activates the typed
Wall source diagnostic. This code does not add another render host or build.

Frozen build/source/EXE/PDB/FMOD evidence and controller dependency hashes are
checked through the existing qualified path before and after each capture.
Exactly one successful test, zero ignored tests, and zero Vulkan diagnostics
are required. Each session must pass the existing CPU/actual-workload/binding
checks and the new Wall observation association, typed capacity/range bounds,
totals, and digest-encoding checks. Native trace parsing must also pass its
scope/chunk closure and dispatch/pass-count conservation rules.

The original diagnostic `run.json` must retain **`valid=false`**. This is the
existing no-ETW marker, not a field this controller rewrites. A final summary's
`valid=true` means the new diagnostic completed with structurally valid evidence;
it does not mean that OFF/ON work matched. Explicit flags reject clean CPU,
production 16ms, performance-gain, and formal source A/B interpretations.

Outputs use fresh, non-overwritten paths:

- `.perf/headless-runs/<series>-off` and `-on`: unchanged run/session/log;
  complete `native-pass-trace.json`; `native-pass-name-view.json`; SHA256 sidecars
  and an inventory for all run files other than generated hash sidecars/the
  inventory itself. Failure paths also retain available raw artifacts.
- `.perf/binding-trace-source/<series>/summary.json`: both paths and inventories,
  frozen/tool identities, exact normalized actual-workload differences, native
  API differences, and raw ordered Wall source differences. The first failed
  capture stops the pair; the summary remains false.

Raw semantic input manifests, default management modes, and observation keys
must still match. Actual native/workload/range differences are retained as
investigation findings. No graph cost, upload, range, or native field is removed
to make an existing count comparison pass. The earlier failed pair is unchanged.

The full trace preserves unique scope/graph/pass/command identities. The
by-pass-name min/max/count view is explicitly a presentation convenience:
identical names do not establish semantic equivalence. Traces include startup
and warmup and currently have **no receipt association**. Successful recording
is not proof of submission or GPU completion. Wall snapshots are before prepare
and carry real observation receipt keys, but that cannot invent a receipt join
for the independent native trace. Python checks the fixture-declared BLAKE3
digest's encoding without adding a dependency; it compares raw ordered range
arrays directly and independently computes artifact SHA256.

Local tests (no actual target execution):

```powershell
$env:PYTHONPATH='C:\game\tools'
python -m unittest -v test_perf_binding_trace_source.py
```
