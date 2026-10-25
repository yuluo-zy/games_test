# G04 construction authority: original B caller policy

Original business module: `system-wall-constructor/src/construct_walls.rs` and `country-core/src/geometry/instanced_wall.rs`. A/B/C PDB source identities and symbol addresses are recorded in [three-version-source-map.json](three-version-source-map.json). These are private game modules. `Vec`, `Arc`, allocator, Bevy system plumbing and shape ownership are reused, not reconstructed as game algorithms. The PDB paths/checksums are provenance, not recovered author source.

Rust policy: `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/building/wall_construction_sources.rs`.

## Exact B call-site inputs

Caller `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-integrate/B-core/140757800.asm.txt`, `14075925f CALL 140769760`. Offsets below are pre-CALL stack positions; callee entry offsets are +8.

| Input | Original source |
|---|---|
| RCX | normalized hole output Vec owner |
| RDX | rebuilt Curve2 XZ from **prepared** Public Curve3 +90 points/+98 count; conversion `1407586b0..1407586ea` |
| R8D | `BrickSourceId::from(WallId)` at `140858790` |
| R9 and stack+20 | copied arch heights Public +e0/+e8 |
| stack+28/+30 | extracted Y from prepared Curve3 +90/+98 |
| stack+38/+40 | Public +f8/+100 third per-point array |
| stack+48 | kind: no Roof + ClutterVersion0 =>2; pitched=>0; flat rim=>1/3 by WallStyle; others0 |
| stack+50 | `WallId * 0x4b28021 + Public+48`, wrapping u64 |
| stack+58 | finalized hole reference |
| stack+60 | row-end perturbation policy below |
| stack+68 | output Vec<96B> owner |
| stack+70 | Public +274 minimum published position Y |
| stack+78 | five floats + tag byte20; tag2 payload is inactive |
| stack+80/+88 | circle width-padding Option discriminant/payload |
| stack+90/+98 | circle thickness-scale Option discriminant/payload |
| stack+a0 | Public +284 constrained source height scalar |
| stack+a8 | literal false from actual caller |

`BrickSourceId` is a brick ownership ID, not a material category. Original low32 ID100000/100001/100002/100003/100011 maps to `0x80000000 | (id-100000)`; all others return low32 WallId. Table is original VA `142a953f0`; masked membership `0x80f`.

Source metadata copies: InnerWall+66 -> Public+295 ClutterVersion; +65 -> Public+296 special; +68 -> Public+299 WallStyle. `is_initial` reads ClutterVersion==0. A fixture with zeros does not establish a global default for these raw fields. [B-publication-source-fields.json](B-publication-source-fields.json) executes 512 arbitrary source-byte patterns through actual original loads/stores: +65 via `140a8ce58..140a8ce6f` and `140a8db74..140a8db81`; +66 and +5c via `140a8d81e..140a8d84b` and `140a8d904..140a8d928`. Scalar +5c copies to Public+288 without semantic naming. This is original schema evidence, not full publisher/Rust acceptance or creation-default proof.

## Prepared geometry is not initial terrain geometry

Initial publisher `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-holes/B-publisher/140a8b530.asm.txt` builds mapped Curve3 at Public +38..70. Its point count at **+48** seeds construction. It then explicitly initializes another Curve3 at Public+88..c0 and segment Vec at Public+c0..d8 empty (`140a8d928..140a8d97c`).

Construction and height cache read **+90/+98**, the subsequent prepared curve. It cannot be assumed identical to initial mapped +40/+48. Its true producer is `sample_wall_path_intersections`, original entry `14074c720`, exported under [B-intersections](B-intersections). The tail `14074e2fd..14074e3dc` rebuilds and writes Curve3 at +88 and segment Vec at +c0. `calculate_path_wall_segmentation` subsequently consumes this prepared state; it does not initialize it. Rust `prepared_construction_arrays` therefore accepts actual prepared Vec3, explicit arch and explicit third array, rebuilds XZ Curve2 through the existing recovered curve module, and rejects mismatched counts. It never resamples raw points or synthesizes heights.

Original init cache `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-runtime/B-14078b550.asm.txt`, `14078b7e7..14078b854`, allocates **zeroed** third array with count **Public+d0**, then writes +f0 cap/+f8 ptr/+100 count. This establishes lawful zeros only after that real producer. The relationship between the segmentation +d0 count and prepared +98 count still needs owner evidence; `initialize_segment_height_array` takes explicit segment-owner count. Subsequent writers are not ruled out by this initialization.

## Recovered scalar policy

Top tag!=2: row perturb0. Tag2 and noninitial clutter: original f32bits `3d3c6a7f` (.046). Initial clutter: `t=clamp01((rawCurveLength-.69)/.69)`, perturb `(3-2*t)*t*t`, using the original multiply order. Length comes from Public+28c, not rebuilt Curve2 length. Original SSE unordered/tie semantics are retained. Scalar/kind native report is 6144 cases with zero differences.

Only circle shape tag0 activates options. Radius=shape+0xc. Width padding `(1-clamp01(invlerp(0x3f666667,3.6,radius)))*.06`. Thickness `clamp01(invlerp(0x3f666667,3,radius)) + (1-t)*0x3f666666`. Rectangle/missing shape yields None; inactive register values are not defaults.

`plain_construction_policy` permits the producer's measured kind2/topTag2/false scope, which requires absent Roof and initial ClutterVersion. It carries actual minimumY/constrainedExtra. Other kinds return a typed gap rather than generating approximate draws. Five inactive tag2 floats are not modeled as meaningful zeros.

## Differential acceptance boundary

`/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/scripts/multi-version/verify_wall_construction_sources.py` executes untouched RNE B instructions for source seed, actual BrickSourceId leaf, actual private predicate/inverse-lerp helpers and scalar policies. No game output is bridged. Report binds only this owned Rust module and probe, so concurrent parent/runtime work does not silently change its claimed scope.

Original all-from_curve production is separately recovered by `/root/building_walls_audit`; see `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/building-production/B-wall-production-verification.json`. That production accepts explicit nonuniform/nonzero arrays and does not establish their authority producers. A/C schema and arithmetic execution remain separate work. World integration, segmentation authority, style/rim kinds, material/plaster/support rendering are not accepted by this narrow report.

Commands:

```sh
cargo build --release --bin wall-construction-sources-probe
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_construction_sources.py
```

## Actual empty path resources -> prepared path owner

New Rust `/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/building/wall_path_preparation.rs` exposes:

- `PathRasters512::from_encoded([Arc<[u8]>;4], sampling_fix)`: requires actual selected encoded buffers, validates extent and measures emptiness. No Default and no synthesized clear resource.
- `prepare_wall_path(mapped_positions, relative_heights, observed_source_5c, &paths)`: now accepts both empty and nonempty actual selected buffers. Source scalar and source geometry must be finite; no arbitrary NaN/Inf policy is manufactured.
- `PreparedWallPath { positions, points_u, length, segment_values }`: original +90, +a8, +b8, +c8; segment_values.len is +d0. This segment vector is distinct from the subsequently zero-initialized +f8 third array.

The owner reads four u8/255 path grids, samples the point and four normal-offset layers with exponential weights, smooths scalar strengths, masks by relative height, cuts at threshold .05 and deduplicates consecutive Vec3 positions. Original generic math entry `142923ec0` is **expf**, not sine; the initial ASM-only interpretation was corrected before implementation/acceptance. Final scalar transform is `(1-exp(-5*strength))*source+5c`.

When all four actual encoded buffers are zero, every original sample/weighted sum/smoothed strength is zero. The measured candidate branch still deduplicates consecutive Vec3s, rebuilds 3D U/length with existing glam finite norm math, preserves closing points and first equal/signed-zero input positions, and applies the final signed-zero scalar multiplication. This is an exact restricted source domain, not replacement of sampling results with a stub.

[B-empty-path-verification.json](B-empty-path-verification.json): 128 cases execute **all original resolved per-wall instructions** from `14074cbe0` through `14074e3dc`, including real 28 u8 sampling calls/point, original attenuation/smoothing/threshold loop, original Curve3 rebuild and scalar transform. Inputs include 2..33 points, adjacent duplicate and closing points, terrain-varying Y, both DLL sampling flags, relative-height gating, positive/negative scalar and signed zero. All prepared XYZ/U/length/segment bytes match. Only standard CRT expf/memory/allocation/reallocation/free are bridged. Invalid/nonfinite source, nonempty grids, outer EventReader/HashMap/profile wrappers and A/C execution remain excluded. First rejection-guard harness used the wrong textual token; it was corrected to change the encoded code, then the full run passed.

All three versions list private `crates/utils/src/float_grid.rs`; B/C checksum is equal while A differs. It is not a cargo registry package. No second generic grid container/sampler was ported. The nonempty branch reuses existing recovered `TerrainHeightGrid512` coordinates, bilinear evaluation and fix flag, mapping actual u8 codes to u16 by `code*257`. Both conversions are exact integers and `code/255 == (code*257)/65535` rounds to the same f32. The view uses bounds0/1; actual u8 native FloatGrid executes in the oracle. [dependency-exclusions.json](dependency-exclusions.json) and [three-version-path-owner-map.json](three-version-path-owner-map.json) record that boundary.

Required runtime migrations: regular circle/rectangle cache `14078b865/87c`, freehand cache `14078b9b1/9c8` and `14078bb0a/bb11`, gable source extraction `14078c00f/016`, and construction `1407586b0` all use prepared +90/+98. Gable then rebuilds XZ Curve2 U; its margin still uses raw +28c length. Initial mapped geometry remains the terrain/publication authority and seed-count owner. Decorator sampling must be traced separately before changing its geometry source.

```sh
cargo build --release --bin wall-path-preparation-probe
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_path_preparation.py
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_publication_source_fields.py
```

## Nonempty owner completion and exact decorator mapping

[B-nonempty-path-verification.json](B-nonempty-path-verification.json) executes the same complete original per-wall range on 128 nonempty-grid scenes, including four distinct simultaneously painted grids. All 1469 prepared point records, U, lengths and final scalars match exactly. The oracle observed 39612 actual u8 samples, 80 low-to-high transitions, 79 high-to-low transitions and116 height-step midpoint branches. Empty128 cases were rerun against the new implementation. Resource creation still requires supplied encoded buffers, with no default zeros.

Original policy restored: point/±four normal-layer samples at offsets .125/.25/.375/.5; weights exp(-2*((layer+1)*.25)^2); min of two sides; normalized weighted sum; five-point smoothing with original f32 weights; relative>.5 masking; threshold.05 crossing; near-end.999 guard; crossing-side midpoint only when distance>.1; both-active height-step midpoint when abs(deltaY)>.05; consecutive XYZ deduplication; `(1-exp(-5*strength))*source5c` and actual 3D Curve reconstruction. All computations keep original f32 operation order. Nonfinite/degenerate source domain, outer EventReader/HashMap/profile handling, buffer producer/swap lifecycle and A/C execution remain outside this native report.

[B-decorator-geometry-binding.json](B-decorator-geometry-binding.json) settles the downstream difference. `140814562 LEA RCX,[Public+88]` -> `140814575 CALL14126a450` reads prepared +90/+98; arch interpolation reads +e0/+e8. Half-subtype therefore migrates to prepared positions. In contrast placement `140c92090` reads Public+30 rawCurve2 length and +8 raw point pointer through `141940780`; retain raw source Curve2 for world-coordinate placement.

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_path_nonempty.py --cases 128
```

## General regular construction caller policy

New `construction_policy` preserves the old plain API and exposes `WallConstructionPolicy { kind, seed, brick_source_id, base, row_end_perturbation, options, wall_float, top, lower_flat_no_rim_arches }`. `ConstructionTopPolicy` is Plain (tag2 with inactive floats) or Gable {height,intervals,tag}. Inputs include actual optional Roof88, actual Public max-Y cache Option and explicit observed special flag.

The earlier entry guard is essential: `1407581b1 CMP byte[Public+296],0`; `1407581b5 JNZ14075936e`. Nonzero special/source+65 enters a separate construction body, so the regular-source API returns `SeparateSpecialConstruction`. It must not generate regular/false-fence-fit instances for this source.

Gable top reads actual cached Public+10c through `140b2a620`; shape must be the actual wall rectangle; intervals come from Rectangle.get_point_u with Roof::gable_sides direction d and d|2, each plus its next index. Style>=2 chooses tag1, otherwise tag0. Missing cache/unsupported rectangle returns a typed gap instead of an invented height or interval.

Before the regular from_curve call, original `1407585c6..1407586b0` clones arch heights. For a flat Roof (`height<=.1`, ordered) with rim.has_rim false (`Roof+3d!=0`), it applies each copied arch `MAXSS(arch + 0xbf23d70a, .5)`; original SIMD and scalar paths share the same constants. This changes constructor input only, not Public arch cache. `constructor_arch_heights` retains that contract.

[B-construction-policy-verification.json](B-construction-policy-verification.json): 512 cases execute actual Roof predicates, gable-side extraction, Public max-Y getter, Shape/Rectangle helpers, original kind/top caller and whole cloned-arch adjustment including scalar/SIMD boundaries, NaN/inf/signedzero arch bits, and32 special-gate cases. Only original CRT memcpy is bridged. Preserved [B-construction-policy-failure.json](B-construction-policy-failure.json) records the first oracle base-register setup error; restoring original RAX=PublicEnd yielded512/512 zero differences. The old6144 scalar/kind/source-policy report was rerun too. Instance production itself is independently owned by the producer worker; runtime integration and special construction body are separate acceptance.

```sh
cargo build --release --bin wall-construction-sources-probe
/Users/yuluo/Desktop/game/tools/venv/bin/python scripts/multi-version/verify_wall_construction_policy.py
```
