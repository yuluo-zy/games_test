# Frame declaration and temporal owner lifecycle candidate

Scope: B only; T13 capture/state producers excluded. This delivery changes the
current reconstructed App bridge and extracts a borrowed check from the current
rhapsody API. It does not restore a legacy resource backend or claim original
whole-frame, GPU, runtime, or A/C equivalence.

`candidate.patch` is the final v2 candidate. `base.json` records the five exact
base-file hashes and candidate hashes; base commit is
`1253c0ab7cc9706c58c4e57c3a61a0b17c7212f2`. All writes by this agent are under
this evidence directory. No production file was changed and no Cargo command
was run. Root must compare with its newly fetched upstream and integrate.

## Concrete findings and implementation

1. The bridge documented that a built graph must be consumed before another
   frame, but returned a naked `rhapsody::rg::BuiltGraph`. `begin_frame` only
   rejects *prepared* graphs. A second bridge build could therefore abandon
   pending named declarations, overwrite specifications, and supersede the first
   graph's identity. Dropping the naked built graph also dropped retirement
   callbacks without invoking them. Frame-import leases then waited for later
   polling rather than following explicit cancellation.

   The candidate returns `BuiltCoreGraph`, an App-owned RAII guard containing the
   original native `BuiltGraph`, original session, and outstanding declaration
   lease. A second build is rejected before invoking its closure. Drop cancels
   through the originating session, and explicitly cancels frame imports and
   callbacks once. Compile/execute/cancel consume the guard only when its exact
   session and graph identity match. A rejected cross-App guard drops through
   its original session, leaving both sessions reusable. `CompiledCoreGraph`
   remains the existing current prepared-frame owner. The only explicit external
   type annotation found by bounded whole-workspace search was the `pixels`
   helper in `original_raster_gpu.rs`; it is adapted in this patch. The rhapsody
   `BuiltGraph`, session compile/execute APIs, private backend and resource model
   remain intact.

2. `accept_gameplay_warmup` removed its one-shot `CompletedNamedResources` before
   the session checked whether adoption was currently admissible. A temporary
   outstanding declaration/prepared frame or pending submission returned an
   error after destroying the only transfer owner. A later activation silently
   saw `None` and proceeded without the worker's actual previous-frame images
   and parity.

   Existing rhapsody adoption checks are extracted into
   `validate_completed_named_resources_adoption(&self, &CompletedNamedResources)`.
   The old consuming method still exists and calls the same checks. The bridge
   holds its one-shot mutex and one session `RefMut`, checks the borrowed owner,
   then takes/adopts it while that same borrow is held. Failure preserves the
   actual GPU owners and parity for retry; successful repeated activation is
   still idempotent. Device/App identity checks are unchanged.

3. A cancelled prepared graph, failed compilation, or terminal execution left
   `named.graph_id` describing a graph that no longer existed. In particular,
   prepared cancellation kept a non-ended ledger plus a nonempty graph identity,
   so the adoption check still rejected an actually idle session after Drop.
   The candidate clears only that declaration identity at these terminal
   transitions. On compile failure it clears only an identity matching the
   consumed graph. GPU completion/pending ownership, confirmed accesses,
   history validity, parity rollback, resource retention, and partial-submission
   invalidation continue through their existing independent mechanisms.

## Existing frame/GPU lifecycle preserved

| Transition | Current mechanism retained |
|---|---|
| Builder error or unwind | `CoreGraphFrame::drop` cancels the original graph, revokes host scopes and runs retirement callbacks; candidate also releases its declaration lease. |
| Prewarm error / explicit graph cancellation | Native graph is cancelled through its original session; the guard also covers an early `?`, ordinary Drop and cross-App rejection. |
| Compile error | Existing reclaim and callback path; temporal `abandon_frame`; result cache abort; matching declaration identity closes. |
| Prepared graph Drop | Existing `cancel_prepared` reclaims allocated resources and retires callbacks; identity closes. |
| Record failure before submit | Existing failed execution reclaim and `abandon_frame`; no fake receipt. |
| Partial submission | Existing pending failed execution retains real owners and timeline receipts; requested histories are invalidated. No early GPU-safe reclamation is introduced. |
| Successful submission | Existing receipt-based result-cache/history publication, pending-frame tracking and GPU retention. Submission and completion remain distinct. |
| Completion / collect | Existing actual timeline waits, command recycle, managed collect, resource-pool return and callback retirement. |
| OOM / resize reset | Existing temporal clear/retired-token retention; this candidate does not turn reset into a GPU completion proof. |
| Worker-to-main App adoption | Exact owners + completed named resources; borrowed preflight preserves a rejected transfer; successful adoption keeps actual images and parity. |

## Original evidence versus current engineering adaptation

Reused B evidence (not freshly reexecuted):

- `evidence/render-recovery/parallel-batch-03/temporal-full/findings.md`: raw
  texture/buffer versus exported states; duplicate lookup rejection; stored
  access reuse; texture extent replacement with failure preserving the old
  owner; pair parity by key; export does not itself flip parity; compile
  `0x140166510` takes/restores temporal ownership separately from graph
  population failure. That evidence explicitly does **not** establish an
  original temporal transactional rollback on compile failure.
- Current `render_bridge.rs` / `production_renderer.rs` cite B
  `render@0x1404e8010` and worker closure `0x1400d7440` for compile-before-execute
  and completed worker warmup. Those addresses are historical attribution,
  not a new original-machine-code validation performed here.

The RAII guard, borrowed preflight, and terminal declaration identity changes
are fixes to the **current hand-refactored integration**. They are not presented
as recovered native structs/ABI or evidence that the original used the same
transaction and cancellation architecture. Standard Rc/RefCell/Mutex, Vulkan
and generic library internals were reused rather than reimplemented.

## Validation and required central acceptance

Performed: source parsing/formatting with rustfmt edition2024,
`skip_children=true`; final `git apply --check` against the five recorded base
files. Both checks returned0. Production hashes were rechecked by
`finalize_candidate.py` and unchanged. No Cargo/test/GPU execution was run by
this agent.

Candidate tests:

- `core_render_bridge::built_graph_lease_survives_rejected_rebuild_and_releases_on_compiled_drop`:
  the first nonempty side-effect declaration remains intact after rejected
  rebuild; compiled Drop retires exactly once; next declarations succeed.
- `core_render_bridge::foreign_built_graph_retires_the_originating_lease_without_submitting`:
  a foreign callback must never record, original retirement runs, no receiving
  pending frame appears, both original sessions remain reusable.
- `core_render_bridge::dropped_uncompiled_graph_cancels_imports_and_leaves_the_session_reusable`:
  a real BufferHost import lease goes1→0 at built Drop; no submission is made;
  callback retires once and retry succeeds.
- Existing actual TAA test
  `original_scene_compute_gpu::completed_background_warmup_preserves_actual_pair_owner_parity_and_pixels`
  now first rejects adoption during both built and prepared declarations,
  cancels/drops them, then retries. It still checks the exact worker GPU image
  Arc identity as the next frame's previous image, actual parity/history and
  original AA shader readback. This test is prepared, not executed here.

Root should run the focused bridge/compile-before-submit targets and this one
existing TAA GPU test in its centralized window with
`RHAPSODY_REQUIRE_VULKAN=1`. No new full-game or original-runtime completion
claim follows merely from these tests passing.

## Remaining cross-owner issue sent to root

`ProductionRenderHost` methods `clear_resource_pool`, `reset_temporal_tables`,
`wait_idle_and_recycle_resources`, `begin_frame` and `drain_finished_frames`
store errors in `first_error` but have void `RenderHost` signatures. The outer
render routine can continue into main compilation/execution; the later acquire
gate is too late to prevent that work. This is sent to root for narrow error
propagation integration. No production-renderer fragment was applied here.
The root's frame_loop agent currently owns the zero-size frame path, graph_chain
owns the full-world/tail builder, and gpu_chain owns platform surface code.

Full scene passes/producer completeness, WSI error branches, original reset/drop
and entire original compile/record/submit equivalence remain partial. T13 stays
excluded.
