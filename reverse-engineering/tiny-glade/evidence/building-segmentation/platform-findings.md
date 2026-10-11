# Platform mask and step range owner

Original A/B/C source metadata in [three-version-stairs-owner-map.json](three-version-stairs-owner-map.json) identify the private `crates/systems/wall-constructor/src/segmentation/split_wall_path_segment_into_platform_stairs.rs` module. Its source checksums differ across all three versions; this B implementation does not assert A/C numerical equivalence.

B outer owner140751960 consumes OnWallChanged IDs, raw calc mask resource, published walls and actual Roof query. It clones unchanged masks when shape>=2 or no validated Roof component or Roof::is_flat1408e3380 returns false. Flat predicate is an **ordered** Roof+30<=.1 comparison: NaN is false. Original input calc map and output processed map are separate resources; raw masks and MaterialStats calc count must not be overwritten. Native140751d1d..d82 binds input and output resources;140752309..317 inserts into the processed map. Third32B-list resource is separately reset for each changed wall.

Eligible regular shapes with an actual flat Roof iterate each original mask. Helpers141202370/141205db0 read `PublicWallState::max_y140b2a620`: flag+108 and **base value+10c**, not gable+110/+114. First helper computes `maximum - max(preparedY, f32bits3f0ccccd=.55)` over the original LoopRange. Second builds a prepared shifted Curve3: add path value to Y only when that height>=3; preserve original zero multiplication/addition in XZ and below-threshold Y.

`threshold_range1411ffbd0` then partitions contiguous local values. Low<=3 runs of at least two points append a step record. High>3 runs of at least two points call actual process_segment140750420 with the original LoopRange subslice, prepared raw points and shifted points, span4 (or3 when Public296!=0). Unlike the initial positive-run calc, this threshold owner adds no preceding/following point. There is no captured-mask injection into the safe candidate API.

Original32B step layout has28 identified active bytes:

| Range | Meaning |
|---|---|
| 0..8 | start usize, local original mask ordinal |
| 8..16 | end exclusive usize |
| 16..28 | original mask LoopRange {start,end,modulus} |
| 28..32 | ABI padding, no authored semantic field |

`WallPlatformStepRange {start,end,source_range}` preserves this contract. Do not treat start/end as global prepared ordinals. Original raw mask coords and length remain unchanged in the source resource. `LoopRange::subslice141200510` adjusts global signedstart/end by local offsets and keeps modulus; the complete threshold native oracle executes this helper directly.

Rust `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/building/wall_platform_segmentation.rs` exposes:

```
split_platform_wall_masks(
  public: &[u8;672], prepared_positions:&[[f32;3]], path_values:&[f32],
  raw_masks:&[WallPathSegmentationMask], roof:Option<&[u8;88]>
) -> Result<WallPlatformSegmentation, WallPlatformSegmentationError>
```

Result fields are `processed_masks` and `steps`. Public bytes are the existing observed publication projection, not an asserted original Rust memory ABI. Runtime supplies its actual published snapshot and validated relationship; missing/stale Roof resolves to None. Empty raw masks do not read maxY. Eligible nonempty arrays require matching lengths, finite values and a valid base maxY cache; failures stay typed rather than choosing a guessed default.

The old mask module changed only process_segment visibility to `pub(crate)` for shared algorithm reuse. Std allocation, HashMap, clone, generated from_iter/take_until storage and ECS lookup mechanics are dependencies. Existing recovered Curve2 and glam norm remain reused. The reconstructed code preserves private threshold and output rules; it does not reimplement a generic threshold/container library.

[B-platform-masks-verification.json](B-platform-masks-verification.json):384cases,941processed masks,432steps, zero differences. Actual whole original calc creates raw masks, followed by the actual private141202370/141205db0/1411ffbd0 chain. Compare all processed flags/coords/range/length and step28active bytes. The gate executes original Roof::is_flat; generic allocation/memory bridges are the only substitutes. MaxY+114 is set to99 independently of+10c to exclude a mistaken gable authority. This verifies the resolved CPU owner, not the original EventReader/ECS/schedule or actual step mesh consumers.

Third construction-height writes are separate: [B-third-height-writers.json](B-third-height-writers.json) binds construct_arches and construct_lintels original stores. Platform thresholding itself does not write Public+f8. G05 worker restores the lintel writer alongside its full support consumer.

```sh
cargo build --release --bin wall-platform-segmentation-probe
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_platform_segmentation.py
```
