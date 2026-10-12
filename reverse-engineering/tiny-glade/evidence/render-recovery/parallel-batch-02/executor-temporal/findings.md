# R4 executor and temporal

Identity: original executable SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; batch02 contract, batch01 compiled-state and registry-pool are inputs. All 14 R4 entries were queried through RE-MCP tg_render_arch; successful metadata and 17 decompilations/xrefs are in mcp-success-report.json. Initial lock failure remains in mcp-report.json; successful close used save:false. Game was not launched. PDB void() is a placeholder. Independent LLVM instruction ranges are in objdump-core.json.

## Observed execution chain

execute 0x1400fb5b0 invokes selected compile instance 0x140166510; returns empty on failure. Success moves 0x218 compiled snapshot into execute_compiled, immediately calls retire, restores 0x90 persistent graph and returns 0x30 export map. Country-core compile closure content is not generic rhapsody.

execute_compiled 0x1400fa190 uses compiled+0xa0 as pass count. Flag recorder+0x188 determines whole-graph or one-pass batches; final batch+0x10 is export-barrier flag. Incoming synchronization metadata flushes old pending CBs before attaching waits/signals and tracked semaphore handles. Each batch calls ensure_active_cb then record_batch. Every nonfinal batch calls submit_pending_cbs; final batch remains pending until a later graph or end_frame. A batch is neither intrinsically one CB nor one submit: record_batch can append several CBs through parallel recording. End/export barriers attach to last CB.

After recording, execute_compiled moves graph compiled+0x00..0x90 and Registry compiled+0xa8..0x1b0, calls into_exports, creates ExecutedRenderGraph, drops closures and scratch. Alignment matches batch01: ResourceInfo registry+0x48=compiled+0xf0; pending initial transitions compiled+0x200, not batches.

ensure_active_cb 0x140bebc90 does nothing when pending len recorder+0x50 is nonzero. Otherwise allocate_command_buffer 0x140bef4a0, begin 0x140bed230, push one stride0x58 CB. submit_pending_cbs 0x140bebdf0 moves pending Vec+0x40, waits/signals and SubmitInfo state into a submitted aggregate and clears them. Empty CB with signal requested panics; empty nonsignaling submit is discarded. Nonempty path asserts last CB state1, vkEndCommandBuffer via device+0x2c8, writes state2, invokes private raw queue_submit 0x140c04e20 (public ash/Vulkan remains external), stores returned16B completion token recorder+0x108. Debug flag additionally calls wait_idle_and_recycle_resources 0x140bf8980.

begin_frame 0x140beabd0 pops old FrameRecords while VecDeque.len>1 and waits their token via wait_submit_done 0x140bfc780. It resets two timestamp queries, writes opening timestamp, ends initial CB and stages SubmitInfo, allocates/begins active CB. Recorder references executor+0x170, device+0x178, pool+0x180. GraphExecutor::new 0x140bea2d0 creates timestamp query pool count2 and readback buffer, owns deque/profiling state. wait_gpu_queue_trim 0x140bea680 only calls wait_duration_or_until_submit_done on oldest token when duration nonzero and queue.len>1; does not pop frame deque. last_known_frame_duration_ns 0x140beaae0 returns None on absent mapping or scaled delta>=f64::MAX; otherwise Some(saturating u64 of wrapping timestamp delta * f32 device timestampPeriod). Decompiler omits RDX return; original instruction evidence controls.

end_frame 0x140bec400 ensures active CB even with no passes, folds profiling state, atomically max-updates tracked handle CB sequence, writes bottom-of-pipe timestamp and copies query results, submits pending CBs, requires last token, pushes FrameRecord stride0x90 with token+0x80, swaps profiling maps and clears scratch. No wait for current frame.

## Immediate CPU retire and deferred GPU disposal

ExecutedRenderGraph::new 0x140bea180 copies graph0x90+ResourceExports0x48=0xd8. retire 0x140be9d00 has no token or wait. Decompiler misses jump table; assembly controls. It iterates stride0x40 RegistryResource records: tag0 inserts texture via pool 0x140be6d70, tag2 inserts buffer via 0x140be7320; imported1/3 drop managed refs, tag4 decrements Arc, tag5 Empty no-op. It clears graph origins (stride0x48), exports and on-retire callbacks; returns cleared graph+export map. CPU reuse may precede final submit; CB managed trackers preserve GPU safety. R3 owns completion and raw destruction. take_resource 0x140be9c80 removes logical export, unwraps (missing Option tag6 panics), returns stride0x40 resource.

## Temporal

get_and_flip_parity 0x140beced0 operates map state+0x30:64-bit key,u32 value,stride0x10. Missing calls insert(key,0), returns0. Hit writes and returns 1u32.wrapping_sub(old), NEW parity. Normal per-key calls produce0,1,0,1; not global bool.

export_temporal 0x1404f7ca0 copies graph recorder0xa8 and temporal aggregate, iterates map buckets stride0x68; raw texture variant0 calls export helper0x140bd6380, buffer variant1 helper0x140bd5120 with access0x37, then replaces entry with exported variant2 and returned logical payload. Already-exported variants stay. It does not flip parity. Key/payload/schema remain opaque.

## Scope and interfaces

R2 supplies compiled snapshot, batches/barrier plan and into_exports. R4 sends R3 command buffers/SubmitInfo and receives completion tokens. R4 immediately returns cleared graph/resources/exports/temporal state to R2; R3 completion independently controls destruction. Candidate is abstract CPU events, not recovered ABI. Candidate counters/CB IDs are test labels, not actual sequence fields. Full parallel recording, profiling/error branches, temporal reset/ring/payload, raw key wrappers and full GPU equivalence remain unknown.

Candidate follow-up: final active CB remains in the recorder until end_frame; nonfinal batches submit earlier. Temporal resource export now replaces raw texture/buffer variants through an externally supplied export callback, retains already-exported logical entries and never flips parity. The callback deliberately leaves R2 export plumbing and access metadata outside this candidate. Four offline locked unit tests cover these CPU boundaries; they are not original-machine-code differential tests or GPU validation.
