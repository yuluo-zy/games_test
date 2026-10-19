# Cottage-window native numeric helper contract v1

B only. This is a scoped numerical/helper design, not approval to replace either complete window generator. C parity is unverified. No production/Cargo/GPU changes. Current window source comments, old helper tests and the discarded identity builders are not original-behavior evidence.

## Canonical inputs and ownership

Reuse country_core::utils::components::transform::Transform (rotation, translation, scale), actual AssetMeshLibrary/Mesh attributes, actual required DerivedDecoratorInfoView/DecoratorMeshLibrary lookup, canonical RawWallHole and the existing DecoratorBlueprint.visual_entities/wall_holes. No local IsoTransform, LintelRecord/Visual mirror, secondary resource, missing-mesh skip, default collision, or zero payload. The collision/interaction schema and loader are owned by01; its full table API is a dependency, not yet a callable integrated producer.

Info::wall_space_coord() is the actual two-f32 coordinate from first8 B. Info destination.platform is an independent original+08 ID, while actual outer storage wall remains the authoritative storage/Cloth wall. Reuse the root-integrated7da61633 codec. The original wall-space21B leaf returns the value {is_loop: shape.tag()!=2, length: original_curve_length}; the current Option<f32> compatibility method must not be read as absence of the length for an open wall. A local semantic WallSpace value can use the existing actual shape().tag() and wall_space_length() without a second geometry owner.

## flowerbed_xforms — complete473B plus own map closure810B

Engineering API proposed: flowerbed_xforms(rank:u8, subtype:u8, parent:&Transform, mesh_lib:&AssetMeshLibrary)->Vec<Transform>. The native RCX output is a return-place fresh Vec, not an append-to-existing-Vec contract. Its DL is rank, not derived subtype. mode/subtype0 or1 selects the three standard location mesh names; subtype3 selects the three corner names. rank.wrapping_sub(1)<3 is required. Mode2 and other out-of-domain modes fail by their original branches. Required mesh lookup and required Float3 normal attribute fail; empty lists are only legitimate for genuinely empty typed attributes. Zip stops at min(positions.len(),normals.len()).

For every pair(p,n), use the existing true utils::rotation_from_normal(n), then output.rotation=parent.rotation*local_rotation, output.translation=parent.rotation*p+parent.translation, output.scale=parent.scale. Parent scale is copied; it is not multiplied into p. Preserve actual quaternion product order. This projection does not replace the existing rotation_from_normal algorithm. The original full936B is retained only to bind that real dependency.

## Actual wall subdivision and segment transforms

The native caller1407ed6c7..7be captures actual collision width, actual destination coordinate and actual PublicWallState. The own inlined closure in the complete417B TupleWindows::next context and its two-map1227B host establish this sequence; std Range/Vec/itertools implementation stays dependency-owned.

Let half_offset=width*f32::from_bits(0xbf000000). Let start_u=wall_space.project_unbounded(coord.u+half_offset). Let start_y=(half_offset*f32::from_bits(0x00000000))+coord.y. Let step=width/(pieces as f32). For usize i in0..pieces+1, compute distance=(i as f32)*step, y_offset=0.0*distance, actual_u=wall_space.project_unbounded(start_u+distance), actual_y=start_y+y_offset. Convert the actual WallSpaceCoord through the real simplified curve's existing to_world_space; do not substitute cached placement, a straight line, endpoint interpolation or a duplicated curve algorithm.

For each adjacent point pair(a,b), compute delta=b-a, reciprocal=1.0/sqrt((delta.x*delta.x+delta.y*delta.y)+delta.z*delta.z) in the original scalar/add order read from the host; the exact machine host uses XY sum followed by Z. Construct the bad-XZ quaternion from Vec2(-delta.z*reciprocal,delta.x*reciprocal), midpoint components=(a+b)*0.5 (sum then multiply), and proven unit scale. Keep0*distance/0*half_offset, division, signed zero and NaN behavior; no simplified y=coord.y or normalize_or fallback. The current systems/decorator/utils/mod.rs three-arg pair-copy helper is a different algorithm and is not evidence of recovered subdivision. Its shared owner/API must be coordinated before any source restoration.

Caller pieces selection is not fully covered by this helper contract: rank2 draws exactly one full-u32 high bit before subtype1 handling; rank1/rank3 branches, post-lintel RNG consumption and the exact caller flags must remain in the forthcoming complete generator contract. This helper design alone does not permit replacing those paths by a fixed count.

## add_lintels — complete2637B and true20B output

Engineering API should consume actual modified ID, DecoratorInfo, &[Transform], exact source semi-transparent byte/color bits, mutable actual blueprint, actual derived-view lookup and the three actual scalar/flag inputs. Native extra stack slots not read by the body need not be simulated as unused raw fields. Do not retain the old standalone LintelRecord/Visual return objects. Exact external input semantic names beyond the shown reads stay neutral until main-caller projection is complete.

1. original coord comes through dst_as_wall, retaining its wall-tag check. shift=(y_parameter+f32::from_bits(0x3fb70a3f))*0.5. Call existing coord.with_y_offset(shift).
2. Obtain actual subtype by modified ID, make required canonical library key from subtype and info.rank (native new ABI takes rank/subtype and returns logical subtype/rank). Lookup actual collision directly by value. Missing collision is panic, not None/empty. Actual bounds.footprint.size.x at native+30 contributes dimension_x=width+f32::from_bits(0x3eb851eb).
3. Append canonical RawWallHole {center:[coord.u,coord.y+shift], dimensions:[dimension_x,f32::from_bits(0x3e4ccccd)],kind:0}. Native first two words are floats from WallSpaceCoord::with_y_offset, not u32 handle/wall. Empty source slice still appends this real hole after required lookup; no visual values are invented.
4. For every source Transform, start translation=(source.x,source.y+shift,source.z), retain source.rotation and source.scale.y/z. Rank<2 retains source.scale.x. Rank>=2: first shifts position by minus source.rotation*Vec3(f32::from_bits(0x3da5e353),0,0), sets scale.x=f32::from_bits(0x3f51eb85); middle sets scale.x=f32::from_bits(0x3f23d70a) without positional shift; last shifts plus the same rotated-X vector, sets scale.x0x3f51eb85. A single source is first-only, not both first and last. Do not use Vec3::splat(.081).
5. Native computes height_ratio=height_parameter/f32::from_bits(0x3f666666); when flags&1!=0, multiply selected scale.x by ratio. Retain the original bit0 predicate and division/multiplication ordering.
6. Append a genuine existing WindowVisualEntityBuilder to the same blueprint: exact mesh decorators/setdressing_window_lintel, shader nani_solid_vertex_color_window, actual pose/scales, Some(actual ColorId bits), actual semi-transparent flag. Other optional fields are the original constructor's proven None/tag2 states; they must not be manufactured as active zero components. Raw ID/color/state inputs and real per-source field read order will be bound to the final main generator contract. Do not call build here or discard the builder.

The old source's1.43/.36/.081 decimal literals do not exactly match the native bits above, and its parameter labels/empty optional result do not prove any original failure semantics.

## Candidate/test boundary

No source candidate accompanies this helper-only document. A future source candidate must retain actual original typed6/7 generator parameters, exact subtype0/1 versus3 gates, full per-shape/rank output and failures, real Box<ClothInfo>, whole blueprint source flags and current ownership, with no isolated helper proving scene reachability. Same-world tests should cover nonidentity parent rotation/translation/nonuniform scale, actual normals and unequal Zip lengths, required mesh/attribute failure, nonempty two/three-piece lintels, first-only singleton, exact RawWallHole floats and color/state values, signed-zero0*distance, real canonical builder output. Tests remain UNRUN until root executes same-source code.

## Remaining bounded prerequisites

The complete generator numeric/phase mapping, two main calls' scalar/flag/ColorId binding, current required four-table API, true subdivide shared function source/API, shutter+40/+48 consumer semantics (new writer evidence sent03), actual WindowLight canonical migration (root-owned), and actual missing-owner publication fields are unresolved integration dependencies. This document does not count the two windows or upstream ClothInfo business producer as recovered.
