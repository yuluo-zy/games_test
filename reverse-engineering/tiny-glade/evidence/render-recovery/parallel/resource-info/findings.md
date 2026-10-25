# ResourceInfo type family

Target: original Tiny Glade `tiny-glade.rne`, SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. Static evidence only. PDB `void()` signatures are placeholders. This package reconstructs a semantic CPU state transformation, not the author's text or a full ABI.

## Observed layout

| ResourceInfo offset | Storage | Element |
|---|---|---|
| 0x00 | vector | 20-byte access chain record |
| 0x18 | vector | 8-byte half-open access-index range per pass |
| 0x30 | vector | 24-byte lifetime record per resource |
| 0x48 | vector | u32 image usage flags per resource |
| 0x60 | vector | u32 buffer usage flags per resource |
| 0x78 | vector | u32 sorted lifetime event words |
| 0x90 | hash container | exported resource indices, narrowed to u16 |

Vectors have capacity/pointer/length at their relative offsets 0/8/16 in this build; that is an observed build layout, not Rust's stable ABI. `clear` (0x140bd4410) resets all six lengths and clears the hash container while retaining allocations. Full object size, source field names and hasher are unknown.

Calculate (0x140530430) takes ResourceInfo mutably, a resource-declaration container, a pass slice pointer and pass count. The declaration container exposes 72-byte origins at pointer +8/count +0x10 and 48-byte exports at pointer +0x20/count +0x28. Passes are 64 bytes; their access vector pointer/count are +8/+0x10. Pass access declarations are 12 bytes: resource u32 at +0, synchronization group u32 at +4, dependency access-code u8 at +8, and three unobserved padding bytes.

Emitted accesses are 20 bytes: resource u32 +0, previous u32 +4, next u32 +8, synchronization group u32 +0xc, dependency access-code u8 +0x10. Next starts at 0xffffffff; previous is the resource's prior last access. Source declaration padding is not copied. Lifetime records are 24 bytes: first endpoint tag/value +0/+4, last endpoint tag/value +8/+0xc, first access +0x10, last access +0x14. Both access indices start at 0xffffffff.

PassIndex::from(usize), 0x140bd4b40, and ResourceAccessIndex::from(usize), 0x140bd4b80, return the low u32 when upper 32 bits are zero; they panic otherwise. They permit 0xffffffff: the sentinel is a consumer constraint, not excluded by these conversions.

## Observed state flow

1. Clear reusable derived state. Obtain the resource count; reserve and initialize one lifetime record per origin. Resize image and buffer usage arrays to that count with zeros.
2. Initializer closure inside standard Map::fold, 0x14048da00: origin tag 0 starts with absent first/last endpoints. Tags 1/2 start first=Some(0), last=Some(pass_count-1) when count is nonzero, otherwise absent. Other tags panic. None payload words are not semantic values and the candidate canonicalizes them.
3. Extract creation flags (0x140bd44b0): only tag 0. Byte +0x36==7 selects buffer flags at origin +0x20; byte==8 does nothing in this extractor; remaining bytes select image flags at +0x18. Valid descriptor discriminator range is not established by these branches alone.
4. Extend export usage (0x140bd4560), before processing passes: export tag byte zero selects resource index +4, otherwise +0x18; access code at +0x28. Ask the public dependency for access info, require resource index <65536, and insert u16 index into the hash container even for a zero access mask. For zero mask stop processing that export without checking usage-array bounds. For nonzero mask, export tag bit0 selects buffer versus image conversion, and converted flags are ORed into that resource's usage.
5. Iterate passes and declarations in input order. Bounds-check lifetime index. Convert current global access length to ResourceAccessIndex; append access with previous=old last, next=NO_ACCESS, then update lifetime: first endpoint is set if tag low bit is clear; last is max(old,current pass) only if old tag==1, otherwise set; first access is filled if sentinel; last access is always updated.
6. Ask vk_sync_fork::get_access_info (0x140c10dd0) for access mask. Bounds-check origin index and call ResourceOrigin::ty (0x140bd6d10): tag0 descriptor byte7 gives type1, byte8 panics, remaining bytes give type0; tags1/2 return byte +8. Type1 converts buffer mask; type2 skips usage conversion; remaining values convert image mask. Zero mask here still calls image conversion and can panic. Earlier append/lifetime writes precede these panics.
7. Append half-open global access range for every pass, including empty passes. Link each prior access to its successor with existing verified leaf 0x140bd47d0. Generate/sort lifetime events with existing verified leaf 0x140bd4860.

The source of access masks, write classification and their enum is the original vk-sync-fork dependency; reuse it instead of reconstructing it. Existing image/buffer conversion and chain/event implementations are reused via the renderer crate.

## Events and consumers

Observed: only resources whose endpoint tags are both exactly 1 generate events. First-pass word is (pass<<17)|resource and has no added bit16; last-pass word adds 0x10000. Sorting unsigned words schedules a resource's start before its end at the same pass. There is no resource mask in this leaf. Inferred: bit16=0 is allocation/start and bit16=1 is lifetime end; the registry populates only bit16-clear events (registry worker's independent consumer evidence). Export index <65536 is established; the full resource-count and pass-count packing-domain constraint is not enforced by the event leaf itself.

## Ownership and unknowns

Observed: this routine reads origin and export declarations but does not rewrite them or touch GPU handles. It owns/reuses derived vectors and the export-index set. Inferred: borrowed declarations feed a reusable ResourceInfo analysis snapshot; allocation/handle ownership transfer occurs downstream in registry population, outside this package.

Registry worker cross-check: tag0 is descriptor/new-allocation; tag1 is Imported; tag2 is an owned-resource origin (ownership wording inferred). This is independent consumer evidence; the candidate keeps neutral Tag0/Tag1/Tag2 naming and records numeric type bytes 0/1/2.

Unknown: full ResourceInfo source type/field names and total object layout; actual names/ABI of origin/export enums and payloads; dependency fork source/patches and code enum; source padding; hasher/order of exported set; raw malformed-state padding/None payload contents; GPU handle lifecycle and compiled-graph integration. Candidate Rust models valid typed declarations and does not claim byte-level original object ABI or unwind equivalence.

Evidence anchors: original saved decompilation `evidence/render-architecture/0x140530430.c`; workgraph `calls.json` and `functions.json`; local `calculate.asm`, `clear-and-export.asm`, `lifetime-initialization.asm`, `indices-and-kinds.asm`, `resource-origin-kind.asm`; prior renderer `verification.json`.
