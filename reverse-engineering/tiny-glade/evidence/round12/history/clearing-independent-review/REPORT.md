# Independent clearing + theme helper review

Review of other owners' complete clearing producer and Olden/Flowery helpers. Production and Cargo were untouched. Original B RNE SHA256 is `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`. This report does not independently accept this reviewer's forest consumers/audio.

## Actionable findings

1. **Particle transform lost signed-zero operations.** Actual production `startup/native_placement_policy.rs` used `Quat::from_rotation_y(rotation)`, while original particle closure140a123a0/521B explicitly forms X/Z quaternion channels by multiplying sine by zero. Original native execution of360 additional cases retained their signs. An actual standalone Rust probe includes the production source and links the already-built glam0.29.3:20/360 cases fail exact output-word comparison. A one-line `Quat::from_axis_angle(Vec3::Y, rotation)` candidate passes360/360. The source-bound result, actual rustc commands, binary hashes and outputs are in `particle-rust-comparison-results.json`. Production source SHA256 `bf87adf54d6f5454266a5a2b9ed09e07fd1f888d1d5e2fe6048d9ab953c2e31c`; candidate file raw SHA256 `53e68fc2e8dc284c502d10e3e5d887e386b146fc754453d59482035aadbef879`. Apply only `particle-axis-angle-erratum.patch`. This uses the real dependency, without recreating general quaternion math.

2. **Actual App omitted two forest consumers and misplaced the third.** Fresh original ForestPlugin::build140650c50/243B constructs `(manage_tree_prefabs, forest_tree_animation, handplaced_tree_animation).chain_ignore_deferred()` at140650c73, passes GameSystemSet byte36 at140650c78, and adds to Update through140ea7c90 at140650ce7. No run condition is attached. It then inserts the canonical empty InteractableForestPlacements resource, processed=false. `forest-registration-original.json/.asm` preserves original raw bytes, per-function hashes, complete PDB aliases and the set vtable. Central must register exactly this tuple/order/set and remove the existing handplaced_tree_animation from history_runtime's40→41 prefab chain to prevent two ticks. Existing forest CPU helper has the wrong order and `.chain()`; it is not an accurate substitute. This registration does not certify the separate, incompletely recovered handplaced canopy-distance owner/algorithm.

## Producer → canonical forest consumer contract

The candidate producer writes exactly80-byte ForestTree records: canonical core PrefabName, full48-byte PlacementDataSsbo, Entity option initializedNone, billboard_expansion_scale. All placement channels are copied, including IDs, custom channels and packed bend data. It keeps exterior positions under `!(abs(x)<65 && abs(z)<65)`, including exact boundaries and unordered coordinates. The24-byte InteractableForestTree component is a distinct ECS component and must not replace those records. Native1412fa010/500B supports these offsets and filter; color's canonical types match this consumer contract.

No implicit degree conversion, Y-height replacement, scale default, forest-record ID-as-state, or mesh-based record reconstruction is present in the reviewed producer. Only exact `clearing/tree/bush` is skipped. Autumn's distinct `clearing/tree/autumn/bush` remains, as original comparison requires.

## Five shipped themes and loading lifetime

`five-theme-independent-inventory.json` is generated directly from original shipped JSON, with each config/placement/emitter mesh SHA256, group counts and original float32 filter calculations:

| Theme | Exterior forest records | Complete emitter vector | Published count | Upload bytes |
|---|---:|---:|---:|---:|
| Summer |1777|0|0|0|
| Olden |682|0|0|0|
| Flowery |629|330|330|5280|
| Autumn |1777|610|610|9760|
| Winter |1390|0|0|0|

Olden helper's six actual placement files contribute60+200+6+11+5+6 rectangles. Flowery helper's lavender file contributes240 records. No repeated placement-library paths exist between a theme's clearing loop and its helper. This is an independent asset inventory, not an actual Rust/GPU session result. Particle emitter mesh index arrays are empty, matching positions_without_indices' precondition. Synthetic boundary fixtures remain needed because shipped data does not exercise1100 truncation, missing storage, NaN and all thresholds.

Native AssetPlacementDataLibrary::load1409365d0 rejects a duplicate loaded path; do not add caching or silently return an old handle. The candidate uninit clears the canonical buffers and paths, and clearing is registered perOnEnter(Game), allowing a subsequent session to reload the same paths. AssetMeshLibrary::add creates a fresh handle and replaces name/source aliases; repeated mesh loads therefore must not be treated as the placement-library duplicate error or deduplicated by policy. Actual two-session App acceptance remains central's responsibility.

## Failure and optional-storage ordering

The reviewed main preserves the original observable order: Winter mesh/watch/register→shader→deferred spawn; theme helper side effects and returned Olden resource insert/non-Olden remove; parse trees config; clear old forest; each group's placement-library load→append exterior records→optional emitter mesh load/transform; conditional full-vector upload; publish capped count; compile common billboard shader→load/watch/register billboard mesh→deferred spawn. An intermediate failure must retain already-performed library mutation and queued commands until the actual caller's panic/error boundary; no transaction rollback or continue-on-error was added. Production caller uses `.expect(...)`, rather than treating failed initialization as success. Helpers load their placements and scene entities in original order; this review found no further concrete omitted branch in their complete native ranges.

Original SSBO `None` is real:140a796a4 calls typed get_mut;140a796b3 branches directly to140a79739 when absent, and140a796bc does the same for empty emitter data. Present+nonempty checks **all bytes**,140a796c5, and uploads them at140a79723; insufficient size and upload failure take original error branches. Only afterward140a79739..75d publishes min(len,1100). Candidate matches; requiring Some in the game helper would alter original behavior. An actual nonempty App acceptance should separately assert the canonical SSBO is present and receives5280/9760 bytes, so None cannot conceal a broken startup producer.

Common meadow_flower shader producer exists in COMMON_SHADERS; billboard_plant_shader is generated by terrain Startup before the per-session Flowery helper. The theme helpers consume those exact shader/library owners and canonical material component TypeIds. No default shader, detached material-view component or fabricated handle was needed. Original helper native execution and their authored Rust fixtures remain distinct evidence; this reviewer did not run full helper/Bevy/GPU acceptance.

## Verification boundary

Observed: fresh original registration bytes/PDB; complete original particle521B executed360 times; actual included Rust production and one-line candidate360-way bit comparison; shipped5-theme independent inventory; static complete original-main/helper control-flow and owner comparison. Not claimed: actual device upload, FMOD, complete two-session App, or render output. Remaining executable checks belong to central integration, with real nonempty Flowery/Autumn data, initial and reload sessions, canonical forest processed reset and actual placement-library buffers/paths.
