# Default suballocated Buffer Drop: heap diagnostic, not performance acceptance

This staged addition changes no production code, dependency, allocator policy, probe or existing test assertion. No Cargo/GPU run has occurred while preparing it. The diagnostic has no measured counts yet. Existing `unused_dedicated_buffer_drop_does_not_allocate_a_statistics_snapshot` remains byte-for-byte unchanged and applies to Dedicated only.

## Actual objects and counted phases

The new `suballocation_drop_heap_diagnostic` test target reuses the existing `tests/allocation_counter` and strict initialization helper. It uses default features and `DeviceConfig::headless()` for the default case, verifies the unchanged default selects Suballocated, and explicitly selects Dedicated only for the named control. No Buffer allocation-kind getter is added. The existing metadata-only `host_range_for_test` delegates to MemoryBlock: direct suballocations expose a finite `(offset,size)`, dedicated memory returns `(0,WHOLE_SIZE)`. Every actual Buffer is checked through this hook, without mapping or reading GPU memory. On a fresh device, every GPU memory range belongs to these actual Buffers: additionally requiring `resource_range_bytes == suballocated_used_bytes > 0` and `dedicated_bytes == 0` proves the default objects did not silently use dedicated fallback. A device flag alone is insufficient. The Dedicated control requires the complementary public accounting.

1. Unused small Buffer: create two real 64KiB native Buffers; keep an anchor alive and count only dropping the other owner. Assert the unused range really disappears immediately. This excludes final-shard teardown from that common-case measurement.
2. Fragmented completed reclamation: create128 real64KiB Buffers, free index mod8==3 before counting (16 holes); mod8==1/5 are32 real GPU fill targets and the80 other owners stay alive. For the default case the test requires one actual native memory block, sorts the existing hook offsets and checks all128 native ranges do not overlap before assigning these roles. This creates additional separate holes on reclamation, but does not guarantee that a HashSet grows. Keep all actual allocation strategies and calls unchanged.
3. Submit real fills retaining the target owners, wait on the actual timeline with a10s bound, confirm completion and drop outer target references. Measure the completed PendingCommand Drop separately: target native payloads and command must actually enter the reserved retirement queue, and no target memory range may disappear yet.
4. Measure explicit `try_collect_all_completed()` separately. The CPU caller knows the GPU point already completed; the measure still includes the real collector's completion polling, command teardown, native buffer destruction and dependency free path. Assert queued items/ranges are genuinely reclaimed, while anchors keep the allocator alive. Cleanup/trim happens outside counting.

There is no artificial GPU gate, logger, new worker, CPU clock or ongoing GPU work inside a counted phase. The explicit full completed-object drain is diagnostic/teardown, not a proposed normal-frame pressure policy.

## Counter meaning

Each `SUBALLOCATION_DROP_RUST_HEAP` line contains `suballocation-drop-rust-heap-v1` JSON. Default and Dedicated run independently through the same workloads. There are six phase records in total: two unused Drop, two completed Pending Drop, two fragmented completed collector.

The reused allocator is the test binary's existing TLS `CountingAllocator<System>`, counting calling-thread Rust global `alloc`, `alloc_zeroed` and `realloc` attempts and requested bytes. It does not count deallocations, other-thread allocations, Vulkan/C/driver heaps, live/peak Rust heap or production Tiny Glade mimalloc costs. The first32 allocation sizes are retained; `allocation_size_prefix_truncated` explicitly reports longer sequences, while total calls/bytes still include all attempts. Logging, JSON building, snapshots, setup, submission, completion wait and cleanup are outside `measured`. The existing Sample timer still runs inside that shared helper, but its elapsed_ns is deliberately not exported or interpreted as CPU time. A count is not an explanation of every allocation site: call stacks or separate dependency instrumentation would be required to attribute each sample.

No allocation-count threshold is invented for the new diagnostic. Zero and nonzero observations are both preserved as observations, never called frame performance or a default zero-allocation contract. Existing stronger dedicated/retirement queue assertions are not removed or weakened.

## Source chain and delayed-retirement question

- `raw/buffer.rs:1113` Drop detaches MemoryBlock once and passes Doomed::Buffer with its original last-use set and pre-reserved ticket.
- `raw/allocator.rs:253` into_deferred moves the actual SubAllocation and marks the allocation retiring; it does not free the range there.
- `device/lifetime.rs:37` retire_reserved destroys never-submitted (`SyncSet::EMPTY`) objects immediately; submitted objects are queued for maintenance.
- `device/lifetime.rs:free_deferred` eventually drops DeferredMemory::Sub.
- `raw/suballoc.rs:323` free_locked calls the pinned gpu-allocator free under its memory-type shard lock; native retire-queue locks have already been released.
- gpu-allocator0.27.0 `src/allocator/free_list_allocator/mod.rs:293..309` assigns a newly allocated disabled-Backtrace Arc per range free and inserts into its free-chunk HashSet; insertion may grow. The test measures real Rust calls, not a guessed fixed number. Default free is distinct from queue admission's pre-reserved-node guarantee.

A source-only possible design is to enqueue even unused native objects using the already reserved node, postponing native destruction/suballocator free until bounded maintenance. Moving ownership can shorten the immediate Resource Drop path without a Device back-edge, premature release or queue reallocation. This is NOT implemented here. It retains native objects, suballocated ranges and budget credits longer, can increase peak usage/admission failures, changes when unused ranges become reusable and needs pressure/cancellation/shutdown analysis. Most importantly, the dependency still performs heap work when maintenance eventually frees the range: delaying it does not remove or make that work HostOOM-safe, does not prove total CPU improvement, and may merely move a stall into another frame. A successful zero-allocation queue enqueue cannot be relabelled zero-allocation native destruction. Any future change needs its own correctness, budget and whole-application cost evidence; no unbounded drain or hidden cost shift is proposed.

## Root-owned serial run after the frozen W1 series

Do not mutate a frozen source checkout or run concurrently with Cargo/GPU/CPU measurements. Review the two-file patch, record current commit/content and requested target EXE/PDB identities, then use the pinned toolchain and default dependencies:

```powershell
$env:RHAPSODY_REQUIRE_VULKAN='1'
$env:VK_INSTANCE_LAYERS='VK_LAYER_KHRONOS_validation'
$env:CARGO_INCREMENTAL='0'
cargo test --locked --offline -j1 -p rhapsody --test suballocation_drop_heap_diagnostic -- --nocapture --test-threads=1
```

Require exactly2 passed,0 failed/ignored and successful real Vulkan initialization; feature-disabled/zero-test/filtered/SKIP is not a pass. Preserve every JSON line, original log and source/EXE identities, and inspect validation errors. Run the unchanged retirement_reservation target separately so this diagnostic cannot replace its existing assertions. There is no timing, frame p95 or allocator-switch acceptance attached to these results.
