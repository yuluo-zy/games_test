# G03 WallPathSegmentationMask authority

Original private business modules and A/B/C debug checksums are listed in [three-version-owner-map.json](three-version-owner-map.json). B gameplay entry: calculate_path_wall_segmentation14074f310 -> calc14074f7a0 -> process_segment140750420. Std Vec/HashMap/Arc allocation, probing and retention mechanics are excluded; private closure calculations inside generated retain wrappers are gameplay and are recovered independently of Vec storage.

## Actual environment and outputs

The pure calc owner receives only output Vec and current PublicWallState. It does **not** query neighbour wall cells, Roof components, stair entities or terrain grids. Those inputs affect its actual prepared data through preceding publishers/path sampling. Multiwall handling is the outer OnWallChanged reader and independent PublicWalls lookup. Do not invent geometric intersections from spatial-cell candidates.

Current calc input: prepared Curve3 points +90/+98, path float Vec+c8/+d0, wall shape+258, observed special+296. It adds path values to prepared Y (and multiplies by0/adds to XZ under original SIMD order), gathers each positive run with one adjacent point on both sides, merges closed outline's first/last runs, constructs original LoopRange and processes each run in order.

The 136-byte output is **WallPathSegmentationMask**, distinct from later 104-byte WallPathSegment. Identified safe fields:

| Original range | Identified value |
|---|---|
| 0..24 | Vec<u8> flags |
| 24..48 | Vec<ThresholdedCurveCoord>: u32 local point index, f32 fraction |
| 48..72 | lazy Vec initially empty, element schema opaque |
| 72..96 | lazy Vec initially empty, element schema opaque |
| 96..120 | lazy Vec initially empty, element schema opaque |
| 120..132 | LoopRange {start:i32,end:i32,modulus:i32} |
| 132..136 | fitted/sampled source length f32 |

The native complete calc harness [native_segmentation.py](native_segmentation.py) executes all private geometry helpers and process_segment, bridging only existing standard allocation/math/memory and disabled puffin profiling. Empty source returns[]; straight open run0,.3,0 yields flags020004, endpoint coords(0,0)/(1,1),range(0,3,3),length2. These raw masks are oracle evidence and cannot be injected as World authoritative input.

## LoopRange and coordinate contracts

Rust file `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/building/wall_segmentation.rs` provides stable safe types for the materializer worker.

- Closed outline modulus=prepared_count-1; open modulus=prepared_count. The duplicated closing endpoint is excluded from modulo period.
- `len=end-start` within original valid signed32 constructor domain.
- Point local ordinal maps to `(start+local).rem_euclid(modulus)`. Original process uses signed idiv and adds modulus to a negative remainder. Start may be negative when first/last positive runs merge.
- `ThresholdedCurveCoord` is local to this range. Last point with fraction0 becomes `(count-2,1)`; other points preserve fraction, including signed zero. Do not treat it as a global prepared point ordinal.

[B-schema-empty-verification.json](B-schema-empty-verification.json): 1024 actual original closed/open constructors+len,1024 actual thresholded coord constructor,128 full calc empty-result cases all match. The modulo rule is supported by original instruction addressing; this report does not separately execute a standalone index helper or accept nonempty candidate generation.

## Registry and MaterialStats

Outer wrapper14074f530..55e updates **MaterialStats at Public+188**, calling init_this_update140a88280 with prepared3D length+b8 and special+296, then set_path_segmentation_count140a88440. It separately inserts Vec<Mask> into a HashMap keyed by WallId at1411fc1e0. That function is a standard HashMap insertion and must be reused, not reconstructed as business.

## Complete nonempty producer

`wall_path_segmentation(WallSegmentationInput)` now computes actual masks from prepared points/path values/shape/special. It rejects unequal counts and nonfinite arrays rather than creating implicit cache inputs. Original calc accepts the published matching-count contract. No event or captured mask injection is part of this API.

Shape-derived span: Circle perimeter inverse lerp10..18, Rectangle9..12; clamp then `4*t+2*(1-t)`; other shapes4; special flag overrides3. Positive path runs acquire their immediate preceding/following points. Closed first/last runs merge into a negative-start LoopRange; materializer must use `index_at` for source addressing.

Native140751610 computes the private candidate score from a2-unit left/right-expanded chord. Its reverse iterator starts at index+1: an already flagged following point can leave the left endpoint at the candidate. Score is `(distance_XZ_to_chord+interpolated_chord_Y-current_Y)*.5`, followed by minus.4/clamp0. First strictly greatest retained candidate wins; std retain and swap_remove reproduce source ordering. This signed valley criterion is not generic3D RDP.

Unmarked spans rebuild an actual XZ Curve2 using existing geometry. `floor(length/span)` controls evenly spaced anchors, with count1 suppressed when length<1.5*span. Fraction>.5 selects the next storage slot; exactly.5 selects the lower slot. New coordinate remains the actual sampled local segment/fraction. Flags bit8 can overwrite endpoint coords. Therefore final mask length samples **new firstcoord→new lastcoord**, not necessarily the whole LoopRange.

Original lazy Vecs48/72/96 remain empty; safe Rust types omit their unknown element layouts. No third height array(Public+f8) writer is present in calc/process. Real masks are separate from `PreparedWallPath.segment_values` and the later104B materializedWallPathSegment. The G05 worker consumes typed flags/coords/range, not captured raw136bytes.

[B-masks-first-failure.json](B-masks-first-failure.json) preserves the first79/272 failures before original reverse-iterator and endpoint-coord rules were corrected. Subsequent enlarged matrices and actual4raster→prepared→mask chains are measured in [B-masks-verification.json](B-masks-verification.json). That report accepts only its explicit finite source domain and B original body, not A/C equivalence, original event schedule, World integration or GPU output. Dependency scope is [dependency-exclusions.json](dependency-exclusions.json).

```sh
cargo build --release --bin wall-segmentation-probe
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_segmentation_schema.py
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_segmentation.py
```

## Identified MaterialStats policy

`material_stats_for_update(prepared_length, observed_special_flag, mask_count)` returns `WallPathMaterialStatsUpdate {regular_path_length,special_path_length,segmentation_count}`. Actual140a88280 branches special==0: +90=prepared_length,+9c=0; nonzero: +90=0,+9c=prepared_length. Actual140a88440 writes count at+ac as low32. [B-material-stats-verification.json](B-material-stats-verification.json) compares these actual original fields over512 arbitraryf32bits and count/special cases. Original also resets opaque accumulator/cache bytes+68..cc; safe projection does not invent their meanings or assert complete MaterialStats restoration.

Final Rust file uses existing glam::Vec3::length for norm and existing Curve2 for XZ construction/coord interpolation. Generatedretain and TakeUntil storage code stays excluded. The left inclusion rule is independently visible in B-sampling/141237c20.c: private predicate reads flag low4, then the iterator includes that marked point and terminates the next iteration. If firstpoint=index+1 is marked, no pair can be formed and left coord stays at index.

Next third-array owner candidate is B140751960 `split_wall_path_segment_into_platform_stairs`. It is not yet accepted as a writer; source/consumer needs complete original evidence. Current complete calc136B mask and later104B materializer must remain distinct.
