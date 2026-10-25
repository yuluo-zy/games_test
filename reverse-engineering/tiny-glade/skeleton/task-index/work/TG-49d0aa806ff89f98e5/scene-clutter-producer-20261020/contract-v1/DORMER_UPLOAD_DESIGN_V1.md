# Dormer48 list -> spatial table -> existing SSBO owners

This contract restores one actual middle consumer before its missing producers. It targets current Rust architecture using original B instructions. Consumer source checksum is indexed B=C, but this does not establish C ABI/dependency equivalence; core Dormers B/C checksums differ and C behavior is unverified.

## Owners and bounded write authority

Root normally claimed both source families with live owner `scene-clutter-producer-20261020`: TG-49d0aa806ff89f98e5 (`systems/decorator/src/dormer_holes.rs`) and TG-57a33750ac4a866a76 (`country-core/src/resources/walls/dormers.rs`). The current live records, source paths and hashes are retained in `current-source-bindings-v1.json`. Existing old core-producer work is read-only history.

AssetSsboLibrary TG-3f1019bf28c1d470d9 remains owned by render-ssbo-lease-20261018. TypedStorageBuffer TG-c36fbd00afb24cafe0 remains render-prelude-ssbo-20261018. GraphicsDevice/render owner TG-43850f1acd4119553a and Rectangle2d TG-6e76d82e8a953a9a45 are also foreign. This design needs no source change to any of those owners, no second BufferHost, no parallel SSBO registry, no Cargo change and no altered original record type identity.

The actual whole-blueprint owner is the existing canonical `DecoratorBlueprints`. Its `dormer_holes: Vec<country_core::resources::walls::dormers::DormerHoleSsbo>` is already integrated. The old `system_decorator::utils::DormerHoleSsbo` byte struct is a different TypeId; the new consumer must not use it or its local SsboType implementation. Existing startup registers the canonical48B hole buffer and canonical4B count buffer into the same AssetSsboLibrary. Roof raster bindings already alias those same actual types.

## Proven native input and geometry

Complete consumer `0x1407b1d40/14341`, PDB426:3168, reads every value of the current blueprint map on every invocation. At `0x1407b2e3a..0x1407b2e78`, actual Arc allocation+0xf8 is the48-byte list pointer and +0x100 its element count. Loop increment is exactly0x30. It does not read `Blueprint.colliders`, compare previous blueprints, gate on Arc identity/change ticks, or create dormer mesh entities.

For each full12-word record, it constructs the existing Rectangle2d value directly with axis=(f32(words8),f32(words9)), center=(f32(words0),f32(words2)), size=(f32(words4),f32(words6)); `0x1407b2ecb..0x1407b2f02` calls actual Rectangle2d::aabb. Preserve raw float inputs; do not call a constructor which adds a finite/unit-axis assertion or normalize the axis. The geometry owner/algorithm is the current utils Rectangle2d::aabb, independently bound to its complete87B native body. Hole records remain unchanged while being copied into cells.

Grid projection is `(coordinate + 65.0f32) / f32::from_bits(0x410aaaab)`, then floor, then Rust's saturating f32-to-i32 cast. The native explicitly saturates positive overflow to i32::MAX and NaN to0; negative overflow maps i32::MIN. Current old code's large-positive ->0 is wrong. For each dimension only lower/min bound is max(value,0), and only upper/max bound is min(value,14). Inclusive nested row/column ranges can therefore be empty for a box wholly outside the grid. Cell index is row*15+column. There are15x15=225 cells, not15x60.

## Exact fixed storage and overflow

Native first calls dummy@0x141286660, allocates a50-row Vec with all50 values initialized to that full48-byte zero value, then clones it225 times. It separately allocates900 **bytes**, zeroed as225 u32 counts (`0x1407b29ef..0x1407b2a7d`). Each cell always retains50 records; it is not a variable-length push list. Flatten order is cell-major with exactly50 rows per cell, including unused dummy rows.

At `0x1407b31b9..0x1407b31f9`, bounds checks remain real panics. For an in-range cell: next=count+1; only next<=49 is stored and used to overwrite row[next-1] with the original full48-byte record. At next>=50 the counter is not advanced, no row is written, and a level2 warning is conditional on the actual log filter; execution continues to the next cell. The format is `Exceeded maximum count {50} of dormer holes per spatial accel grid cell at cell id {cell}`. The advertised50 slots and accepted49 rows are both native facts. Do not change this to50 accepted rows, unconditional panic, false success count, or a silent skip.

Future candidate should retain the original per-invocation fixed225x50 CPU state and flatten only on the actual registered hole-buffer path, preserving the observed allocation/lifetime scope. No unrequested performance cache or incremental dirty registry is introduced.

## Same library, write order and failures

Original lookup order is count buffer first, holes second. Each present entry asserts its complete canonical type name and downcasts to the actual InstanceBuffer. Missing entry/downcast-None skips only that buffer's upload; this is observed native optional behavior, not permission to fabricate a successful registration. The real App startup must still provide both existing owners.

Count: lookup@0x1407b364f -> full-name assertion -> downcast -> check900-byte capacity -> one GPU write of all225 u32 values, byte range0..900/offset0 -> unwrap/panic on error -> only then set instance_count=225 (`0x1407b3748`). If this write fails, hole lookup/write and later copy do not run.

Holes: lookup@0x1407b3760 -> independent full-name assertion -> downcast -> flatten the actual fixed table -> capacity check -> one GPU write of11250 full canonical records,540000 bytes/offset0 -> unwrap/panic on error -> only then set instance_count=11250 (`0x1407b38ce`). A hole failure retains the preceding successful count write/count metadata. No rollback or synthetic atomic transaction is claimed.

Current TypedStorageBuffer::gpu_write takes `&self`, validates capacity and returns BufferWriteError. Metadata publication still requires `&mut InstanceBuffer`, so the safe current host should use the existing `AssetSsboLibrary::get_mut<T>` with **ResMut<AssetSsboLibrary>**. This is an explicit host mutability adaptation from original Res<AssetSsboLibrary>, preserving the same owner. Do not cast an immutable reference, use unsafe interior mutation or edit the foreign library. GraphicsDeviceResource supplies this App's same actual device/BufferHost. Preserve panic on each returned upload error; current BufferWriteError erases backend detail, so exact native error payload/source-chain equivalence is not claimed.

Proposed public system interface is `(Res<GraphicsDeviceResource>, ResMut<AssetSsboLibrary>, Res<DecoratorBlueprints>, Res<UiSignifierStream>)`. The fourth dependency remains the actual canonical scheduling resource. Original body does not retain/use incoming r9 before its first clobbering call, and has no signifier output call; no fake signifier is generated. CPU grid helper should return canonical count rows and fixed full record table rather than the old fake900-element counts/variable rows/count sum.

## Canonical record constructors

Current canonical tuple `[u32;12]` and both existing StorageElement/SsboType owners remain. Restore only the exact three methods here: dummy -> all12 zero words (complete18B); gothic -> first Vec3 bits to words0..2, actual incoming u32 to word3, second Vec3 bits to words4..6, word7=0, four actual tail f32 bits to words8..11 (complete85B); cottage -> same first8 words, two actual tail f32 bits to words8..9, fixed native words10/11=0x3f800000/0xc2480000, i.e. f32(1,-50) (complete76B). A movsd loading that constant is a64-bit copy; the old chapter calling it a caller-supplied f64 is unsupported.

Keep the scalar name neutral (`native_u32_0c`) until actual producers prove its nominal ID. Proposed value constructors accept u32, Vec3, Vec3, and `[f32;4]`/`[f32;2]` tail; host value arguments are not a claim to match original by-reference ABI. No default selector, dimensions or transform is invented. Consumer's projection uses the actual words and existing Rectangle2d, not guessed header byte arrays.

## Removal, editing and scheduling

The consumer has no per-hole authoritative registry or deletion event. Every run starts fresh zero counts/full dummy table and copies only currently authoritative blueprint records. Removing or editing a record therefore overwrites old membership/unused rows on both real buffers; even an empty current map uploads zero225 counts plus the full11250 dummy rows. Same-Arc blueprints are still read. Original current -> fullvisual -> middle -> copy phase and fail-stop conditions remain mandatory; upload is middle member3, not the previously mislabeled cloth upload. See REGISTRATION_CONTRACT_V1.md.

This does **not** close the nested dormer-roof geometry member. That distinct first sibling has a real generate_dormer_roofs14207B plus a deletion system using actual DormerTiles and DeleteRoofCmd. Current DormerTiles is `_marker`, so mounting the old geometry function is not accepted. Its current hand-written maintain state/cursor schema cannot substitute for canonical resource ownership.

## Current source differences and next producers

Current dormer-hole source reads the first collider footprint; emits a different TypeId; writes only16 of48 bytes; has900 counts; allows50 records; creates variable cell lengths; drops count upload entirely; ignores hole GPU errors; publishes a count sum rather than11250; and has a hand-call-only3-parameter entry. These are concrete differences, not permitted shortcuts. Existing own tests mirror those wrong inputs and must be replaced with real canonical-list tests.

Current global source has no `.dormer_holes` writer outside the canonical field/equality. Arrow-slit and trap-door code calculate an old `_hole` then discard it; other current generator bodies also do not publish the list. Corresponding original core-constructor caller tasks are arrow_slit TG-475f26c75112d21731; cottage_dormer TG-4a10377b42911028e3 (live owner clutter-startup-20261020); cottage_wall_window TG-52f7fc4f94ad125530; gothic_dormer TG-6a3ef5303dfe62a5a7; gothic_wall_window TG-73d8f60997815a3b36; trap_doors TG-0e52e2ec64528325e0. Do not write those files without central ownership coordination. Restore their real filtering/geometry and insertion after this consumer contract; no synthetic test input counts as their runtime producer closure.

## Required validation after review and implementation

1. Canonical constructor bit-exact tests with all twelve words, signed zero/NaN payloads and the constant cottage pair; compare constructor inputs at actual native callers before naming IDs.
2. Grid tests use actual canonical blueprint Vec values, deliberately conflicting colliders, rotated axes, two different cells and full48-byte payloads. Verify fixed stride50 and all unused dummy rows.
3. Test49/50/51 inputs to one cell, boundary/inclusive boxes, wholly outside each edge, positive/negative infinity and NaN projection. Preserve warning/filter branch and actual bounds-error behavior.
4. Same actual library/device host: both canonical buffers; successful900 then540000-byte write; metadata only after each success; first-write failure stops second; second failure preserves first; missing owners follow exact native optional branch/full-name collision asserts. Use current real host adapters, not a second BufferHost.
5. Two frames remove/move a hole and a third empty frame; uploaded canonical bytes clear stale membership. SameArc reupload remains. Root owns Cargo/typechecking/GPU/registration and a full App phase fixture. No execution has occurred for this contract.
