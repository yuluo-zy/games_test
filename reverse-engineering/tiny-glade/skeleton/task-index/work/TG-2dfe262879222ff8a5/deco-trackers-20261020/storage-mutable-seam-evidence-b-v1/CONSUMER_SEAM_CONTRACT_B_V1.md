# Anchor consumer mutable seams, B original evidence V1

This is read-only evidence and a minimal consumer design under TG2dfe. Storage TG889 and core anchors remain foreign source; no claim/write authority is inferred from this package. B V2 six-source hosts are independently integrated; these anchor seams do not extend that admission or its unrun tests.

The complete native four-function pack is 273+99+29+178=579B, with PDB module540 procedure rows and instruction bytes independently matched to the original B PE. Selected original anchor callsites are likewise matched. Collector boundary_verified stays false; address/length cross-binding is recorded separately. The public hashbrown insertion dependency is reused by symbol/PDB identity, not reconstructed or modified.

## Proven guard and selected-ID behavior

- wall_decorators_mut140a1e420 first compares Storage+1b0 at e42b and branches at e432 to e518 panic when nonzero. This occurs before even the empty-per_wall check e438, so immutable missing/empty storage must also fail. Return/miss map search cannot be used to bypass the guard.
- iter_walls_mut140a1e540 first compares Storage+1b0 at e544 and branches e54b to e58a panic before making the map iterator. Both use the exact33-byte string at142ac9098, `assertion failed: !self.immutable`. The original anchor invokes this method at14081cff9 after consuming its four event streams, even if no modified row will subsequently exist.
- MaybeMut is per Info, not an enum wrapping the whole Decorators bucket: its real next140a1b200 returns id atout+0, info pointer atout+8, pointer-to-modified-ID-set atout+10, id atout+18. Its get_mut140a1b1e0 loads wrapper+8 as the insert target and wrapper+10 as DecoratorId, calls1402c9200 (HashMap<DecoratorId,()> insert), then returns wrapper+0 as the Info pointer. It has no independent storage immutable check; the outer mutable bucket/iterator owns that guard.
- Anchor roof invalidation at14081ce2a reads via shared get, ce2f checks Info+22==2, only then ce38 calls get_mut. Height invalidation repeats atcfd8. The temporary returned mutable pointer is not written there; the observed effect is selected ID insertion into modified tracking. Neither all-ID mark nor cleanup/clear is equivalent.

## Current gaps and bounded legal implementation direction

Current Storage.get_mut already asserts immutable. Current wall_decorators_mut and iter_walls_mut omit the proven guard. Current DecoratorMaybeMut is a bucket-level Ref/Mut enum and iter_maybe_mut returns shared entries; neither matches the per-Info native adapter. This package does not propose a broad replacement of that public type across unknown callers.

A legitimate Storage owner can make wall_decorators_mut delegate to existing guarded get_mut(Wall(wall)), and add the same guard at the beginning of iter_walls_mut. The guard must run before empty/missing returns. This changes neither map ownership nor creates a registry. Any root-authorized source change still needs independent review and current caller tests.

The owned anchor consumer can implement only its necessary roof invalidation with canonical guarded get_mut, safely splitting the existing Decorators.decorators map and modified_ids set. Iterate the authority map, inspect Info.dst_tag()==2 before inserting that exact map ID into modified_ids; do not change the Info or mark unrelated IDs, clone all entries, call mark_mutable, or clear tracking. This is the current architecture adapter for the observed native temporary get_mut effect, not a claim that the current MaybeMut type has become correct globally.

After the outer guarded wall iterator, the ordinary modified loop can split-borrow existing modified_ids and decorators to avoid the existing per-wall Vec collection. The exact missing-modified-ID failure/skip contract still belongs to the native iter_modified_mut consumer evidence and must be retained; this pack does not invent an unwrap or skip policy. Event iteration order, missing-Public panic only after real modified entry, original ordinary position write before optional subtype lookup, actual outline borrowing and typed collision owner remain the previously frozen nine-parameter requirements.

The core ordinary-prefix seam remains distinct: existing move_wall_decorator_following_anchor rejects a bottom flag before ordinary work, whereas original ordinary stores14081d2e1/d2e5 precede bottom optional lookup. A legal core owner can extract the existing single arithmetic body into a prefix helper usable by the nine-parameter consumer, retaining the legacy API's explicit BottomDoorConsumerRequired contract. Do not clear the flag to fool the old helper or duplicate the shared curve algorithm. Actual canonical subtype/collision schema and loader still belong to01; Legacy bbox is not accepted as the bottom collision.

No source candidate, test, Cargo/rustc, original execution, App/GPU or throughput result is produced by this pack. Required coverage includes immutable empty/missing bucket and iterator, roof-only selected marking, duplicate event progress, missing modified-ID semantics, ordinary-before-None, and real bottom collision integration.
