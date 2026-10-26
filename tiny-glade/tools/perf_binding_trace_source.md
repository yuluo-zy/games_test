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

The default command keeps the original behavior and does not require a canonical
source capability. An inherited `TG_PERF_CANONICAL_WALL_LAYOUT` value is removed
by the existing environment cleanup and cannot activate preparation.

For a **new** diagnostic with the reviewed optional rich startup hook, request:

```powershell
python C:\game\tools\perf_binding_trace_source.py <new-rich-frozen-build-label> <new-canonical-series-label> --canonical-layout
```

Only this explicit option adds `TG_PERF_CANONICAL_WALL_LAYOUT=1` after cleanup in
both fresh OFF and ON processes. It neither changes the frozen EXE nor supplies
an annotation to an old session. A refused owner/reference preflight aborts the
real fixture and the series; it must not be retried or treated as a valid run.

The option requires the reviewed shared cold `perf_canonical_contract.py` module.
Its bytes must match reviewed SHA256
`98d7d1cebad9556bb31fcc8fd03811a300ef2b4a5d76eaf65fe7ebf10a9437e4`.
Its resolved file must also be the controller's sibling module; a preloaded
foreign module is rejected even if it has identical bytes.
Its separate `rich_source_contract` accepts caller SHA256
`2d7dd64b1a36d07139d7ebe098b214e8cf1b3cc6ea9f673becf40fd5d79e23f8`
and helper SHA256
`5ccf99448777fdee307a417c1dcf43d12b0f5551938d3f783eace9a21ff14ff9`.
The complete reviewed closure also includes shared input/workload/upload support,
five read-only getter source files and the audited original boot body. It is a
separate rich diagnostic capability, never the formal common-v2 harness.

The read-only current checkout must contain those exact frozen closure bytes;
every file is checked against the selected build's `source.json`, in addition to
the reviewed hash/body checks. The controller saves all closure bytes and SHA256
identities in `canonical-source-archive.json` under the new diagnostic directory
and binds its hash into the summary. It checks the same closure before and after
each child and before finalization. The shared contract module is also included
in controller tool identities. No frozen build/source metadata is rewritten.

Every requested raw session must retain `canonical_layout_input_policy` equal
to `canonical-brick-layout-v1` and the actual typed `canonical_brick_layout`
stamp with phase `fresh_boot_before_first_gameplay_warmup`. The same original
preparation object must be present in both raw input endpoints. Count, fixed
Brick capacity, range ordering/bounds, digest encoding and endpoint association
are checked by the shared validator. Empty owner lanes are legitimate, so owner
count may exceed occupied chunk count. There is no runtime `complete` field and
no invented `prep-v1` compatibility schema. Source capability alone cannot prove
that the real native-owner checks succeeded; a new successful process and actual
report are still required.

Both raw endpoint stamps are retained in each run's summary evidence. They
describe the original startup preparation, **not current/post-measurement
layout**: the real World can reallocate later. All raw semantic inputs, including
the full preparation stamps, must match between processes. The separately
observed per-frame Wall ranges and native command counts remain full unfiltered
diagnostic observations; differences are still displayed and the original
strict count comparator still rejects unequal work. Preparation does not prove
pixels, final layout, CPU gains or production acceptance. No historical pair is
reclassified, and runs are not selected by the resulting ranges/native cost.

The protocol is fixed: static scene, warmup 60, samples 30, OFF then ON, same
frozen EXE. The existing count controller still performs cleanup and requests
`TG_CPU_BILL=1`, the explicit binding mode, and the validation layer. This new
wrapper adds `RHAPSODY_SUBMIT_TRACE=1` after cleanup and, only when explicitly
requested, the canonical startup flag described above; it leaves phase logs,
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
