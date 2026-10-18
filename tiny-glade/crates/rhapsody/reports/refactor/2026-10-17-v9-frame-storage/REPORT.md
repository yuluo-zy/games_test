# V4 cached CPU frame storage: allocation evidence

Final fixed imported-resource serial Vulkan Session measurement: begin, execute and completion each assert zero actual Rust/System allocations across 128 warmed frames. Both GPU copy results match the reference with zero validation errors/warnings. Fresh construction remains 32 allocations / 3467 requested bytes; arbitrary callbacks, parallel recording and the whole FrameLoop are separate scopes. No V9 performance PASS is inferred. The sections below retain the initial CPU checkpoint and native iterations.

## Measured scope and counts

The production serial `ExecutionChain` runs with the existing CPU no-driver backend. The resource case has two imported buffers, eight retained passes, real declaration/plan/state/transaction/recording flow, and allocation-free callback bodies. The process-wide allocator delegates to `System`; thread-local counters count actual Rust allocation/reallocation calls and requested bytes only during the selected interval. Fresh graph construction, report output, and trace symbolization are outside the execute interval.

Each variant warms 300 frames and samples 128. The enabled-pool variants assert zero allocations in every sampled cached execution, rather than merely reporting an arena request counter.

| Case | Before cached execute | After cached execute | Fresh construction after |
| --- | --- | --- | --- |
| Eight passes, no resources | 0 calls / 0 B | 0 calls / 0 B | 20 calls / 1112 B |
| Eight passes, two imported buffers | 14 calls / 649 B | 0 calls / 0 B | 32 calls / 3171 B |

The current CPU adapter with pools disabled reports 30 calls / 1290 B for the resource case. Its fallback table preparation changed during this refactor; this is not a comparison against an unchanged old production renderer. Debug-build timing fields are diagnostic and do not establish the V9 p95/p99 performance gate.

`allocation-counts.json` contains the complete before/after samples and counters. `after-allocation-tests.log` records 3/3 tests, including callback error propagation/event equivalence and 1000 warmed worker-arena leases with zero actual allocations. `lib-cpu-tests.log` records 247/247 tests, including the new frame storage tests. The latter exercise 32 resources and 12 passes beyond inline capacity, 1000 pointer-preserving reuses, CPU owner retention, local backpressure versus host OOM, failed admission, exact capacity release and idle trimming. That larger table test does not claim zero allocations for an entire larger graph execution.

## Production changes

- `rg/frame_storage.rs`: configurable frame count, live/cache byte limits, explicit Timeout backpressure, CPU capacity reservations and pressure-driven idle trimming. Exact vector capacities are charged; the fixed owner envelope is conservatively reserved separately. CPU metadata does not appear in GPU VkDeviceMemory records.
- `rg/graph.rs`: resource population borrows the frame declarations instead of cloning all origins/names; frame instances own reused registry, history, transition and snapshot tables. Dynamic tables have an immutable public projection so a caller cannot grow or move pooled vectors behind the reservation. Their last CPU owner returns the tables.
- `rg/resource_registry.rs`: retained export-index vector replaces per-frame HashSet cloning. The generic standalone export helper remains; the frame-specific moving export shortcut was removed after its sole production consumer was migrated.
- `rg/recorder.rs` and `rg/executor.rs`: the initial resource list borrows an arena slice. Only Copy values/references enter the temporary worker arena; no GPU lease or Arc destructor is discarded by bump reset.
- `rg/vulkan.rs`: production resource backend owns the bounded table pool. Completed frames drain backing entries while retaining vector capacity, preserving created/imported/exported handling and actual resource retirement. Frame metadata and GPU-visible allocations have separate completion/ownership domains.

The default pool admits at most 32 simultaneous CPU frame owners, 4 MiB total capacity and 1 MiB cached capacity, all subject to the shared known-byte budget. `configure_frame_storage` and `frame_storage_statistics` expose the production limits and counts. Live owners retain both allocation and CPU reservation when idle storage is trimmed or the outer pool handle is dropped.

## Verification and identity

- All-features tests typecheck: PASS.
- CPU library suite: 247/247 PASS.
- Allocation/equivalence suite, including ignored explicit measurement: 3/3 PASS.
- All-features, all-targets strict Clippy: PASS, zero warnings.

The retained `cpu-frame-allocation-tests` executable and relevant source hashes are recorded in `identity.json`. The earlier 14-allocation diagnostic was symbolized using its matching binary before edits, but that pre-change executable was not separately preserved; Cargo subsequently replaced it. The raw and already-symbolized traces are retained as diagnostic evidence, without claiming a retained identity-bound pre-change binary.

## Scope remaining at the initial CPU checkpoint

At that checkpoint, the fresh builder still allocates, as counted above. Actual Vulkan Session/FrameLoop receipts, managed binding projections, user callbacks and larger transaction spills are separate allocation scopes and have not been shown to allocate zero by this test. No GPU/WSI/long-run or V9 performance acceptance is inferred from these CPU results. The PGO Arc name ownership change must rerun the same three allocation/equivalence tests before this count is attributed to that later source snapshot.

## Post-PGO CPU recheck and actual Vulkan baseline

The same three allocation/equivalence tests passed after the PGO Arc-name migration (`post-pgo-cpu-allocation-tests.log`); both cached CPU variants still assert zero actual allocations in every sample.

The separate ignored `gpu_frame_allocations` fixture now measures the real serial `VulkanGraphSession` with two actual imported buffers, eight real native copy callbacks, exported host-read results, 300 warm frames and 128 samples. Both 64-byte GPU outputs match their independent reference, and validation reports zero errors/warnings. This is a strict native GPU run, without hardware skips. Current-thread Rust/System allocation scope excludes native driver allocations and other threads.

| Real Session phase | Calls per frame | Requested bytes per frame |
| --- | --- | --- |
| begin_frame | 0 | 0 |
| fresh graph construction | 32 | 3467 |
| execute_frame | 86 | 9758 |
| completion / wait_frame | 2 | 64 |

`real-session-baseline-counts.json`, `real-session-baseline.log`, and `real-session-baseline-symbols.log` retain the measurement and diagnostic stacks. `real-session-baseline-identity.json` identifies the retained `gpu-session-baseline-tests` executable and explicitly records the concurrent root command-pool reset optimization boundary. The actual Session still allocates; the CPU adapter's zero result is not generalized to production Vulkan.

## Native Session receipt implementation (measurement pending)

The production access policy now stores the complete u8 code space inline instead of cloning BTreeMap nodes per frame. `SessionFrameReceipt` exposes immutable pooled contents through Deref; frame ID and stats remain directly accessible. The Session admits receipt capacity before recording/submission, then publishes by writing already reserved SyncPoint storage and cloning its slot Arc. Snapshot vector capacities, nested pre-transition vectors, and the slot envelope share a CPU reservation. SnapshotStatic remains shared under the existing plan/cache ownership; this receipt reservation does not claim to re-charge that complete static planning allocation.

The configurable default is 32 simultaneous CPU receipt slots, 4 MiB local total storage and 1 MiB idle cache. A caller retaining receipts or clones keeps its slot and reservation alive after GPU completion and after Session destruction. Only an exclusive Arc slot may be rewritten; exhausted CPU ownership returns Timeout before submitting. Pressure trimming drops only idle slots.

`receipt-storage-check.log` records the all-features tests typecheck. `receipt-storage-cpu.log` records two targeted owner/budget tests, including 12-pass nested storage, 1000 pointer-preserving reuses, retained-clone backpressure, local Timeout versus OutOfHostMemory, and last-owner budget release. The initial CPU test setup incorrectly treated the default unlimited budget value as a finite size; the test now explicitly sets a limit. No production panic was involved. Actual native zero-allocation evidence still requires the next complete integration run.

## Combined native iteration: 86 to 27 allocations

The next identity-bound strict native run passed all GPU copy references, zero validation errors/warnings, and the two-slot receipt admission/recovery test. Across 128 cached real Session frames, execute fell from 86 calls / 9758 requested bytes to 27 calls / 2912 requested bytes. begin_frame and completion/wait both measured zero. Fresh graph construction remains separately measured at 32 calls / 3467 B. The CPU adapter's three allocation/equivalence tests also passed again, along with 12 targeted receipt/transient/timeline CPU tests.

`real-session-optimized-{counts.json,log,symbols.log,identity.json}` and the retained `gpu-session-optimized-tests` binary preserve this iteration. The remaining 27 actual allocation stacks are completely captured: nine `add_transition` buffer barrier growths of 224 B; fourteen `raw::barrier::pre` temporary lists of 12 B; three owner-vector growths of 64, 128 and 256 B; and one 280 B native SubmitScratch spill caused by collecting duplicate use cells before deduplication. These are active implementation work, not a zero-allocation PASS.

## Final fixed-resource native execution: zero asserted

The final frozen-source binary (`real-session-final-identity.json`) passes strict native zero-allocation hard assertions for every measured begin_frame, execute_frame and completion/wait sample. The same test also verifies actual GPU rejection/recovery for exhausted receipt slots and the newly bounded owner-list pool: no callback or queue submission on owner admission failure, exactly one retire callback, CPU lease returned, then a successful GPU copy after restoring the configured limit. Both 64-byte outputs remain exact and validation stays at zero errors/warnings.

The production owner collector checks out a real bounded `TempListPool<ResourceOwner>` lease in serial recording and on each parallel worker. It drains references into command-owned GPU keepalives before returning only the empty CPU list capacity. Its default limits are 64 simultaneous CPU lists, 65536 entries per list, 16 MiB local peak storage and 4 MiB idle cache. Pool storage has CPU budget tickets, peak growth admission, idle-only pressure trimming and last-lease lifetime; no Arc destructor is placed in the Copy bump arena. `PassContext::keep_alive` reports the original admission error through the existing callback error channel. CPU-only adapters retain the allocation-free empty fallback.

Evidence: `real-session-final-zero-{counts.json,log}`, `final-owner-build.log`, `final-owner-strict-clippy.log`, `final-cpu3.log` (3/3), and `final-targeted-cpu.log` (15/15 including owner/receipt/projection and normalized use-cell merge). The retained final GPU and CPU executables plus source hashes are recorded in the identity file. The strict Clippy run preceded the final Legacy configuration addition; its types and tests are included in the final successful no-run build, with the repository's final matrix responsible for the complete last-snapshot lint.

This closes the measured cached fixed-resource **serial Session execute** gap (86 → 27 → 0 calls). It does not assert allocation-free fresh construction, arbitrary user callback bodies, parallel task scheduling, native driver internals, or the complete Window FrameLoop. Timing values in these debug diagnostic runs do not establish the controlled V9 p95/p99 regression gate.
