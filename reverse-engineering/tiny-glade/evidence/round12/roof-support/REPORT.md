# B roof support-beam producer investigation

No game algorithm, shader, library or production schedule was changed. No Cargo
or GPU run was performed. `source-correction.patch` is a documentation candidate
for the central integration owner; it removes an incorrect wall-lintel attribution.

## Observed compiled evidence

Original RNE SHA256 is
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
The independently scanned original PDB SHA256 is
`2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`.
All scans read original files, without a database mutation or analysis overlay.

1. `pdb-support.json` exhaustively covers 4,894 DBI modules, 6,980,730 module
   symbol records and 132,599 IPI records. The exact roof-support source has
   one retained method identity: IPI `0xd9e3`,
   `system_roof::visual::place_support_beams::place_roof_support_beams::closure$1`.
   C13 declares it at source line95 in modules906 and2245. The source SHA256 is
   `e8bca1414a059659d536760683f7c43abfa618bb16f5e4fa77304607834f5256`.
   There is **no matching PROC, INLINESITE host or physical C13 source-line
   range**. The two LLVM dumps independently confirm the declarations and
   actual emitted routines in those modules. A retained declaration is not a
   recovered producer body or an inline host.
2. `public-symbols.json` independently scans all188,352 symbol-stream records.
   Six matching public symbols describe the draw callback, marker registration
   and drops. No `place_roof_support_beams` public alias is present. Thus the
   missing module body is not explained by a visible public ICF alias.
3. `native-references.json` scans every section of the original RNE for the
   exact SSBO identity `0x83773598f5ecfead`. Its only three occurrences are:
   `1412c272c` in `add_new<RoofSupportBeamSsbo>`, `1412cb52b` in
   `get_rg_buffer<RoofSupportBeamSsbo>`, and descriptor `142c2e320` in rdata.
   Common RIP-relative references to that descriptor and the exact type-name
   string `142a9da0e` resolve only to constructor, get-buffer and draw routines.
   The InstanceBuffer TypeId halves occur only in its `type_id@14160efd0`.
   These scans are bounded raw-reference candidates, not a complete dynamic
   alias analysis.
4. Original `add_new@1412c2650` calls `TypedStorageBuffer::new@14161a590` with
   `r8=0` (None), writes count0 at `1412c2678`, and inserts the allocation.
   The typed constructor allocates `0xa00000` bytes, usage `0x22`, memory1,
   alignment4: capacity131,072 records of80B. It is a **dense InstanceBuffer**,
   not a SparseInstanceBuffer. There are no per-entity chunks to allocate/free
   in this retained path. The initial count write and actual construction
   parameters match current `roof_buffers.rs` and the existing bounded
   constructor test; this investigation did not run that test.
5. Original `draw_instanced_roof_support_beam_shader@1409d98a0`,788B, gets the
   library object by the descriptor, asserts the exact type name, calls
   `as_any` and reads the count at object+0 (`1409d99b7`). It binds terrain
   height/lighting and the roof-beam buffer, then draws that count. This is
   distinct from both Nani roof-floor ownership and WoodenPlank64B producers.
   The public drop alias `140876260` is only9B and aliases the ordinary managed
   GPU-buffer handle release; it is not evidence for a per-roof beam allocator.
6. The retained roof plugin evidence reviewed by the roof owner registers
   tile assembly, stone floor and spires as its complete three-item visual
   tuple in set29 (`140ddffb0`). It has no fourth support-beam system. This
   corroborates, but does not alone prove, absence of an active producer.

## Conclusion and implementation decision

There is strong static evidence that B retains an **unpopulated roof-beam render
channel** while omitting its nonempty producer from the compiled roof plugin.
There is no evidence authorizing a newly invented beam placement algorithm,
random policy, material policy, dynamic-length owner or delete lifecycle.
Specifically, `system_wall_constructor::construct_lintels::place_support_beams`
writes WoodenPlank64B and must not be used as a substitute. The existing
`visual/mod.rs` claim that the roof file is covered by that inline implementation
is incorrect; the candidate patch corrects only that claim.

This is **not** a nonempty beam producer/GPU acceptance and does not prove global
runtime unreachability. Keep the retained shader/SSBO/draw contract intact. Do
not introduce an empty system and label the producer restored. The central owner
should distinguish this bounded B-inactive candidate from actual missing roof
trigger/tile/floor/spire behavior, so that speculative work does not delay those
proven active chains.

## Exact remaining uncertainty / next query

The unresolved question is whether an anonymous or externally configured runtime
path can mutate the dense library object without any of the visible identity
references. The next decisive check is a runtime watchpoint on the actual object
count word after `1412c2678`/library insertion, following real rectangular and
circular pitched-roof creation, edit, switch and deletion. Record each nonzero
write's instruction and stack; that writer becomes the producer seed. A static
alternative is complete instruction-aware dataflow from descriptor `142c2e320`
and the unique global SSBO object through every generic library retrieval, with
all indirect targets resolved. The current common-RIP scan alone is not that
proof. Do not cross-apply A/C source or addresses to fill this gap.

If a real writer is found, extend the existing production roof test through
that writer, require nonzero80B records, physical GPU readback and the actual
roof-beam draw count, then follow its actual count/owner update on deletion.
Do not fabricate a beam to satisfy a nonempty assertion, and do not require
Sparse or Nani lifetime when the original dense owner does not use them.

T13 remains excluded. Tool-script execution and these static inventory counts
are evidence work, not gameplay implementation completion.
