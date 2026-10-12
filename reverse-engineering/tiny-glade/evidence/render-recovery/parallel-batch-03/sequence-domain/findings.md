# R5B sequence-domain findings

Target SHA-256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

## Observed

- `CommandBuffer::begin` (`0x140bed230`) selects a counter at `device+0x668 + ((queue_tag & 1 ? aux_index+1 : 0) * 8)`, atomically fetch-adds it, stores the old value at CB `+8`, and inserts that value into `device+0xa50` buckets. A new CB has tag 0; begin accepts 0 or recycled tag 3 and changes it to tag 1.
- `queue_submit` (`0x140c04e20`) allocates a global submission range from `device+0x7f8`. Each submitted CB receives a timeline value from that range; the value is appended to the queue's pending timeline vector and the CB is queued with `PendingCommand+0x58 = timeline_value`. The returned 16-byte token is `{queue_tag/aux_index, last_timeline_value}`.
- `Queue::update_pending_submissions` (`0x140bf0b90`) queries the queue timeline semaphore and stores the completed value at queue `+0x80`; pending entries are popped while their `+0x58` threshold is reached.
- `Queue::reclaim_command_buffer_resources` validates recycled CB tag 3 and queue identity, then passes CB sequence to `SequentialDisposal::dispose_in_order` and returns the CB to the pool.
- `SequentialDisposal` uses one `u64` sequence prefix and a ready-gap deque. In-order completion removes the same key from the global bucket map, empties the bucket, and drains consecutive markers. Out-of-order completion stores a marker at sequence-relative offset; duplicate markers panic.
- Bucket `insert` and `remove` take a single `u64`; no queue/domain field is present. `dispose_cb_resource` reads the last tracked CB sequence and looks up this same global map.

## Interpretation and boundary

The implementation therefore has two independent monotonic domains: CB-use sequence (bucket/reclamation ordering) and queue submission timeline (GPU completion). The static evidence proves the mapping `PendingCommand.threshold -> queue timeline`, then `completed timeline -> ready CB`, but does not prove how `device+0x668` counters are seeded or whether per-queue entries alias a shared atomic. Until constructor writes are recovered, a reconstruction must reject cross-domain key collisions instead of packing queue identity into the observed bucket key.

## Four-entry follow-up

Read-only MCP retrieval of `0x140bef4a0`, `0x140bdb940`, `0x140bed4c0`, and `0x1418c3b80` completed successfully; `mcp-report.json` retains each response and successful `close_database(save:false)`. No database annotations or saved changes were made.

`create_command_buffer` (`0x140bef4a0`) takes a queue-family/pool index, locks `device+0x9b0`, pops an 88-byte CB from the corresponding vector at `device+0x9c0` (stride 24), or calls `CommandBuffer::new` with the family record from `device+0x658` table `+0x38` (stride 28). It does not initialize or allocate a command sequence. `CommandBufferPool::new` (`0x140bed4c0`) creates empty per-family vectors, not sequence counters.

The range helper `0x1418c3b80` appends every integer in `[start,end)` to the queue pending timeline deque; capacity checks and ring wrapping are ordinary collection mechanics. Its behavior corroborates global submission timeline allocation, but is not evidence of CB-counter initialization.

`GraphicsDevicesForSurface::create_device` (`0x140bdb940`) forwards to `0x140bf2560`, the actual device-construction target. That target was outside the authorized four-entry follow-up and remains unexpanded. Constructor seeds, counter aliasing, and same-resource multiqueue safety therefore remain explicit unknowns. The Rust crate is a bounded mapping candidate, not a faithful device constructor; its zero defaults are fixture choices, and its collision errors conservatively expose the unresolved global-key constraint.
