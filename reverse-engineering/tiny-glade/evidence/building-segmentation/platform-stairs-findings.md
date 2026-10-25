# Actual B platform stair creator

The private creator source is `crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs`. [three-version-platform-creator-owner-map.json](three-version-platform-creator-owner-map.json) records A/B/C native identities and different source checksums. C additionally splits stone/wooden/bottom-riser owners into new files; the B implementation does not assert C migration or A/C numerical equivalence. PDB paths/checksums are provenance, not original author source text.

Std Vec/HashMap, ECS query/changed-ID tracking, float-grid storage, glam arithmetic, fastrand1.8 and collision acceleration remain dependency boundaries. We reuse the existing recovered Curve2 selector, TerrainHeightGrid512 adapter, shared104B materializer and G04 wall instance producer. No HashMap bucket scan, lazy Mutex storage, framework tick tracker or BVH implementation is reconstructed here.

## Authoritative decorator input

Creator140774e8b..ea6 calls140a1c2c0 on its actual wall-scoped decorator resource, then scans value HashMap entries with80B stride. This is a game record inside a dependency-owned map, not an ECS entity stride. `WallPlatformDecoratorObserved(pub [u8;80])` preserves the observed source record without asserting author Rust ABI.

At140774f3d..fd5 the actual private filter reads:

| Record field | Actual condition/use |
|---|---|
| byte4e | family==63 |
| byte2a | exclude2/3/4; keep0/1/5..255 |
| f32[8,c] | WallSpace physical coordinate; use original raw Curve2 to_world |
| extra38, lowtag byte49 | original as_stair getter accepts low3 bits0/1; others panic if reached |
| f32 extra+4 =3c | actual stair width, not guessed asset width |

The caller samples actual TerrainHeightSsbo141246fb0 and keeps `coordinateY-groundY<3` strictly. The safe API returns typed invalid-extra failure in place of original panic. Finite width/coordinate proof domain is explicit. The terrain sampling fix/bounds/encoded512 field are required inputs; none is defaulted from fixture zeros.

`collect_platform_stair_blockers(rawCurve,terrain,records)` gives selected physicalXY+width in observed iteration order. [B-platform-blockers-verification.json](B-platform-blockers-verification.json) executes actual original filter, rawCurve2 constructor, to_world, terrain sampler and variant getter for384cases/960records. Only map row lookup/iteration and preallocated std output Vec capacity are resolved at the boundary.

## Steps to materialized segments

`prepare_platform_stair_segments` consumes the actual32B-step projection and source arrays. Steps are local intervals within their original LoopRange. Private141206070 converts prepared positions back through raw Curve2 and excludes horizontal distance strictly less than `width*.5+.2`. WallSpace distance140c91df0 only uses physical X; closed uses `min(abs(a-b),abs(period-abs(a-b)))`, with original raw length, without generalized modulus or Y distance.

Each permitted run of2+ points creates a range subslice, invokes shared process_segment with raw==shifted points/span10000, then shared104B materialization. This is separate from the platform high-mask resource. [B-platform-stair-segments-verification.json](B-platform-stair-segments-verification.json) covers192cases/467104B outputs, comparing all arrays/source indices/range. Its blocker list boundary is explicit; the later full-chain proof closes that input with actual80B rows.

## Actual partitions and collider geometry

Creator first uses104B length_ws140775512 and skips segments shorter than.5. For eligible segments,1407755b5..745 computes actual3D andXZ lengths; horizontal boundary count is `max(round(lengthXZ*.5),1)+1`. It calls actual utils::init_rng(0), whose state is0x4d58618879da0616, then random_splits with amplitude0. The original fastrand state still advances for interior boundaries; TileRng(0) would be wrong. Reuse existing random_splits/TileRng, not dependency RNG code.

Original materialized global Us are projected modulo1; start=end=0 with positiveXZ length means a full interval ending1. Each boundary pair interpolates the wrapped interval, projects modulo1, and selects actual prepared Curve3 coordinates1411f8c10. Curve2 and Curve3 share this scalar CurveU selector; the existing selector is adapted with actual3D Us, never recomputed fromXZ geometry.

At140775920..ff2 each cell reads the actual base maxY cache+108/+10c and constrained source extra+284. It samples terrain at both interpolated prepared endpoints offset1.948 along the outward horizontal tangent normal. The original3D tangent is normalized first, then itsXZ projection is normalized; glam math is reused with preserved operation order.

```
top = (maxY + f32(-.69)) + f32(-.1725)
terrainBottom = min((maxY-extra)+f32(.345), terrain1+.345, terrain2+.345)
count = clamp(saturating_u64(floor((top-terrainBottom)/.345)),2,4)
renderBottom = max(terrainBottom,top-count*.345)
depth = min(count*.448,1.5)
if omittedHeight > selectedHeight:
  renderBottom=max(terrainBottom,top-.69); count=2; depth=.896
```

The exact constants are evidenced native f32 words, not decimal re-rounding. Collider lower four corners use **terrainBottom**; upper inner corners use **top**, and upper outer corners use **terrainBottom+.05**. Selected renderBottom is separately stored and cannot be substituted into collider vertices. Original skewed_box_trigger_vol140ad9f10 produces8Vec3 and36indices with wrapping u32 index offsets. All vertex and index order is retained.

`platform_stair_cell(WallPlatformStairCellSource)` returns these scalar fields and indexed vertices. [B-platform-stair-geometry-verification.json](B-platform-stair-geometry-verification.json) covers256 original creator cases,2048vertices/9216indices, including2/3/4 levels, both terrain sampling branches and wrapping offsets. Its resolved Curve3 coordinates boundary is closed in the full-chain proof.

`platform_stair_geometry(public,preparedPositions,preparedUs,segments,terrain)` returns ordered cells plus actual3D `stairs_length`. MaterialStats::add_stairs_length1407774aa receives the sum of eligible segment3D lengths, not produced instance count or successful render-curve length.

## Actual platform holes

The final pass1407776b0..849 iterates every cached104B stair segment across the wall registry. It does not derive holes only from successful render cells. In particular a short segment skipped by geometry still contributes. The two segment endpoints are projected from raw Curve2 at maxY; last physicalX receives one raw period when below first. Average physical coordinate is divided by raw length, projected modulo1, then multiplied back. The actual producer emits existing `RawWallHole`:

```
center = [wrappedPhysicalCenter,maxY]
dimensions = [segmentLengthXZ+.4, f32bits(0x3f9ef9db)]
kind = 3
```

The helper14003bb80 incorrectly shares a framework-plugin name in decompiler output; its original body is RET and preserves the pair inXMM0/XMM1. No Bevy Plugin::finish behavior belongs here. `platform_stair_holes(rawCurve,maxY,segments)` reuses the existing WallHole type/storage. Original finalizer/store lifecycle and World hole replay remain the parent's integration responsibility.

[B-platform-stair-chain-verification.json](B-platform-stair-chain-verification.json) composes actual original rawcalc→platform steps→80B/terrain filter→104B consumer→original lengths/RNG/partitions/Curve3 coordinates→cell geometry→final holes for64frames,176segments/354cells/176holes. It compares all scalar and ordered geometry bytes and3D stairs-length sum. Each native owner executes original instructions, resolved at its source boundary; no captured mask, segment, partition or cell result is input. Original ECS scheduling, shape-specific render curves/96B rows and collider World lifetime are not accepted by this report.

## Remaining actual consumers

At1407760aa..dd0 original Circle/Rectangle closest-U/as_curve2/circular_slice rules create a render curve. Rectangle three-point slices with an edge<.05 collapse to two endpoints. Each rise offsets shape-specific normals, performs private dedup, and emits only ordered targetHeight>0. It calls the shared full WallConstructor with kind0/top-tag2/fencefalse, seed0, row-end perturbation.2, width/thicknessNone, arch filled by actual targetHeight, second array filled by terrainBottom-.46, third actual zero array, wall_float=targetHeight-terrainBottom. Negative effective-height bands may naturally produce no records; no invented clamp is permitted.

At140776e71 actual1408587c0 is the96B GPU-instance default initializer, **not a color getter**: affine/payload overwrite, flags0x10 and SparseWriter push remain to recover. At14077718c..476 the accumulated indexed collider is inserted/replaced if vertices exist, or removed when empty; generic Arc/BVH/collider registry are dependencies. Changed/deleted wall source cleanup must remove cached segments/instances and collider, and final all-registry hole pass must stay separate.

Commands:

```sh
cargo build --release --bin wall-platform-stairs-probe --bin wall-platform-blockers-probe --bin wall-platform-stair-geometry-probe --bin wall-platform-stair-chain-probe
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_platform_stair_segments.py
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_platform_blockers.py
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_platform_stair_geometry.py
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_platform_stair_chain.py
```
