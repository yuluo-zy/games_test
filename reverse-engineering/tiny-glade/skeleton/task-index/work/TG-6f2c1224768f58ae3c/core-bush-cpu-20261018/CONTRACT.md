# B canonical mapped Bush positions consumer

Owner core-bush-cpu-20261018, root claims TG-6f2c1224768f58ae3c (upper reader) and TG-6b43449ef07afe08ea (canonical core constructor). Only the upper reader candidate and necessary clutter/lib registration adapter are delivered here. Root owns canonical BushCpuSsbo construction and production/Cargo integration. No production files or Cargo changed by this worker, no native target execution.

## Original identity and corrected evidence boundary

B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; matching PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Six identity-bound LLVM spans contain394 instructions/1730 bytes, no instruction overhang. Reader140eebaa0 is **750B**, from exact PDB module1081 record260. Root's older745B locating span ended at the final CALL opcode140eebd88; its178 instructions/744 decoded bytes are not a complete reader. The old evidence remains untouched. The new750B pack has180 instructions and includes that final full bounds-panic call. calculate_hash141cf7270 is454B from exact PDB procedure metadata. Other spans are explicitly bounded comparisons/registration, not claimed complete functions.

## Data owner and business corrections

Canonical element supplied by root is transparent [u32;2], NAME `Bush positions for CPU`, MappedStorageBuffer<'static, BushCpuSsbo>, BUFFER_LEN9601. The upper fake Resource{count,positions} had no production producer. It is removed; its import path reexports the existing canonical type, and compatibility BushPositions aliases that element. No second mapped buffer, count Resource or mirror positions list is introduced. The existing BushGrid per-cell owned Vec<Vec2> remains the real CPU grid owner.

Original reader does canonical AssetSsboLibrary lookup; missing entry/downcast returns before grid mutation. The current canonical library already owns full Rust type checks. Read mapped[0].0[0] as count; positions are rows1..count+1, each two u32 bits reinterpreted as x/z. Empty mapped slice panics, and len<=count panics before clearing old grid lists. The second header word and trailing rows beyond count are ignored. Root's constructor creates a real9601-element mapping; tests may register deliberately shorter typed spans to observe the original failure rules.

Native sort is unsigned lexicographic **[u32;2]**, not f32::total_cmp. Original insertion comparator141cf9cd5..ce4/9d26..37 compares u32 words via unsigned CMOVB and progresses to word2 only when equal. Reuse std sort_unstable_by_key on the two raw float bit words; do not reconstruct insertion sort or ipnsort dependencies. Signed zero, negatives and NaN payload identities remain exact.

Native calculate_hash initializes zero-key DefaultHasher state, writes slice len usize, then for each pair writes array length2usize and eight raw bytes. Reuse std Hash for [u32;2], keeping existing u64 hash API. Caller140eebc62 XORs only EAX/EDI (low32 cell hash and empty-list hash), compares/stores one u32. Thus return widened u32(hash^empty_hash), and empty normalized hash is0. This prevents initial4096 empty cells from all becoming dirty. No SipHash algorithm is handwritten or newly introduced.

Current DirtyTiles, world_rect_cells and publisher stay authoritative. Radius0.7 and grid indexx*64+y are retained. Native stores a changed cell hash before checking dirty index<4096 at140eebca0..cb7; the candidate explicitly asserts that bound before calling the existing DirtyTiles::set_cell (which otherwise silently ignores invalid coordinates).

## Positive registration evidence and current adapter

Original Plugin build14065226c calls chain_ignore_deferred140ee2560. That function140ee2723 stores the reader's full function name at142b97980 (87B), then140ee27ce passes its vtable142b978c8. The retained vtable includes run_unsafe140ee1ec0, whose actual direct call140ee204e targets reader140eebaa0. Build652271 assigns native set byte0x24, calls in_set65229f and App::add_systems652310. `registration-and-types.json` retains these bindings and exact PDB procedures.

Current configure initializes the existing AssetSsboLibrary only if absent, preserving host-loaded entries, and removes fake Resource initialization. It adds the reader once after existing subscriptions and before current populate via a separate add_systems call. The current existing chain has20 systems; adding a21st tuple entry would exceed Bevy's tuple support. This is the current graph's data-dependency adapter, not a claim to map native set0x24 onto an invented current set. No other clutter system is reordered/rebuilt.

## Validation source and explicit remaining consumers

Both final candidate sources parse with rustfmt (skip_children for the isolated lib candidate) and both patches pass same-batch git apply --check. Eight regression sources: missing canonical entry retains grid/publisher; valid count0 header emits no dirty batch; actual canonical library/mapping finite negative/positive data sorts raw, publishes four cells, ignores poisoned header/tail, ignores reorder-only updates, publishes changed bits and later clear once; NaN/negative-zero raw sorting/hash identity versus std array Hash; empty span and bad count panic; out-of-capacity dirty index panic; actual configure graph contains exactly one reader and preserves preloaded typed storage. These tests have not been run or compiled here; root owns execution. No native/JIT run or approval-limit bypass was attempted.

The tests supply an actual owned mapped page registered as the canonical storage. They represent completed host readback and do not claim GPU submission/fence synchronization. The root constructor's device/map behavior has a separate candidate/review.

A separate existing `populate_autoclutter_regions::BushGrid {cells:Vec<[f32;2]>}` remains an unused system parameter (`let _=&mut busher`), while the original PDB uses the reader's BushGrid there. Current populate subscription/acceptance/filter logic is therefore still incomplete. This task does not create another fake grid or rewrite that other module under a different claim; full bush exclusion/population, the global renderer completion boundary, and shared geometry nonfinite/extreme-coordinate behavior are not declared closed. NaN regression here validates raw storage sorting/hash identity, not a new world-rect sampler policy.
