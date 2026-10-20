# Independent screenshot lifecycle review

Reviewer: temporal_lifecycle. Scope: B only, T13 capture/state producers excluded.
Read-only review; no production writes, Cargo, test or GPU execution.

Final verdict: **accept this bounded candidate for central integration**. The
four concrete findings below have been corrected in the frozen source; no
additional blocking source defect was found within this review scope. This is
not a runtime acceptance or whole screenshot feature completion verdict.

Frozen source hashes independently checked against candidate-sha.json:

- screenshot.rs: e1bdd81ee0093af05f93da87dbb291637957f1fc49b2243b3aef2beb149f255f
- screenshot_capture.rs: 54f9b255d7a0551eb79a8b2b163e2b2fb4a5b4090b9daa482fc543d9224f2184
- candidate.patch: db21f918da4cd2e9337e684a187387a9aaa581a3473271ad4096384f40ae1447
- host-migration.patch: aedc0670761bd613bd393023bdc4da7931afa7d450df479ca721f5b21d4c60fb
- CONTRACT.md: 1a04e361cdb6004ff8be98a88ecaf81f07a9c601400dfe28a0992d72675deae5

verification.json records every manifest item checked and the B MCP result
status. The retained full finish/read_back C files match their complete MCP
decompiler output after whitespace normalization. Both functions report
decompiler_completed=true and no decompiler_error. No missing text was supplied
from the author's prose.

## Evidence read independently

- round12/integration/0x14031af10_gather_screenshot_requests.c, full function,
  and this round's gather-map-original.asm.
- round12/integration/0x14031d140_handle_screenshot_opportunity.c, full function.
- screenshot/0x14031ea20.c and finish-original.asm, including the actual dl=1/0
  read_back calls (the void-PDB decompiler loses this argument).
- screenshot/0x14031f2e0.c, complete freshly bounded function, including tag5,
  synchronous branch, normal named thread spawn, retained JoinHandles and error
  branch; readback-bounded job/result for exact PDB range and B identity.
- screenshot/retain-original.asm: state+0x50==2 removal independent of tag+0x96.
- Current rhapsody raw/buffer.rs host_access_with/read_bytes, rg/vulkan.rs
  compile/execute/error/retirement/frame-import paths, and graph/resource.rs
  GraphId/ResourceHandle definitions.
- Current menu_background_host.rs and world_postprocess.rs actual owner/slot
  consumers; graph/candidate/tiny-glade/crates/tiny-glade/src/world_postprocess.rs
  T4/T5/T12 order; screenshot author's candidate source/CONTRACT directly.

No original game execution was performed. This review corroborates retained
static B evidence; it does not claim original runtime or whole-frame equivalence.

## Findings returned for correction

1. **Unsubmitted memory could be admitted by logical-index reuse.** Original
   candidate `submitted(world, receipt)` checked only a u32 logical index.
   A stale cancelled capture and an unrelated successful graph could share that
   index, causing CPU readback from an unwritten buffer or publishing another
   graph's destination. Author changed Capture to full ResourceHandle/GraphId,
   per-graph cancellation and a submitted hook carrying origin bridge + graph
   identity + receipt. The hook now requires nonempty receipts, exported logical
   index, physical mapping/token and actual GpuResource type/Arc pointer equality
   against the independently retained actual owner. This addresses index reuse
   and cross-App confusion provided root supplies the paired graph/receipt from
   the same consumed graph guard. `read_bytes` only waits *registered* GPU uses;
   it cannot prove an allocation was ever written, so the submitted gate is
   necessary even with safe mapped access.

2. **JPEG tail-write errors were ignored.** JpegEncoder owned BufWriter<&File>,
   so Drop could ignore its final flush error and tempfile.persist could then
   yield a false success callback. Author now holds the writer outside encoder
   scope and propagates `writer.flush()?` before persist. PNG/direct-file and
   persist errors continue through the completion Result. No runtime disk-fault
   test was performed.

3. **Hurry shared textures skipped the original wait.** B finish
   14031ecd5..ece7 waits idle for state1 before read_back(dl=1), including tag5.
   Initial candidate waited only in Buffer. Author added the actual texture
   owner's device wait before taking its dimensions callback/marking Done.
   Waiting failure leaves request/callback/actual owner available for retry.

4. **Delaying slot publication changed same-graph business ordering.** Original
   opportunity writes shared slots during declaration; T4 Menu2 is consumed by
   T5 and LUT1 can be consumed by T12 in that same graph. Author restored
   declaration publication; submitted now only validates/adopts readback status.
   Capture independently retains ManagedTexture and records published state.
   Failed published slots remain with their existing shared owner, preserving
   native side effects; unpublished entries are released. The next main graph
   must recapture every pending shared request before consuming those slots.
   Independently read graph candidate T4: unconditional for [0,1,2] followed by
   T5, then T12. This makes retry before consumption possible; an error must
   cancel the whole builder. Changing T4 to only the first awaiting purpose
   invalidates this proof. The actual integrated error/retry graph still needs
   central execution verification.

## State/ownership paths checked

- Native gather-map writes state0/remaining3/tag6. Meta branch uses remaining1.
  Awaiting tag6 does not mean completion. Native finish skips tag6/state2,
  decrements nonzero budget, then reads on the *following* zero-budget finish.
- Buffer and texture opportunities preserve request order and explicit size
  override. Shared destinations use actual storage+sampled texture and blur
  chain; file buffers use GpuToCpu without that blur chain.
- Safe candidate buffer export0x26 replaces native0x37 for current HOST_READ
  visibility. Actual buffer.read_bytes waits overlapping registered GPU writes,
  locks host/submission ranges and invalidates noncoherent memory. These are
  current rhapsody implementation contracts, not original ABI reconstruction.
- Unsubmitted cancellation does not read pixels. Graph/command owners retain
  physical resources independently through partial submission/completion.
  The retirement AtomicBool is only a conservative cancellation cleanup signal;
  it never substitutes for successful receipt admission.
- Normal readback copies bytes before spawning the writer. Spawn failure keeps
  original request/callback for retry. Successful spawn transfers responsibility
  to retained JoinHandle; writer I/O Err reaches callback/log. Hurry delivers
  synchronously after idle. No false Steam upload success is introduced.
- Requests Drop joins already-started writers. It does not replace explicit
  hurry/readback for submitted captures that have not yet produced a writer.
- LUT slot publication uses the exact captured Arc and rejects a detached old
  slot; it cannot fill a new gallery slot after close/reopen. Menu publication
  validates the exact App TextureHost namespace. Independently retained native
  texture owner survives a stale-slot host-entry release until request cleanup.

## Central integration obligations / remaining limits

Root must pair graph_id before compilation with the actual successful receipt,
including warmup, and call submitted before any collect/complete-warmup path can
retire/remove frame-import tokens. All build/compile/record/submit errors,
prepared Drop and partial submission must cancel only that graph's staged
requests. The retirement signal covers callbacks when a builder returns Err
before an outer graph id is returned; it does not replace the success hook.

GameLoop currently marks shutdown_done before its fallible renderer shutdown.
When adding screenshot hurry, a readback error must not skip remaining GPU and
writer cleanup or make future calls falsely report complete. Root was notified;
this reviewer did not change production.

Actual gameplay screenshot producers (file/Steam/Meta/save-thumbnail), native
shutdown timing arithmetic and exact panic/thread-priority policy are explicitly
unrecovered in the author's contract. Awaiting requests at shutdown remain
awaiting rather than receiving fabricated image success. A per-App queue can
only progress while that App is visited; root must finish submitted captures
before its World/device owner is permanently removed. None of these boundaries
is covered by static formatting or a successful compilation.
