# Next batch: actual stair requests and interpolation

No stair request schema or producer source was changed in this batch. Do not
count these analysis artifacts as recovered gameplay. The verified cache and
visual-chain changes can be committed independently.

## Newly retained original evidence

The existing B-seed Ghidra program was opened read-only using the already
installed PyGhidra and explicit existing JDK. No re-import or whole-program
analysis was run. Each exported function's entire PDB-sized byte range was
compared with the original F507 executable before decompilation. The seed's
program name is not used as proof of identity. Hashes and the full original PDB
names are in `decompile-manifest.json`; `.bin` files are those exact original
ranges. The program was released and the project closed without a transaction.

| Original address | PDB bytes | New raw decompilation | Meaning and limit |
| --- | ---: | --- | --- |
| 140276e80 | 30178 | 140276e80.c | generate_playermade_stair_requests false; 161086 bytes C, type-propagation warning |
| 14027e470 | 30226 | 14027e470.c | generate_playermade_stair_requests true; 161339 bytes C, type-propagation warning |
| 1407cf500 | 8622 | 1407cf500.c | interpolate_and_compare; 42804 bytes C, currently a false-return stub in Rust |
| 140807090 | 10808 | 140807090.c | compute_stair_assembly_params; 41969 bytes C, currently an empty-output stub |
| 14080c640 | 455 | 14080c640.c | derive_stair_node_params; the decompiler reports completed but **does not resolve its two indirect jump-table branches**. Current Rust returns only the dispatch index, not real node parameters. |
| 1407d8fe0 | 598 | 1407d8fe0.c | calculate_adjusted_ladder_positions; 2646 bytes C. Its 1407d9240 closure remains a necessary dependency. |

`*-named.c` are readability sidecars. Their aliases are a lexical substitution
from the existing original PDB procedure ledger; `pdb-call-aliases.json` retains
the exact long names. Raw decompilations and their hashes remain unchanged.
The Ghidra datatype inventory contains no matching named request/adjusted-edge
structures; `original-types.json` is empty, not evidence for an empty schema.

The old source comments about decompilation timeouts are historical. These six
exports now exist. A completed decompiler operation does not prove every branch
or inferred variable type is correct; especially preserve the derive jump-table
gap above.

## Required canonical owner recovery

The cache's original nine SystemParams borrow the requests resource and a real
StairsState. The current canonical `PlayermadeStairsAssemblyParams` contains only
positions, heights, widths, tangents and segment. Original cache consumption also
requires the generated spatial curve and segment classification metadata:

- request +0x98: optional Curve3 state; +0xA0/+0xA8: generated Vec3 samples;
- +0xB8/+0xC0: accumulated curve coordinate array; +0xC8: length-related scalar;
- +0x130..0x148: SmallVec segment descriptors, containing type byte and u16 count;
- actual AdjustedStairEdge source and directed outgoing orientation, rather than
  choosing an arbitrary neighbor of the canonical public undirected graph.

These are consumer-established offsets, not a fully verified source schema.
Trace the producer's writes, clone/previous-request ownership and interpolation
before naming fields or publishing a new type. Preserve all anchors, alignment
choices, segment kinds, quantization, random inputs and dirty side effects.

The dst4 cache searches exact endpoint pairs. No-match original
1408169d0..140816a06 writes identity rotation, position (-0,-100,-0), scale ZERO,
then still performs previous-transform comparison and dirty marking. Restoring
only this failure result would not close the positive stair path.

For a matched request, original cache consumes Curve::get_coord_at_u, segment
counts/types, interpolated tangents/widths, step height quantization and full
adjusted ladder positions. The original mode2 quantization and mode3 ladder
branches must not become a single linear interpolation.

## Ownership coordination at handoff

The visual agent owns the request schema, request/compute producers in stairs.rs,
and interpolation/ladder portions of stairs_assemble.rs. The save-chain agent owns
stairs_collider.rs, structural_graph.rs, invalidate.rs, and the three structural
collider/graph functions in stairs_assemble.rs; do not format or replace the whole
shared file. The save agent has since prioritized validate_blueprints and actual
frame-end tracking/host integration for this commit. Resume new stair production
work only in the next batch, after central coordination.
