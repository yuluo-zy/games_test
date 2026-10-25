# V4 production queued uploads

Implemented one queued CPU front end over `TransferManager`'s existing native upload kernel. Both `upload_buffer` and `flush_uploads` call `submit_upload`; the queued path is used by Window Lab S07 and the real Tiny Glade `BufferHost` GPU-only sparse-update consumer.

## Production behavior

- `enqueue_upload(buffer, offset, bytes, UploadOptions { key, epoch, priority })` accepts a bounded dirty interval and returns a CPU ticket in `Pending`. It publishes no point until native submit succeeds. `flush_uploads(max_submits)` is pumped by the production frame/service owner; it does not require another worker thread or renderer.
- Physical buffer identity (and validation of its actual device generation), caller logical key and epoch define a task. Adjacent/overlapping dirty regions coalesce; overlapping bytes use the latest accepted write. Disjoint gaps are not copied. Disjoint dirty regions can share the same packed staging/native submit.
- Work on the same physical target keeps acceptance order, including across different logical keys. Independent targets use priority then FIFO. Under memory pressure the service first trims CPU-exclusive task envelopes and processes device maintenance; low-priority work then defers, while High work can use the existing staging capacity.
- An accepted newer epoch cancels older pending work of that normalized key. Stale requests remain rejected after previous work has been submitted. Already submitted tickets retain their real point and cannot be cancelled. Cancellation of one coalesced ticket cancels the pending native job as a unit.
- Default CPU limits are 64 pending jobs, 128 live key histories, 128 outstanding task slots, 64 dirty regions/job, and 32 MiB known host capacity (including replacement-payload peaks). Tasks, failure envelopes, payload/region backing, key/task tables and local-budget metadata have device CPU budget reservations. A live escaped ticket prevents slot reuse; an escaped failure diagnostic retains its separate real allocation and reservation. Key histories use weak physical identity and reclaim dead resources rather than evicting a live watermark and accepting stale data.
- Staging fullness leaves accepted work pending and reports deferral. Native/allocation errors remain typed on the failed ticket. Known DeviceLost rejects new work and fails all pending tickets without inventing points.
- Readback envelopes are now allocated once per slot and reused only when CPU-exclusive and GPU-completed. External ticket clones prevent reuse. Their known CPU allocation credit follows the last actual owner, including after the manager is dropped. Normal submission entry performs bounded completed-object maintenance.

`upload_buffer_ranges_wait` validates every sparse range before work, splits larger payloads into bounded batches, and waits only on the batches' actual points. Tiny Glade's GPU-only `BufferHost.update_buffer_bytes` uses that entry with 1 MiB chunks, one staging slot, High priority and monotonic request epochs. It preserves completion-before-return. Existing mapped-buffer behavior remains delegated to the device adapter. Invalid ranges are checked before creating the lazy service.

Window Lab S07 now enqueues four dirty parts for each 1 MiB chunk, pumps the same service, and reports actual merged requests, submitted jobs, deferrals, cancellations and peak queue CPU capacity. Its GPU graph/display/readback reference is unchanged. Shutdown explicitly cancels an unsubmitted tail before reporting completion.

## Validation

| Evidence | Result |
| --- | --- |
| `cpu.log` | 11/11 CPU tests: exact dirty overlays/gaps, physical-target ordering, priority/pressure, expiry, failed-admission rollback, ticket backpressure/recovery, bounded live-key history, failure ownership and exclusive pressure trimming |
| `window-cpu.log` | 24/24 Window Lab CPU tests |
| `strict-gpu.log` | 6/6 actual MoltenVK GPU tests, zero SKIP/VUID/validation warnings or errors |
| `tiny-sparse-strict-gpu.log` | 1/1 actual Tiny Glade consumer GPU test, zero SKIP/VUID/validation warnings or errors |

The native tests establish real points, exact GPU dirty-region/gap results, high-priority independent-target ordering, actual blocked-queue staging deferral/recovery, nonblocking host-use rejection, DeviceLost protocol failure, and stable/recoverable readback-envelope CPU ownership. The Tiny consumer test submits an overlapping sparse payload larger than 1 MiB in two bounded batches, checks completion before return, verifies the untouched holes/prefix/tail, and checks invalid ranges submit nothing. None uses device/queue idle.

`native-build-manifest.json` binds both retained binaries to stable build-source fingerprint `8758d39656e9efb36aad4cd80c260511fd49b75b5d331366ffa0360d41a4f4ef`. `strict-gpu-summary.json` records exact binary SHA-256 and results. The shared source tree moved after this source window was handed back; its `source_stable_since_build=false` is intentionally preserved. These results validate the saved snapshot and do not replace the eventual fully frozen module matrix.

## Remaining acceptance

Run the updated S07 real-window 360-frame fixture from the final identity-bound Window Lab build. This domain ran CPU window checks and native GPU transfer/host tests; it did not run a new visible window or claim M4/module PASS. Final integration must also include the subsequently planned host range ledger and per-subresource graph synchronization work. Windows and screen-visual permission limitations remain separately tracked by the root task.
