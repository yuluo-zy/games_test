# DirtyTiles implementation and live collision boundary

Status: implementation-first draft. No cargo build, probe, oracle or test was run for this new core/runtime after the user asked to pause validation. Earlier backend/query proofs do not validate this new raster or Bevy wiring.

## Original helper schema

B `140960ae0` receives `(RCX=DirtyTiles rows, RDX=world rect)` and swaps its arguments before tail-calling `140ade8e0`. The latter receives `(RCX=rect pointer, RDX=rows pointer)`. The rectangle reads are four f32 values at offsets0/4/8/12: minimum X/Z and maximum X/Z. The destination is64 existing u64 row words, totaling512 writable bytes; row Z stores X bits.

At140ade954/968 and140adea4c/5f the original uses CRT fmaf for each coordinate. Static reads from original B's PE section, distinct from the patched analysis database, give scale bits0x3efc0fc1 (0.4923076927661896), bias32 and clamp63. Minima use floor, maxima use ceil. The SIMD ordered mask maps NaN to0, followed by clamping to0..63 and saturating casts. The strict min<end gate requires both dimensions; endpoints are exclusive. Existing bits are preserved by OR. The clipped coordinate63 is therefore an exclusive limit; do not replace it with64 based on a guessed grid extent. std fused arithmetic/rounding and primitive arrays are reused.

`building/dirty_tiles.rs` exposes `DirtyTiles64::from_rows([u64;64])`, `rows()`, `world_rect_cells(rect)` and `rasterize_world_rect(rect)`. It does not reconstruct private fields outside the512-byte row projection or reset/consumption policy. A/B/C owner provenance is recorded in `B-dirty-tiles-implementation-status.json`; source-path/checksum evidence does not imply identical behavior across releases.

## Authoritative initial/reset rows

Neither exported helper initializes or clears the512 bytes. The original DirtyTiles constructor/reset owner has not yet been identified. `SetPlatformCollisionDirtyTiles { rows }` therefore supplies the actual preexisting mask projection. No automatic zero mask is invented. `ClearPlatformCollisionDirtyTiles` removes authority, clears existing physical colliders through their real removal path, and queues their dependency-produced AABBs. A later explicit rows input consumes those queued rectangles with the real new raster algorithm.

## Actual live collision consumer

`runtime/src/wall_platform_collision.rs` registers in parent `BuildingStage::PlatformCollision`: PlatformStairs, its ApplyDeferred, then collision, then WallStructures. It consumes actual `WallCpuPlatformStairs.geometry` and CPU generations. Source disappearance, stairs pending, missing raster authority and independent RemoveWall messages remove stale real physics objects. The actual frozen Rapier backend supplies indexed sensor shapes, generational collider/mesh keys, AABBs and ordered Upsert/Removed diffs. The callback now calls the actual DirtyTiles implementation when authority is present; it is not only a recorded rectangle notice.

The runtime flushes the actual isolated-subworld7 QueryPipeline after ownership changes and then answers `QueryPlatformRay`. Pending source/authority/invariant states produce `PlatformRayOutcome::Pending`, so an incomplete scene does not fabricate an authoritative miss. Collider changes are emitted with the backend's real handles and original0/1 diff ordering.

## Remaining downstream boundaries

The current actual isolated-subworld query flush uses the frozen backend's dirty marker and dependency QueryPipeline.update. It does not consume the64-row bitmap to emulate the original17-subworld mesh-intersection subscriber pipeline. The bitmap is available in `WallPlatformCollision.raster.tiles`; original dirty-mask clear/drain, actual subscribers and cross-subworld filters remain separate owners. Parent has been asked to export original `RaycastWorld::flush_colliders1408cd5c0` and Rapier query selectors to trace those readers. Queueing/raster-writing and real subworld7 queries are implemented; that does not establish the remaining original consumers or schedule.

Required shared registration and conflict handoff are in `tracking/parallel-implementation-conflicts/collision.md`. There have been no conflicts on the owned files so far.
