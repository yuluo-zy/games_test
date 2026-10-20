# Real UI hover picker — candidate, not production

The candidate source is `proposed/tiny-glade/crates/country-core/src/systems/ui/hover_intersection.rs`.
It depends on the canonical WidgetParams/UiState candidates owned by roof_support_producer in
`../roof-ui/proposed/tiny-glade/crates/country-core/src/systems/ui/resources/`.
The separate complete ui_update/get_ui_volume owner is escape_menu_full in `../ui-update/`.
No production files or shared App registrations were modified by this delivery.

Integration requires `pub mod hover_intersection;` in country-core systems/ui/mod.rs and
`bevy_tasks.workspace = true` in country-core/Cargo.toml. The workspace already fixes the reused
Bevy tasks0.16.0 dependency. Do not reconstruct its generic task/executor/sort implementation.
Mount the original complete set6 nine-system chain supplied by the ui_update owner; do not replace
that complete tuple with an invented two-system tuple. UiState hover_intersections is the original
first Vec of56B records. The input owner consumes its first result and clears frame widget
registrations after updating interaction; active drag holds a separate Arc.

## Observed original contracts

* PDB hover outer3887B@1408383a0; game picker closure is in async host3455B@140ac3f50,
  module565. Original jump table142adfb28 has **eight entries**, tags0 through7. Earlier
  reports referring to six variants omit PlaneDisc and Empty.
* Signature independently confirmed by ui_update owner through original SystemParam generic:
  ResMutUiState, ResMainCamera, ResCursorTerrainRaycast, ResCursorPositionSS,
  ResMouseDepthValue, ResWindowSize, LocalHoverIntersectionCache.
* Hover recomputes Ray::from_mouse_ss using current cursor/window and camera inverse matrices;
  it does not consume set10 CameraCursorRay before that producer has run.
* Tags: Sphere0, SphereCloud1, TriangleMesh2, ScreenAabb3, WorldCanvas4,
  TerrainCanvas5, PlaneDisc6, Empty7. Mesh/cloud preserve nearest element and first equal-distance
  element; clouds reject nonfinite closest distances. Meshes return the selected triangle normal;
  clouds intentionally have no normal. Screen rectangles preserve their explicit distance.
* WorldCanvas uses Rectangle2d centered at zero and current terrain XZ, including the original
  zero terrain position when no terrain hit exists. It gets normalY, distanceMAX, occlude=false,
  and a separate layer priority. TerrainCanvas preserves its supplied distance, position, no normal,
  and bypasses world occlusion. PlaneDisc uses the real Plane helper, min_t0 and strict radius²
  greater-than with original scalar arithmetic order.
* Native retain509B@1412f6680 recomputes the hit position from ray/distance and the negative viewZ.
  It retains an occluded hit only when MouseDepthValue.view_depth+**1.0** is strictly greater.
  The view row is camera native+88/+98/+a8/+b8, i.e. Mat4.z row, not Mat4.y.
* Negative/NaN distances are discarded unless flags bit2 is set; remaining ordinary hits are
  stably sorted by partial_cmp distance (original unordered comparison panics). Canvases are
  stably sorted by unsigned layer and appended after ordinary hits.

## Independent original execution

`native_picker_oracle.py` executes the unchanged entire original async host, including all original
private geometry helpers. Its only bridges are allocator/free/Vec capacity and CRT memory.
`native_picker_boundaries.py` records39 original cases, covering all tags, empty collections,
nearest/tied/behind sphere lists, mesh ties, screen edges, canvas XZ edges, non-unit rays,
plane strict radius/parallel/behind cases. Full records are in native-picker-all-fixtures.json.

`native_arbitration_oracle.py` separately executes the unchanged outer postprocessing bytes
140838bfd..140838eec, including the real retain and stable sort code. Twenty-one cases cover
strict depth equality, occlusion bypass, negative-distance permission, signed zero/ties, layered
canvas order and nonidentity view transforms. It does not evaluate a candidate or a Python model.
Results are native-arbitration-fixtures.json. Explicit code hooks stop both oracle regions, avoiding
Unicorn translated-block cache crossing an emu_start until address.

`native-manifest.json` hashes each exact original span, native-cfg.json records conditional/direct
edges, calls and the complete8-way table. Original RNE SHA is
f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
ReVa get_decompilation was attempted but returned Program not found/no program loaded; no
claim relies on that failed call. PDB void() placeholders were observed and were not used as ABI.

## Validation status and remaining acceptance

Actual candidate files are directly included in direct_candidate.rs (canonical candidate resources
plus current compiled core/utils/glam/Bevy dependencies), with dependency SHA/command recorded
in direct-candidate-manifest.json. A direct rustc run was launched before the central compile
throttle. That actual candidate compiled successfully but the39-case fixture test FAILED on
case21, screen-edge-1: original no hit, candidate one hit. The current private helper
utils::geometry::aabb::is_inside_bbx incorrectly delegates to inclusive Aabb2.contains.
Original44B@14123ffc0 excludes all four boundaries (cmpltps and seta). The narrow
is_inside_bbx.patch corrects only this private helper, preserving Aabb2.contains and all other
optimized code. It is a candidate patch, not applied to production. The picker still calls its
real private helper; no tolerance or fixture was adjusted to hide the failure.
No new codegen will be started until the central owner grants a compile window.

The candidate now contains tests for all39+21 original cases, and a real scheduled picker test
registering an actual widget, moving CursorPositionSS50→90 and observing changed results without
injecting UiInteraction. These latest added tests need the central Cargo/direct compilation window.
The combined real input→hover→ui_update→wall/history App regression remains with the integration
owners and is not claimed complete from isolated geometry samples. T13 remains excluded.
