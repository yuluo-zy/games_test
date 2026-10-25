# Actual dimensions-to-box-triangles helper

TG-b0d4797db2453f6794 was fresh-showed null and normally claimed to clutter-startup-20261020 at2026-10-19T19:33:06.045812Z after central authorization. Only box_trigger_volume@140ada0e0/1302B, its genuine internal triangle mapper and necessary existing local helper tests/callers are in scope. No rectangle orientation UI parent, skewed_box_trigger_vol or other source behavior is being recovered. Central alone mutates production/Cargo/GPU. Before source is archived whole with its SHA.

## Complete original and real caller

B f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 / PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911, GUID09F1A02A-49F2-F8C8-4C4C-44205044422E age1. Full1302B body140ada0e0..140ada5f6 is retained, PDB567:87968. Full actual business map/fold1414784a0/690B (1610:81144) and collect1412fd740/558B (1585:210448) are also retained with statusok; their standard-library symbol names do not remove the game closure from scope. No target binary is executed. Vertex-index lift is merely a bounded lane construction aid, not original execution or a Rust test.

Actual load_collision_as_bbx1407bb4b0/1284B calls this at1407bb602. RCX is Vec<Triangle> sret; XMM1 actual collision rectangle width, XMM2 actual top-bottom height, XMM3 actual depth, caller stack+20 points to the true rotation/translation/scale Transform. At original callee entry the same pointer is stack+28, then+370 after push/sub. It is not (indices,Mat4,positions), and there is no subtype parameter here. Caller builds real collision centered/rotated Transform, scaleONE, from required actual Mesh::collision_bbx and keeps the triangles plus real cuboid mesh and actual rectangle in the resulting collision record. No bbox-only substitution or default box.

Proposed canonical current signature is `pub fn box_trigger_volume(width:f32,height:f32,depth:f32,transform:&crate::utils::components::transform::Transform)->Vec<Triangle>`. This reuses the actual existing core Transform (rotation,translation,scale at0/10/1c) and actual existing Triangle.points[3] record; no raw native ABI layout imitation, new transform type or second mesh library. The later owned load_decorators producer will call exactly this interface with original dimensions and real Transform. Current workspace has no production caller of the wrong signature; only the three local test calls, enumerated in the archived whole file, require adjustment.

## Real local vertex/index generation

Fullbody128..2bc builds24 position Vec3s using actual width*-.5/width*.5, height*-.5/height*.5, depth*.5/depth*-.5. Negative factors are multiplication by-.5 (not a later subtraction fromzero), preserving f32 order/sign lanes. Native constants142aa5d60=[-.5,-.5,.5,.5],142a8cc80=[.5,-.5,0,0] are retained. There is no dimensionabs/clamp, nonzero/finite guard, vector repair or early empty return. The six duplicated face tables preserve actual order and winding:

| Vertex | XYZ |
|---|---|
| 0 | width*-.5, height*-.5, depth*.5 |
| 1 | width*.5, height*-.5, depth*.5 |
| 2 | width*.5, height*.5, depth*.5 |
| 3 | width*-.5, height*.5, depth*.5 |
| 4 | width*-.5, height*.5, depth*-.5 |
| 5 | width*.5, height*.5, depth*-.5 |
| 6 | width*.5, height*-.5, depth*-.5 |
| 7 | width*-.5, height*-.5, depth*-.5 |
| 8 | width*.5, height*-.5, depth*-.5 |
| 9 | width*.5, height*.5, depth*-.5 |
| 10 | width*.5, height*.5, depth*.5 |
| 11 | width*.5, height*-.5, depth*.5 |
| 12 | width*-.5, height*-.5, depth*.5 |
| 13 | width*-.5, height*.5, depth*.5 |
| 14 | width*-.5, height*.5, depth*-.5 |
| 15 | width*-.5, height*-.5, depth*-.5 |
| 16 | width*.5, height*.5, depth*-.5 |
| 17 | width*-.5, height*.5, depth*-.5 |
| 18 | width*-.5, height*.5, depth*.5 |
| 19 | width*.5, height*.5, depth*.5 |
| 20 | width*.5, height*-.5, depth*.5 |
| 21 | width*-.5, height*-.5, depth*.5 |
| 22 | width*-.5, height*-.5, depth*-.5 |
| 23 | width*.5, height*-.5, depth*-.5 |

Indices are literal six groups `[b,b+1,b+2,b+2,b+3,b]` for b=0,4,8,12,16,20, exactly36usize entries written140ada2c5..457. Map chunks of3 in that order and collect exactly12 real Triangle values; don't replace the24 face vertices with8-corner topology just because the positions overlap geometrically. Duplicate face vertices and triangle winding remain observable to the collision consumer.

## Transform and map arithmetic

140ada463..55e reads actual Transform quaternion/translation/scale and builds the three rotation axes using the original doubled-component quaternion formula, scales each axis by its matching actual scale, and embeds translation. Current Transform.compute_matrix/compute_affine uses the pinned glam0.29.3 S/R/T constructor and same semantic owners; its source fingerprint is external. No quaternion normalization/defaulting is introduced. Native has no explicit guard rejecting non-unit or nonfinite quaternion; any optional glam_assert host feature would need to remain an explicit current build boundary, not silently be called native behavior.

The map transforms each local Vec3 via separate f32 multiplications and adds in order `(y*axisY+x*axisX)`, then `z*axisZ+previous`, then translation. For the first and second triangle point native final operand order is previous+translation; for the third it is translation+previous. A narrow private actual map can spell those operations with matrix columns instead of invoking the old generic Mat4::transform_point3 whose source always adds translation first. Do not use mul_add, f64 or normalize/finite filtering. The matrix's unused W lane does not become a perspective divide. Keep actual XYZ values and use Triangle::new; current Triangle is exactly three points and performs no geometric validation.

The internal map's original bounds and chunks checks are real: an index>=24, a chunk missing index1/2 or chunksize0 panics. Canonical outer always supplies valid36/3/24 and therefore cannot encounter those checks under its own literal topology. Vec allocation/drop/panic remains standard Rust collect behavior; no retained raw pointers or fabricated success/error marker.

## Current defect and old test migration without deletion

The existing public same-name function is only the inner arbitrary-index/position map. Preserve that genuine logic as a private clearly named mapper invoked by the restored outer, with actual matrix column arithmetic above; it must not remain a parallel public wrong native signature. Existing transformed arbitrary positions `(1,2,3),(4,5,6),(7,8,9)`, shift(10,0,0), one/two triangle tables and out-of-range24 panic cases remain exactly present and tested by that private map. These are closure-domain fixtures, not proof of dimensions-to-box generation. Historical original whole source is retained in before/. No case deletion, ignore or tolerance expansion.

New actual-method tests should verify all12 triangles against the exact24/36 topology for nonuniform dimensions2/4/6, actual translation+nonuniformscale+rotation, reflection via negative dimension/scale, zero dimension's actual degenerate triangles, and nonfinite lanes propagation without filtering. Tests should distinguish exact simple finite expected values from arbitrary-rotation floating arithmetic with independently grouped f32 expressions. Keep all skewed-box/UI tests unchanged. This worker does not run Cargo/GPU; central execution follows independent source review.

## Producer boundary

This helper alone does not populate DecoratorMeshLibrary, prove original collision schema migrated, implement full17386B26-subtype/rank loader or register genuine Startup producer in the actual World. Those are the same owner's separate authorized loader/subtype/outline closure. The producer must retain actual triangleVec+Arc<RaycastMeshData>+Rectangle3d and fail on required assets, never infer ready from an empty Resource. Independent1302B/mapper/ABI and test-domain review precedes candidate writing.
