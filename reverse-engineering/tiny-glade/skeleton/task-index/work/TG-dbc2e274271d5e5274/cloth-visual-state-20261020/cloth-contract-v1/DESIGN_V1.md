# ClothInfo / DecoratorVisualState bounded design v1

Status: design only; candidate source not yet written. The authoritative write surface is existing `crates/systems/decorator/src/decorator_visual_state.rs`, root-delegated task TG-dbc2e274271d5e5274 / cloth-visual-state-20261020. Production integration, Cargo, runtime and GPU validation remain with root. Original evidence is identity-bound B only; no C-equivalence claim.

## Proposed shared interface

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ClothInfo {
    pub transform: country_core::utils::components::transform::Transform,
    pub wall: country_core::resources::walls::WallId,
    pub dimensions: [f32; 2],
    pub decorator_id: country_core::resources::walls::decorator::DecoratorId,
}

// Add to the existing global owner; do not install a second resource.
pub struct DecoratorVisualState {
    pub cloth_infos: Vec<ClothInfo>,
    pub persistent: DecoratorPersistentVisualState,
    pub transient: DecoratorTransientVisualState,
}
```

`dimensions` names an observed pair without inventing original field names, width/height ordering or a new shape algorithm. Both producers and the consumer must preserve the actual +0x38/+0x3c order. `WallId` is specifically the canonical outer key used by DecoratorStorage/PublicWalls (`resources::walls::WallId`), not the separately named publication_fields type. `DecoratorId` is the actual iter_modified key. Existing canonical Transform provides the ten observed float fields and ordered float equality.

ClothInfo has no Default, Eq, Copy, Resource or Component implementation, no raw-byte payload, no ABI promise and no new handle/allocator. Debug is host inspection support. Clone means cloning the known semantic values; it does not assert the original source used a particular derive. PartialEq compares all semantic fields. Original 180B Option<Box<ClothInfo>> equality rejects a compared NaN even for the same Box, equates signed zeros, compares both IDs, and ignores native padding. The existing Transform/IDs and `[f32;2]` support those semantics. The candidate should test every float lane rather than relying on pointer or byte equality.

## Original schema and ownership

PDB supplies the full ClothInfo name and typed relations but not a field declaration. The schema is established by complete native producer, consumer and equality bodies, not a stand-in or inferred size alone. `CLOTH_SCHEMA_V1.json` records each field and exact instruction witnesses.

Native allocation is 0x50 bytes aligned to 0x10. Effective semantic fields end at 0x44: Quat at 0, translation at 0x10/14/18, scale at 0x1c/20/24, WallId u64 at 0x30, two floats at 0x38/3c, DecoratorId u32 at 0x40. No business field or ownership is inferred from 0x28..2f or 0x44..4f padding. The original copy in fullvisual moves three 16B chunks plus two qwords and one dword; this includes some padding but does not make padding part of equality.

Cottage wall/corner window generators allocate and fill an `Option<Box<ClothInfo>>` in DecoratorBlueprint semantic +0x110 (outer raw Arc allocation +0x120). This is not an Arc<ClothInfo>. Replacing a prior Box directly deallocates 80 bytes/alignment16 with no nested field destructor. Fullvisual clones the effective value into a by-value first Vec with stride80. Original VisualState destruction deallocates that Vec allocation as capacity*80/alignment16 without per-element owned-field destructors. Therefore the bounded recovered fields can use a safe semantic value; unknown pointer Copy/Drop is unnecessary and forbidden.

Producer facts:

- Wall generator 0x1407ebfd0 / 14303B, PDB442:2224. Outer WallId load 0x1407ec3cd -> local+0x1f0; modified DecoratorId load 0x1407ec48e -> local+0x278. Inline add_cloth allocates at 0x1407ee313..324, copies Transform at 335..352, writes wall +0x30 at355, floats +0x38/+0x3c at359..366 and id +0x40 at36c..373. Prior Box deallocated at389..394; new Box stored399.
- Corner generator 0x1407f66b0 / 9983B, PDB445:1924. Outer WallId load 0x1407f6a56 -> local+0xc0, also passed to actual PublicWalls::get at0x1407f6c2c. Modified DecoratorId at0x1407f6b1a -> local+0x168. Inline add_cloth allocates0x1407f7e93; explicit Transform writes0x1407f7f0f..31, wall+0x30 at3f, pair at43/49, id+0x40 at4f, old Box deallocation65..70, store7c.
- Exact PDB inline-site records module442:19132 and445:14388 identify DecoratorBlueprint::add_cloth; both records are retained separately. No bounded evidence identifies generate_flags as this typed Box producer.

## Same global owner and lifecycle

Original DecoratorVisualState contains first Vec at+0, persistent map at+0x18 and transient map at+0x48 (overall observed owner0x78). Its original plugin default explicitly builds empty first Vec (`cap=0`, aligned dangling pointer0x10, `len=0`) at0x14034329e..2b4 and inserts that same resource via0x1403432c9. Empty Vec is correct initial state, not evidence that cloth generation is complete.

Original fullvisual0x140827910/7991B:

1. Clear the first Vec length at0x140827bc0..bca before the collider call0x140827bfc. This retains allocation and does not drain persistent/transient entity maps.
2. For every current blueprint, read typed Box at rawArc+0x120 (0x14082814c..15d); clone the actual value into firstVec at0x14082816d..209.
3. This append precedes the native differential equality gate. An unchanged blueprint still contributes its current cloth candidate. A frame with no current cloth candidates clears stale firstVec entries.

02 owns the fullvisual/Blueprint implementation and has agreed `Blueprint::cloth_info:Option<Box<ClothInfo>>`, `visual_state.cloth_infos.clear()` and push of `(**value).clone()` before that gate. Old `Blueprint::cloth:Option<u8>` remains explicitly legacy and must never be promoted to this typed field. No ownership of 02 source is transferred to this candidate.

Original unload0x140347090/9451B calls actual VisualState drop at0x140347916, then writes a full default owner at0x140347922 onward (empty Vec plus reset maps). This is whole-owner replacement/drop, distinct from per-update firstVec.clear. Current unload source is incomplete; this design does not modify that foreign owner or claim unload integration. Ordinary safe Rust fields/Default allow the original replacement once root restores its caller.

Current visible `system_decorator::init_resources` initializes only global DecoratorVisualState. Wrapper persistent/transient types derive Resource and many current generators request them separately; this is a visible interface-routing mismatch, not proof of running duplicate resources. This design adds no standalone resource registration and does not claim whole-App validation. Future generator migration must use nested fields of the actual global owner.

## Consumers and distinct cloth domains

Known direct typed firstVec consumer is original generate_clothline_visuals0x1407e5190/11167B, PDB440:2044. It calls typed from_iter0x141261410 at0x1407e547a, which maps `Iter<ClothInfo>` through the complete434B fold0x141293730 to `flag_connections::CandidateInfo`. The fold consumes Transform, wall, pair and id; it applies its original geometric projection and adds CandidateInfo-specific fields (+0x44/+0x4c). CandidateInfo is not a type alias of ClothInfo. The original consumer then calls greedily_connect at0x1407e572e and clothline at0x1407e712c. Their business algorithm and output build are outside this one-file shared-state candidate.

The retained exact function-type catalog contains six VisualState SystemParam signatures: clothline, fullvisual, clear_out, unload, a flags-like producer signature and rollercoaster state access. Their wrapper spelling alone does not prove binding or which field is read. Complete selected native bodies prove the typed producer/firstVec/consumer/lifecycle paths above; this is not whole-program field-use completeness. The read-only manifest keeps the full six signatures without silently upgrading unknown wrapper-to-game binding. `clear_out_visual_entities` currently drains the nested transient map, separate from firstVec clear.

Do not conflate:

- `system_decorator::decorator_visual_state::ClothInfo`: this window blueprint candidate -> firstVec -> CandidateInfo projection.
- `country_core::components::ClothlineCloth`: original Nani set44 query `(&Key,&Transform,&ClothlineCloth,&NormalMap)` with With<InstanceDataMarker<ClothInstanceData>>, textures/GpuWorld. It is a rendered cloth instance payload, not this input candidate.
- `country_core::components::WallFlagCloth`: raster wall flag parameters, prior20B raw stand-in/canonical migration separate.
- `RoofFlagCloth`: separate roof raster parameter.

No Nani kind, raster callback, raw cloth parameter or whole Roof/Cloth plugin is changed by this design.

## Current visible-source gaps and requested separate ownership

All observations are scoped to the source hashes in SOURCE_SNAPSHOT_V1.json. Root is integrating other work and user pending-sync modules exist; these observations do not assert a complete App has run or that the user has not implemented an upper registration elsewhere.

1. Existing VisualState lacks firstVec and leaves ClothInfo marked U. This proposed one-file candidate can remove that obsolete U, add the safe typed value/field, preserve nested state behavior, and add meaningful CPU tests. It alone does not create cloth candidates or render cloth.
2. Current cottage_wall_window.rs and cottage_corner_window.rs construct then discard WindowVisualEntityBuilder, do not write typed Box ClothInfo, and do not implement the original actual geometric values. They require separate root claim allocation to restore real producers; no default Transform/zero dimensions should stand in for those values.
3. Current generate_clothline.rs takes DecoratorStorage/CachedDecoratorTransforms/standalone transient, calls collect_flag_anchors, creates quads and discards them. It never reads VisualState.firstVec or uses the original10-param contract. This is the earliest concrete consumer gap after typed fullvisual accumulation. Please allocate this existing source (and only if needed its exact flag_connections.rs algorithm companion) to a separate bounded producer/consumer recovery task. It must consume the same authoritative VisualState Vec and build using actual asset libraries/Commands; an empty list or manual flag anchors is not equivalent.
4. Current decorators_on_glade_unload source has an empty body; true whole-state default replacement belongs to root/that owner. This design records required behavior without a foreign edit.

The shared-state candidate needs no new source module path or Cargo/lib registration. Proposed downstream paths needing separate authorization are the two existing window generator files, generate_clothline.rs and possibly flag_connections.rs; not all are part of this candidate.

## Candidate validation contract

Only after scoped design approval and root authorization, prepare one-file candidate against frozen current beforeSHA; root applies it. Do not mutate this design manifest or prior Nani manifests.

Planned unrun CPU tests in the same shared-state source:

- Explicit nonzero ClothInfo values clone/equal and changing either actual ID or either dimension makes them unequal.
- Each of the12 compared float lanes containing NaN makes a clone unequal; +0/-0 in those lanes stay equal. This checks semantic equality rather than raw padding or pointer identity.
- The same global VisualState preserves nonempty persistent/transient maps while clearing/repopulating its cloth Vec; typed values remain independent clones when a producer Box is replaced. This is a shared-owner unit boundary, not proof of real generator execution.
- Global Default is exactly empty cloth list plus ordinary empty nested maps. No ClothInfo Default is introduced.

02's actual fullvisual CPU tests must additionally exercise two explicit nonzero typed Blueprint values, append before equality-gate skip, stale clearing on the next frame, persistent/transient map preservation and Prev ownership. The consumer's future tests must execute its actual query/function with nonempty typed data and real output ownership; tests of this type alone cannot certify that chain.

Already verified: ten fixed complete original native ranges (nine retained here plus 02's7991B fullvisual) are continuous and every instruction byte equals the direct original PE bytes; see PE_RANGE_VERIFICATION_V1.json. No Cargo, compiler/typecheck, CPU tests, full-App scheduling or GPU validation has run for this design.
