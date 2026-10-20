# Decorator visual restoration, 2026-10-20

This batch restores the complete B `check_for_door_stairs` leaf. The surrounding
`generate_gothic_balcony_doors` visual producer remains partial. No decorator
end-to-end or overall percentage increase is claimed.

## Source changes

- `decorator_visual/door_stairs_geometry.rs`: original lazy terrain/path sampling,
  transformed front/left/right points, all rejection branches, quaternion-derived
  direction, and all eight assembly payload fields.
- `decorator_visual/gothic_balcony_door.rs`: actual five-owner adapter using the
  canonical `TerrainHeightsData`, current `WorldRaster<512>` read buffer,
  `CachedDecoratorTransform`, `Rectangle3d`, and existing blueprint's boxed
  `DoorStairsAssemblyParams`. Success replaces the box; failure preserves it.
  The partial caller now supplies those actual inputs instead of fabricated
  zeros. Two actual-resource adapter tests are authored but not run here.
- `cottage_dormers/cottage_dormer.rs`: the current canonical Rectangle2d method
  is `get_closest_u_from_pos`; the obsolete method call was corrected narrowly.

The old leaf incorrectly interpreted actual terrain samples as wall-space
evaluation, interpreted path coverage as terrain height, derived a direction
from three height differences, always rejected large drops, and used door height
for payload fields that are actually fixed -0.8 and collision width. All those
misinterpretations have been removed. Normalization has no invented X fallback.

## Evidence and verification

Original B executable SHA256:
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
`check-door-stairs.asm` is a direct original-image disassembly. The exact inspected
function range and its SHA are in `door-stairs-native-manifest.json`.

`door-stairs-native.py` runs the complete native function under Unicorn. Only
identified terrain/path sampling callees, the WorldRaster accessor and allocators
are hooked. The callbacks record every actual sample coordinate and order;
allocator hooks verify original 32-byte allocation, old-box preservation and
replacement/free behavior. This verifies the business geometry independently of
the shared sampling algorithms; it is not a test of those sampler libraries.

2000 native cases include 364 successful outputs, early rejections, both coverage
branches, varied yaw and full 3D quaternions, exact threshold and NaN branches.
The standalone Rust runner directly includes the actual production geometry
source and reuses the unmodified glam 0.29.3 rlib produced by the toolbar worker.
It compares every output float bit and every sample-coordinate bit/order:
**2000 cases, 0 mismatches**. See `door-stairs-actual-source-result.txt`.

The independent interaction worker added **192 targeted IEEE cases**, including
NaN in all four terrain-return lanes, signed zero/infinities in scalar inputs,
and non-unit/non-finite quaternion inputs. After review found that its initial
script varied `case['q']` but executed an older local `q`, the worker corrected
the input identity and regenerated/re-ran both native fixtures and the actual
production-source Rust runner. Final receipt: **192 cases, 0 mismatches**.
The final fixture SHA is
`46e39f0e7ea661d159c6494b940d0c29d9b149296d78eb3ad8d985365b006568`;
the checked production geometry SHA remains
`1223e698cd8b9e462fa201132f0c607f619bd89f819f89223c74f1757edaedf0`.
See sibling `decorator-interaction/door-stairs-independent-result.txt` and
`actual-source-manifest.json`. Those hashes were independently checked against
the current files. The original SSE NaN-mask/MINSS sequence agrees with the
current ordered Rust min calls on these cases; no production change was needed.
This is bounded IEEE case coverage, not an exhaustive proof for all float bits.

No Cargo was run by this worker. The full five-owner adapter and authored tests
still require central crate compilation/execution. No real GPU, full visual
generation, scene replay, or interactive game acceptance is claimed.

## Remaining gaps observed

The full Gothic/Cottage door producer, broader dormant visual producers, lantern
SSBO generation/update, and dormer collision generator remain unclosed. Existing
lantern CPU code is not reliable enough to mount: it substitutes coverage/time,
zeros positions, only visits modified entries, and loses equal-ID updates; the
native compare callbacks allocate/reuse GPU IDs and compare float lanes. This
batch retains the direct disassemblies for follow-up without inventing missing
owners or promoting the current stub to a complete system.
