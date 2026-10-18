# Current bindless descriptor pressure: bounded read-only analysis

This is analysis of the user's current refactored runtime, not a proposal to recreate the original rhapsody. No production source changes are made here.

## Actual observations

- v33 reached `Nani fill unsorted instance indirection / parameters.with_push / Timeout`; heap aggregate had `peak_live=256 live=256 retiring=0 capacity=832 free=576`. The exhausted kind was StorageBuffer; free slots in other kinds cannot satisfy that request.
- v34 storage budget 4096 was rejected by the actual device layout support check; `maxPerSet=1212` was reported by the parent run. Do not bypass that support check.
- Current host candidate uses 600 storage buffers with 256 sampled images + 256 storage images + 64 samplers (1176 total). It is bounded, but full two-round scene pressure is not yet verified.

## Workload lower bound without removing original commands

For default native fine classification (two fine kinds and 25 non-fine kinds), the first round has `F=8` fine lists and `U=100` non-fine lists. HZB round has `F=2`, `U=25`. The original master emits these payloads even if an indirect command ultimately has x=0; the work cannot be reduced for a diagnostic fixture to fit the heap.

| Original callback work | Payload descriptors, first round | HZB round |
|---|---:|---:|
| prepare non-fine commands | 100 | 25 |
| fill unsorted + sorted | 108 | 27 |
| non-fine prefix | 100 | 25 |
| allocate/group/subtotal/generate fine | 32 | 8 |
| fill non-fine final indirection | 100 | 25 |
| subtotal | 440 | 110 |

There are also fixed dispatch producers/clears, actual live cull batches/ranges, actual per-pass shader resources and uniform blocks, and frame globals. Thus two Nani rounds alone require more than 550 distinct immutable payload descriptors. Real main/shadow/transparent draws and other world compute work are additional. A 600 storage budget is a measured stopgap for bounded validation, not proof of full game capacity.

`PreparedBindlessParameters::with_push` clones the same resource leases but allocates a fresh constants slice and **one descriptor for each unique immutable parameter block**. `BindlessHeap::bind_pass` retains each lease in the recorded command until completion. Overwriting a descriptor or recycling it before completion would change already recorded GPU work and violate existing lifetime contracts.

## Safe first improvement, without shader ABI changes

The current `BindlessParameters.cache` is local to one parameter construction. A new pass rebinds the same immutable FrameConstants chunk and same preset views through `FrameGlobals::write_to`, allocating equivalent descriptors again. Repeated buffer/view resources across passes are similarly duplicated.

A bounded **same-heap Weak lease memo** can deduplicate resource leases using the already established resource identity: real Vk object + exact offset/range or view/layout + resource kind + formal uniform-upload role + device generation. Store Weak references so the memo cannot extend GPU owner lifetime; a cache hit upgrades the actual current lease, whose descriptor generation and same-heap device are still checked. Command retention remains unchanged. Limit memo capacity and prune expired entries; do not grow an unbounded global map. This is a current heap resource reuse feature, not a second heap/backend or native ABI claim.

The formal uniform-upload role matters: dynamic-constants chunks may be retained through `retain_upload_read` only through the existing validated constants-ring path. A dedup key must not alias an ordinary storage read with that role or allow any arbitrary buffer to masquerade as an upload page.

This reduces repeated resource descriptors and particularly global image-array pressure. It **does not** remove the lower bound of unique payload blocks.

## If real workload exceeds the device's single-table capacity

The next architecture feature would need bounded descriptor-set pages **within the current heap layout and ownership system**. Each page would carry its own real set and slot generations; a PreparedBindlessParameters packet and all resource indices it embeds must refer to one page. Bind that real set before its original draw/dispatch. The same existing pipeline layout can admit identically laid-out descriptor sets, and the current four-byte parameter index ABI can stay unchanged. Resource and parameter leases must retain their actual page through command completion; bind validation must reject cross-page leases. Global/ordinary resource memoization must be per page.

This is substantial current-architecture work and should only be implemented after actual full-scope measurements justify it. It cannot be replaced by arbitrarily increasing counts beyond the device limit, weakening lifetime validation, skipping native dispatches, mutating original shader payloads, or adding an independent legacy renderer.

Submitting/waiting between passes alone is insufficient if recorded-frame owners still retain the descriptors. Real page retirement or explicitly proven fine-grained command retirement is necessary; a wait must not be treated as permission to overwrite still-owned heap slots.
