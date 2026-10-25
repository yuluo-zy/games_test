# Decorator clutter producer contract — review before candidate

Status: native B evidence established below; no candidate accepted or runtime verification. This restores the current Bevy 0.16/B consumer chain by root instruction. Task-index target C remains a separate unverified version boundary. C has a different source checksum, a 6,399-byte generator and a new pack-change marking system; no B/C equivalence is claimed.

## Scope and ownership

Root delegated source families TG-b3ec56669fe9d97baf (generator), TG-652d8e2c9cc79a01e3 (necessary decorator helpers), TG-69cc2f973d7c14cb28 (blueprint), TG-5d112ff9638b93a02e (canonical ClutterMeshes), and TG-b374246e420cab4618 (loader). Claims remain unchanged. Only their work directories may contain candidates. Root applies production/Cargo/scheduling and runs tests/GPU. The visual builder belongs to 04; its exact shared-source hunk must be supplied by that owner. Collision world TG-f707… is foreign-owned and must not be edited here.

## Identity and exact evidence

Original oracle: B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`; PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`. `ORIGINAL-SCOPE.json` binds the retained decomp and original instruction spans.

* Generator `0x1407e3370`, PDB size **6,631 bytes**, 1,258 decoded instructions. The previously reported “42 KB” is decompiled text, not native code. The source span includes hot body and seven cleanup funclets; `generate_clutter.asm` preserves the entire span and runtime-function boundaries are recorded.
* Helper `0x1408b8e90`, **730 bytes**. Complete native span and original relative dispatch table are retained. Exactly entries 0..55 point into this function; words after the table do not form extra variants.
* Cuboid `0x1408cfff0`, **290 bytes**, byte SHA256 `751506853b9dd302e1ad0b4b7a7b214793f2612ad3dffcaae4b448629937fe0c`; calls `IsoTransform::from_translation` at `0x1408d0073`.
* Loader `0x1422c0760`, **5,472 bytes**, closure0 `0x1422c1cc0`, **928 bytes**. There are 55 actual calls to closure0, plus a separately generated dynamic vector. `loader-constant-call-arguments-B.json` contains exact strings and five scalar arguments from each original call site; `loader-final-resource-fields-B.json` binds each returned metadata pointer to its final canonical resource field.
* Original instance consumer `instantiate_blueprints` `0x140827910`, **7,991 bytes**, complete native assembly retained. Original retained decomp is `evidence/toolchain/decor-trunk/decompiled/140827910_instantiate_blueprints.c` (navigation; machine code resolves ambiguous pointers).

## Authoritative inputs and filtering

Original Bevy parameter spelling: DecoratorStorage, CachedDecoratorTransforms, mutable DecoratorBlueprints, PublicWalls, Query<&Roof>, UiSignifierStream, ClutterMeshes. This generator does not take Commands, DerivedDecoratorSubtypes, transient state, or a mesh library. None should be substituted as a parallel authority.

1. Iterate the native storage iterator (wall buckets plus its inline non-wall buckets), then each bucket's modified-id set, looking up its DecoratorInfo. A missing modified id has an original panic edge at `0x1407e4454`, so the current `iter_modified().filter_map` omission is not proof that a dangling modification can be silently accepted.
2. Native map entry is `{key, padding, DecoratorInfo[0x48]}` with stride `0x50`. The type at entry+0x4e is therefore `DecoratorInfo.ty` at +0x46. The predicate at `0x1407e3a76..3a8a` **skips** types 0x38..0x3d and 0x3f. The current `is_clutter_type` accepts this set and is inverted.
3. The byte at entry+0x2a is `DecoratorInfo.dst_tag()` at +0x22, not a new state/RNG field. `0x1407e3a90..3aba` admits only tags 2, 3, 4; tags 0/1 (wall destination) skip. Tag 2 (roof destination) enters a roof-lookup guard.
4. Roof guard resolves the bucket wall through PublicWalls::roof_entity and Query<&Roof>. Missing/stale entity or absent component means `is_flat=false`. Type 9 is admitted regardless; all other types require `Roof::is_flat()`. The guard still performs the roof lookup before checking type 9.
5. The remaining helper input must be a valid 0..55 ClutterMeshTy. Type 0x3e passes the explicit type exclusion but cannot legally index this helper. Its destination-domain invariant is not yet independently proven. A safe Rust boundary must reject unsupported values explicitly; no fallback-to-0/None is justified.

No direct RNG call occurs in the entire generator; no per-decorator seed is read after locating the record. Position, rotation and scale come from `CachedDecoratorTransforms::transform(id)`; missing cache is a required invariant/panic. Already selected decorator type determines the mesh; do not invent random selection.

## Canonical mesh and selector producer

The helper is not `fn() -> Option<u8>`. It takes a typed variant plus canonical ClutterMeshes and returns a metadata reference, a nani-name string and an Option selector. Dispatch maps types 0..28 to fixed fields +0x18..0xf8, type29 to the first element of the nonempty dynamic vector at +0x08/+0x10, and types30..55 to fixed fields +0x100..0x1c8. Type29 panics if that vector is empty.

Strings at original addresses are exact:

| Original string | Role |
| --- | --- |
| `nani_solid_vertex_color` | Base shader; helper selector None (native byte 9) |
| `nani_clutter_SolidAlbedoMap` | Regular solid albedo shader |
| `nani_clutter_SolidAlbedoMapEmissive` | Emissive solid albedo shader |

All nine selector values 0..8 occur in real loader metadata. Their names remain unknown; use 04's checked canonical NaniSolidTexture. Native 9 means Option None. No Default is justified.

Crucial consumer distinction: this generator copies metadata+0x20/+0x28 to builder.mesh_name and helper's returned string to builder.nani_name, but it **does not consume the helper's returned selector**. At `0x1407e41b3` it reads metadata+0x45; `0x1407e43dd` writes that byte to builder+0x150. The real producer of metadata+0x45 is closure0 `0x1422c1cf0`/`0x1422c1f3f`, from the caller's stack+0x48 argument. The JSON call and final-field evidence binds this writer. Supplying selector from a guessed enum/table while retaining fake metadata would not close the chain.

Metadata layout observed from real closure0 construction: Rectangle3d+0..0x20; mesh name str +0x20/+0x28; actual registered mesh identity +0x30; NameHash +0x38; radius +0x40; four scalar bytes +0x44..+0x47. Preserve the unknown scalar meanings as raw checked data until their consumers prove them. The closure loads/parses a real mesh, computes collision bbox/radius, registers the mesh under the same name and derives its name hash. Its pack-disabled branch intentionally makes dummy metadata and does not load/register a mesh; this is an original conditional, not permission to default the whole resource to unloaded.

Current loader's local `system_clutter::…::ClutterMeshes { by_ty: Vec<(mode_manager::ClutterMeshTy,String)> }` is a different Bevy TypeId and only registers mostly inferred names. It must be replaced/re-exported to the canonical country_core owner, loading real assets through current AssetMeshLibrary and retaining their returned handles. Do not add a second canonical-looking resource or assume the 56 guessed paths are correct.

## Geometry and builder output

Clone the metadata Rectangle3d, then call the existing Rectangle3d::set_size for these exact type-specific dimensions. Other types keep original metadata bounds. Constant bit patterns are in `tables-constants-B.json`; decimal spellings here are labels, not an alternate numeric oracle.

| Type | New (x,y,z) |
| --- | --- |
| 0 | (0.9,0.9,0.9) |
| 7 | (1.54,1.33,1.283) |
| 13,19,43 | (0.9,1.472,0.9) |
| 14 | (1.788,1.481,1.545) |
| 20 | (2.0,1.2,1.0) |
| 23,24,40 | (1.645,1.403,1.74) |
| 25,26,41 | (1.445,1.217,1.0) |
| 34 | (1.693,1.078,1.916) |
| 35 | (1.77,1.523,4.456) |
| 36,42 | (1.611,1.546,1.575) |

Let `w=footprint.size.x`, `h=top-bottom`, `d=footprint.size.y` after optional resize. First cuboid receives complete dimensions `(w,h,d)`, **not half extents**, plus local translation `(h*0.0*0.5,h*0.5,h*0.0*0.5)`. New-cuboid multiplies dimensions by 0.5 internally before using the original physics dependency. Second cuboid receives `(max_num(w-0.5,0.1),max_num(h-0.5,0.1),max_num(d-0.5,0.1))`, same original-height translation. SIMD NaN handling chooses 0.1 for the first two lanes; scalar MAXSS chooses 0.1 for the third. Preserve operation order, including h*0; do not replace nonfinite input with an invented zero center. Blueprint::set_colliders rejects nonfinite world translation or quaternion.

The original allocates two Arc<RaycastMeshData> and passes them, cached translation, and cached quaternion to Blueprint::set_colliders. That function replaces both the collision slot (+0x50) and interaction slot (+0x80). Current `Vec<DecoratorCollision>` is not this pair. Then the generator requires an empty outline/transform slot, installs the cached transform, and appends exactly one real WindowVisualEntityBuilder to the blueprint's visual list. There is no Commands spawn here.

Builder carries cached transform, actual metadata mesh name, helper shader name, actual metadata solid selector, and optional bird sit point. Exact bird offsets are type-dependent: types3/4 `(0,3.02,-0.25)`; 9 `(0,0.3,0)`; 11 `(0.85,1.08,0.378)`; 12 `(0.5,1.08,0.378)`; 16 `(0,1.59,0.45)`; 17/18 `(0,0.945,0)`; 44 `(0,1.56,0)`; other types None. Encode as actual 3×f32 bits through existing builder field; don't use current `has_birdspot` with only a boolean.

Builder size correction: original list item is 0x160 bytes. Consumer copies first qword separately, then the 0x158-byte tail, and passes the beginning to build at `0x140828a1e..a30`. The old builder documentation calling its full size 0x158 is wrong; metadata/name offsets +0xa0/+0xa8 remain correct.

## Blueprint instantiation, edit, removal, history

The original blueprint library stores current and previous per-id blueprint identities. `clear_blueprints` removes modified/removed ids (including removed buckets) before generators recreate admitted modified ids. Unmodified blueprint identity remains unchanged; `copy_blueprints_to_prev` preserves the prior map used by the next differential pass.

`instantiate_blueprints` first removes persistent visual entities whose old blueprint no longer exists. For each current blueprint, unchanged identity skips expensive visual rebuilding. Changed identity compares actual visual list and relevant output fields against prior blueprint; equivalent content retains existing entities. A real change removes old persistent entities, despawns them, clones the visual builders, invokes existing `build<DecoratorPersistentVisualState>` for each, then seals that decorator's persistent entry. A blueprint without its required outline/transform is an original logged panic, not an empty successful product. Collision slots and interaction slots participate in comparison and are registered with the canonical raycast owner, not discarded after visual spawn.

For clutter, edits/replay operate on DecoratorStorage and its modified/removed tracking, not on a new private entity list. Generator is deterministic for unchanged metadata, type and cached transform. Removal/type change/roof no-longer-flat must remove the old blueprint and persistent entities even when the new generator branch skips. Asset pack/reload changes must invalidate metadata/mark affected decorators through their original producer, not leave old blueprints stale.

The current source lacks the actual visual list, both native collider slots, instantiate system and canonical asset resource. A new visual-only direct spawn path would omit this lifecycle and is rejected by design. Full non-clutter fields of instantiate_blueprints (holes/cloth/chimney/stairs) remain separately mapped; they cannot be silently treated as supported by a narrowed clutter implementation.

## Proposed Rust integration and outstanding decisions

1. Restore canonical `resources::clutter_meshes::{ClutterMesh,ClutterMeshes,ClutterMeshTy,clutter_mesh_ty_data}`. Use real typed mesh handles and owned/borrowed name storage with clear lifetime. Keep numerical selectors checked, unknown scalar roles unlabelled, no successful empty constructor. A compatibility re-export removes the loader's duplicate TypeId.
2. Loader retains the 55 original constant load calls/order and six dynamic entries (original RangeInclusive 1..=6), actual bbox/radius/metadata+0x45 writer and original pack condition. Dynamic name ownership is still being finalized from closure1; no `Box::leak`/Arc choice has been implemented yet. Original post-load command inserts the canonical resource only after its actual contents exist. Current asset maintenance remains GPU owner.
3. Blueprint adds the native visual list and a real two-mesh collider/interaction representation. Preserve other current fields; no ABI-mirror byte arrays. Restore the native per-id differential lifecycle in this owner, with explicit unsupported boundaries for unrelated fields until their actual consumers are present.
4. Generator replaces inverted predicate, fake DerivedDecoratorSubtypes dimensions, ignored helper, false outline and dropped builder with exact inputs, bbox policy and appended builder. No seed/default/random fallback. The helper forwards cuboid creation through the real canonical physics API once the translation bug is resolved by its owner.
5. 04 owns any builder API/name-lifetime adjustment and doc correction. Root owns module exports, native-physics feature unification, registration and dependency order: clear → generator → instantiate → prior-map advance → tracking clear. Ordering must be based on original registration evidence rather than the current unregistered skeleton.

Open facts needing closure before implementation acceptance: canonical physics translation binding, dynamic asset name ownership and pack owner, per-type legal domain around 0x3e, exact native clear/copy scheduling, and all side effects of original instantiate fields. These are actual chain gaps, not reasons to invent defaults. Proposed tests after design approval: actual selector writers for all 9 values, exact path/table and resize/birdpoint coverage, roof/skip guards, first spawn, stable unchanged frame, edit rebuild, skip-after-edit despawn, removal and replay through real modified tracking, asset failure/empty-dynamic failure, actual shape center/dimensions. Native B differential and root runtime integration remain separate from compilation or candidate-only tests.
