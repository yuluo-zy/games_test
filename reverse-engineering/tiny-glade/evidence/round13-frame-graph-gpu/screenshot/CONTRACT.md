# B screenshot candidate, round13

Frozen candidate only: `screenshot.rs`, `screenshot_capture.rs`, `candidate.patch`.
Base digests are in `base-sha.json`; candidate digests in `candidate-sha.json`.
No production source changed by this worker. No Cargo invocation or GPU test.
`rustfmt --edition 2024` parses both candidate files successfully.

## Native observations

Identity is B patched EXE SHA256
`cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`.
Original instruction oracle is B RNE SHA256
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
No A/C inference and no T13 feedback capture/state producer.

* gather `14031af10` exact PDB length 8740: existing full c/asm in
  `../../round12/integration/0x14031af10_gather_screenshot_requests.c` and
  `gather-capture-original.asm`. It creates requests, never capture textures.
* map closure `14048db70`, PDB length 465: `gather-map-original.asm`,
  `14048dba4..dbe8` stores `0x00000003_00000000` at +0x50 and tag6 at +0x96:
  state0, remaining3, AwaitingOpportunity. Existing source discarded an eagerly
  allocated handle and initialized the wrong state semantics.
* MetaCommander special `14031c38d..c443` constructs purpose4, state0,
  remaining1, tag6; path/callback/visibility and command consumption belong to
  its producer, not the graph opportunity.
* opportunity `14031d140`, PDB length 6359: existing full c/asm in
  round12/integration. Iterates ALL matching awaiting requests in list order.
  Purpose test normalizes bytes2..6 and does exact-byte comparison for the
  fallback category; exact byte equality is equivalent across all 256 inputs.
  Explicit dimensions override source extent.
* Shared destination sentinel `request+0x18 == 0x8000000000000001` allocates
  RGBA8 texture, storage+sampled; selects blur stages by round(log2(source_h /
  output_h)) saturating to u64. Writes actual destination texture, exports0x22,
  replaces the Arc slot with a managed clone, and saves payload tag5.
* File destination creates mapped GpuToCpu buffer with usage0x20 and native
  size `(u32 width * u32 height) as u64 * 4`. Import initial access0;
  capture shader variants sample1/4, source binding hashes d15be69af90f5947 /
  681ca9c93557dc9c, buffer0f6804da1cecc30e, dst_dims ca502818f423c41d.
  No texture-branch blur chain for buffer captures. Native exports access0x37.
* finish `14031ea20`, PDB length1193: `finish-original.asm` and fresh
  `0x14031ea20.c`. Skip tag6 and state2; state1 calls wait_idle and read_back
  with true; otherwise zero remaining calls read_back with false, nonzero
  decrements. Only after read_back is state2 written.
* read_back `14031f2e0`, PDB length5047: fresh `0x14031f2e0.c` plus
  `readback-bounded.result.json` and complete stdout artifacts. Takes payload by
  replacing tag with6. Tag5 only invokes optional dimensions callback, with no
  pixel readback. Buffer branch with hurry=true writes synchronously; normal
  branch clones sink/bytes/callback, launches a named writer, reaps finished
  JoinHandles and retains outstanding ones. This worker did not claim to
  reconstruct profiling or Rust/Vec/Arc/thread runtime glue.
* retain `1402b5c00`, PDB length664: `retain-original.asm` confirms retain only
  state!=2. It is not `tag==6 && state==2`.
* write_image_with_callback_impl `1403206a0`: existing source implementation is
  preserved: bounds check, parent creation, tempfile in destination parent,
  jpg RGBA→RGB quality, other format dispatch, persist, Result dimensions callback.
  Review found a real I/O bug in the existing JPEG path: BufWriter Drop ignored
  flush errors before persist. Candidate now explicitly flushes after encoding
  before persist, so a flush failure cannot report Ok.

## Candidate ownership and API adaptation

One `Requests` ECS resource per App replaces the native global queue and owns
normal file/callback requests plus existing menu/LUT shared destinations. The
file/Steam/Meta gameplay producers remain to be recovered; generic public enqueue
does not count as their restoration. The existing Menu/LUT gather is preserved.

An actual ManagedBuffer allocation is imported into the existing CoreGraphFrame
and bound through typed `write_rg_storage_buffer`. It is NOT a second renderer or
opaque fixture handle. Buffer export uses current host-read access0x26, rather
than native0x37, to establish HOST_READ visibility required by the current safe
read_bytes API. That API waits on actual overlapping GPU writes and invalidates
noncoherent memory. Byte-size overflow is rejected (current safe Rust admission)
rather than reproducing native u32 multiplication wrap.

Captures have explicit unsubmitted/submitted ownership in the current split
build/compile/execute API. **Texture publication remains at graph declaration**:
T4 purpose2 is read by T5 Menu bg in the same graph; T4 purpose1 may be read by T12
LUT gallery in that graph, particularly after a cancelled earlier attempt.
Deferring publication until submit would lose those dependencies. Each request
retains its own actual ManagedTexture, independent of the slot's host owner and
any stale gallery-slot release. Published slots keep their original lifetime.

Capture stores a full ResourceHandle with GraphId. The submitted hook is passed
that exact BuiltCoreGraph graph_id alongside its successful receipt and the
origin CoreRenderBridge. It checks nonempty submission receipts, exported logical
index, physical mapping/token, actual GpuResource Buffer/Image type, and Arc
identity against the retained actual owner. Only then can CPU readback advance;
the hook does not republish textures. Host must pair graph_id/receipt from the
same consumed BuiltCoreGraph/CompiledCoreGraph guard.

`cancel_unsubmitted(world,graph_id)` resets only that graph's staged declarations,
retaining older submitted captures. Unpublished host texture entries are released;
already published Menu/LUT entries remain with their original shared owner, and
retry at T4 replaces them before T5/T12 consumers in the next graph. This preserves
native declaration side effects rather than inventing a rollback of already
consumed slots. This requires the current compile_main T4 invariant: unconditionally process
ALL purposes [0,1,2] before either T5 Menu bg or T12 LUT gallery, regardless of
bvar43's first-awaiting purpose. Any error before completing that sequence must
cancel the builder; no graph containing a consumer of the failed slot may execute
without that recapture. Current graph_chain candidate lines1136..1141 satisfies
this order. If future code gates T4 by only the first queue purpose, this proof
expires and slot validity must be checked separately. The failed shared-slot
branch still needs real integration validation. No failed capture is admitted
to CPU readback. Graph/command
owners defer physical retirement, including partially submitted failures.

Each capture also registers an on_retire Arc<AtomicBool> signal. If not submitted
and its graph retired/cancelled, the request is treated as Awaiting and resets at
the next opportunity or finish. This covers builder `?` failures without an outer
returned GraphId, built/compiled guard Drop, compilation errors, recording errors
and deferred partial-submit cancellation. The signal is only cleanup evidence,
never evidence of submission or permission to read back. A successfully submitted
request ignores that signal. It relies on the temporal bridge's guarantee that
all cancelled graph paths eventually run retirement callbacks. Readback failure preserves
buffer/request/callback and zero remaining budget for retry. Thread spawn failure
preserves callback and GPU source for retry. Completed writer I/O errors invoke
the completion callback with Err and warn, and never pretend a file was written.
Shutdown marks hurry, waits the actual device for BOTH texture and buffer
captures before callbacks/readback (native state1 contract), synchronously
finishes submitted captures, always joins old
writers even if readback fails; remaining awaiting captures are preserved.
Requests Drop also joins writers before per-App teardown and releases unpublished
textures; submitted menu/LUT allocations belong to their existing shared owners.

## Required host integration, central owner only

0. Independent loading Worlds intentionally have no screenshot resource. Gate
   optional host finish/submitted/shutdown work with
   `shared_texture_capture::installed(world)`. Enqueue/opportunity still require
   the installed gameplay owner and return a real missing-owner error.
1. compile_main uses `screenshot::active_capture_purpose(world)`; first awaiting
   entry or7. Existing `shared_texture_capture::opportunity(frame,world,purpose,
   source,samples)` signature remains. T9/T19/T22 gating belongs to graph_chain.
2. Immediately after successful `bridge.execute_compiled` in execute_main_graph,
   BEFORE acquisition/export validation/presentation, call
   `shared_texture_capture::submitted(self.world, &mut self.renderer.bridge, graph_id, &receipt)` and report a real
   error on failure. A main graph may have succeeded even if presentation fails.
3. On build/compile failure, cancelled prepared graph, or failed main execution,
   call `cancel_unsubmitted(self.world,graph_id)` after the bridge's own graph cleanup.
   Do not cancel older submitted captures because acquisition/presentation fails.
4. RenderHost::finish_screenshot_capture calls
   `screenshot::finish_screenshot_capture(self.world)` and reports real errors;
   remove the old unsupported association guard only after these hooks exist.
5. Before the application's final device/World teardown call
   `screenshot::hurry_active_screenshots_on_shutdown(world)`. Resource Drop joins
   writers as an ownership backstop, but is not a substitute for GPU readback.
6. The old `has_active_screenshot_requests()` signature now requires `&World`.
   All old allocate/readback fake ScreenshotBackend APIs have no production
   callers (bounded source search); this candidate replaces them, not shims them.

## Remaining gaps and validation boundaries

This is candidate_partial, not whole screenshot equivalence. It recovers the
actual GPU allocation/graph/readback/writer closure and keeps known lifecycle
branches, but lacks actual gameplay save-thumbnail/user-input/Steam callback /
MetaCommander production request sources. Steam SDK upload is absent from the
current core interface; no empty upload-success fallback exists in the candidate.
Native old shutdown timing arithmetic and exact API-error/panic/thread-priority
policies were not recovered. Original JPEG/PNG file I/O and buffer sampling have
not been executed by this worker. Warmup uses the same split hook policy and must
also confirm/cancel captured requests when it executes/cancels a graph.

The central owner must compile, review current renderer shader availability and
run a real capture across successful submit, graph cancellation, GPU completion,
file write and shutdown. No other module or T13 work is counted as complete.


## Narrow migration package

`host-migration.patch` contains exact source-context hunks for all concrete
callers requiring the new owner/receipt API:

* production_renderer: CompiledGameGraph carries BuiltCoreGraph.graph_id across
  all three constructors (full/minimal/loading), execute success confirms that
  exact receipt before taking the main export, execute failure cancels that key,
  finish uses the installed World owner instead of the unsupported fake guard.
* render_bridge::warm_up_gameplay_with_world_builder: preserves built graph_id,
  cancels it on execute Err, confirms screenshot receipt on success before
  complete_gameplay_warmup retires the graph's import tokens.
* tests/original_capture_gpu: preserves graph_id and confirms actual successful
  receipt before waiting/reading its mapped verification buffer. Samegraph slot
  assertions continue before submit and verify native publication order.
* game_loop::shut_down: illustrates installed-owner hurry before renderer teardown
  while evaluating both cleanup results. **frame_loop owns final shutdown retry,
  loader and save lifecycle integration**; merge this single call into its newer
  candidate rather than overwriting the function. Its shutdown_done correction
  is required independently of this screenshot patch.

Full `host-candidate/` files are review context only; use the narrow patch, never
copy these snapshots over another agent's later edits. `host-base-sha.json`
records their exact production inputs. They parse with rustfmt skip_children.
Source/test-wide API search found no external users of the removed fake
ScreenshotBackend/ActiveScreenshotRequest/ScreenshotOpportunity helpers or byte
conversion functions. Existing installed/opportunity callers keep their API;
only the old production finish guard needed migration. No compatibility stubs.
