# B resolved wall context → subtype → actual opening input

## Complete CPU consumer

`reconstruction/building/decorator_subtype.rs` restores the finite wall-destination path of original `derive_decorator_subtype` and `is_half_dormer`. It accepts the actual raw source Curve2, published mapped Curve3 outline, current arch heights, wall shape, and optional actual Roof record. It does not accept a preselected half flag or substituted empty roof trigger.

Original precedence is corner → door → half/plain. Cottage results are corner 3, door 4, half 1, plain 0; gothic results are corner 6, door 9, half 7, plain 6. The apparently shared gothic corner/plain value is preserved from original instructions, not renamed or guessed. Cottage corner subtype 3 is excluded from the four accepted opening families and returns an explicit unsupported-subtype error.

The half consumer executes shape filtering, flat-roof filtering, actual gable-side exclusion, raw wall coordinate→world, thresholded XZ nearest segment of the **mapped** outline, arch-height interpolation, and collision vertical-bounds comparison. Rectangle edge selection preserves original first strictly closer tie behavior, its 1000-unit initial distance and original denominator floor. The mapped nearest-segment search uses the separate original 1e-8 denominator floor. Original comparison is strict `>`; the unusual bounding-height expression from the earlier findings remains unchanged.

## Actual half resource and producer chain

B loader subtype 1 selects `window_cottage_{1,2,3}x1_halfdormer_collision`, distinct from ordinary cottage assets. Subtype 7 takes the same original loader branch and mesh names as subtype 6 (`window_gothic_{1,2,3}x1_collision`), while its library key remains 7. No asset alias is inferred by filename similarity: original bounded rank dispatch selects each name.

`cw-subtype-accept.asm.txt` confirms original cottage producer accepts subtypes 0 and 1. `gw-subtype-accept.asm.txt` confirms gothic accepts 6 and 7. Their raw-hole production ranges subsequently read rank, host shape, wall coordinate and collision metrics; they do not read subtype again. Thus existing raw-hole rules apply to accepted half subtypes with the correct selected collision records. This does **not** validate the preceding visual mesh transform/bundle path.

The runtime resource adds `load_b_wall_opening_assets(assets_root)` for 18 original keys (12 plain + 6 half keys). `derive_and_place_wall(request, door_attachment, corner_attached, context)` derives actual subtype using the plain-window collision entry, then retrieves the actual selected subtype/rank collision mesh and emits `WallDecoratorPlacement` with the true key and corresponding opening input. Missing assets and unsupported corner consumers remain explicit errors. A failed load preserves the previous library.

## Original instruction validation

`B-subtype-verification.json`: **1,326 cases, zero failures**:

- 1,152 complete original wall subtype executions: original entry 140813d00 through original half helper 140814470, mapped segment search 14126a450, and actual roof/rectangle helpers. Includes door/corner precedence, no roof, flat threshold, pitched roof, and both gable directions.
- 72 exact/adjacent-f32 comparison cases using actual six plain-window assets. The equality boundary remains ordinary; crossing the strict comparison changes half classification.
- 6 original half asset-name dispatch cases.
- 96 composed original subtype → actual selected half mesh bounding box → original family raw-hole producer cases for all six half keys and both host shape tags.

Only original PublicWalls lookup, resolved optional Bevy Roof query, and successful std collision-map lookup are bridged. Raw source/publication/roof/collision data are supplied; neither segment/fraction, interpolated height, subtype output nor hole output is injected. The actual B RNE digest, actual mesh hashes, candidate sources and candidate probe hash are bound to the report. B/C original `derive_decorator_subtype.rs` source checksum agrees, but A/C runtime behavior is not established by this B proof. C loader source differs and has a mod-swap reload entry; B resource policy remains explicitly versioned.

The new runtime module initially passed `cargo check --offline -p tg-building-runtime`. Parent integrates it with actual Roof lookup and refresh on publication, roof bytes, arch-height bits and asset revision. Attempted continuous World tests were temporarily blocked by concurrent edits in `wall_bricks.rs` (`Result`/`Option`) and the new 21-stage schedule tuple; those files are parent/other-agent owned. Parent performs the final full World run after integration.

## Continuous World fixture

The existing asset/parameter scene has raw rectangle X=[-3,3], Z=[-4,4], terrain floor 2 and extra 3, hence regular arch height 4.54. Cottage rank 1 at [-3,2,0] remains plain. At [-3,5,0] with height=1 and ridge=0.5 it becomes half and must select the actual half asset. Height=0.05 forces ordinary even at Y=5; raising it back restores half. With ridge=1 and direction=0, left edge [-3,5,0] is excluded; upper edge [0,5,4] remains half. Direction=1 excludes that upper edge, returning ordinary. Direction=0 restores half. These transitions require the 18-key loader and should update opening holes through the actual World consumer.

## Remaining boundary

Family intent still supplies style and, in the parent event adapter, initial door intent. Original authored attachment tags, corner selection and roof UV/raycast creation must be recovered upstream to remove that adapter. Full non-wall roof attachment, RoofTriggerVolume generation, drawable transforms and bundles, asset availability, and C mod-swap resource policy remain separate required consumers. Arrow-slit style 0x3a also uses the half helper but its family/assets/producer are outside this four-family package.
