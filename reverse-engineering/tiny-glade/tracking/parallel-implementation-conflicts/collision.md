# Parallel collision implementation handoff

Status: implemented, not compiled/tested/native-verified after the user's implementation-first instruction. Frozen existing backend/hash/policy files were not modified.

Owned new source:
- `reconstruction/building/dirty_tiles.rs`: explicit existing64x64 row-mask authority; B fmaf scale0x3efc0fc1/bias32, floor minimum/ceil end, NaN→0, clamp63, exclusive end, OR marks preserving previous bits. Constants were read statically from original B PE sections, not executed.
- `reconstruction/crates/tg-building-runtime/src/wall_platform_collision.rs`: `pub fn register(app: &mut bevy_app::App)`; parent `BuildingStage::PlatformCollision` after PlatformStairs/ApplyDeferred, before WallStructures. Actual CPU generation→existing lockedRapier backend insert/Keep/replace/remove, original-order ColliderDiff messages, real old/new AABB→new raster, query-pipeline flush then platform subworld7 query. Stale/pending/Delete cleanup included.

Parent shared edits required:
1. Export root alias `building_dirty_tiles` at `../building/dirty_tiles.rs`.
2. Runtime `pub mod wall_platform_collision;` and call its `register(app)` from parent plugin. Existing backend alias `building_wall_platform_rapier_backend` already exported.
3. Use parent BuildingStage::PlatformCollision in the chain Stairs→PlatformCollision→WallStructures. The owned module now registers directly in that stage.
4. Supply `SetPlatformCollisionDirtyTiles { rows: [u64;64] }` from authoritative initialized/reset mask state. Original DirtyTiles constructor/reset is not yet identified; no automatic zero-mask default is fabricated. The real raster algorithm is implemented, but an explicit mask authority is still required.

Live API:
- `WallPlatformCollision`: actual world/backend, shared raster state, pending reasons and changed ownership handles.
- `SetPlatformCollisionDirtyTiles`, `ClearPlatformCollisionDirtyTiles`: explicit mask owner lifecycle.
- `QueryPlatformRay`→`PlatformRayResult`: real Rapier subworld7 hit/miss; unresolved authority/geometry/invariants yields Pending instead of a fabricated miss.
- `PlatformColliderChanged`: ordered backend Upsert/Removed diffs.
- Missing mask clears existing physical colliders and retains their actual AABBs until authority is supplied; next initialized mask consumes queued rectangles. Roof/steps/CPU disappearance removes old collider and generational mesh keys. RemoveWall has an independent event reader.

Conflicts: no concurrent modification of owned files observed. Parent must not insert a zero mask solely to avoid pending. Current queue is actual backend AABB output, not resolved blocker/collision fixture input. Original17-subworld filters and exact original ECS scheduler remain outside this implemented platform adapter scope.
