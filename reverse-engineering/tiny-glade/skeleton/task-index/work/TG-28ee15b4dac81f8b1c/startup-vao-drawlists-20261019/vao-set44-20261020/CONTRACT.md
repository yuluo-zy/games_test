# VAO set44 prefix: bounded B contract and current-host design

## Evidence and scope

Target facts are B, not C equivalence. Original RNE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 was freshly checked before slicing only two PDB-bounded functions. PDB identity is 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Module481 records19212/23672 give build_missing_vaos@1408646b0 length1331 and rebuild_vaos@140864bf0 length1115. Both full ranges decoded without gaps; byte hashes and all calls are in evidence/native-manifest.json. Original decomp in evidence/country-core-deep/backend-misc is checked against these full ranges; the old vao-original.result.json is ok=false and is not used as evidence.

The existing root-delegated backend44-registration-pdb.json retains original helper1415f5310 full type/name and outer_tuple. Its first two entries prove:

1. Query<(Entity, &Handle<Mesh>), Without<Handle<Vao>>>, Res<AssetMeshLibrary>, Res<GraphicsDeviceResource>, ResMut<AssetVAOLibrary>, Commands.
2. ResMut<AssetMeshLibrary>, ResMut<AssetVAOLibrary>, Res<GraphicsDeviceResource>.

The full tuple is build_missing → rebuild → maintain → explicit ApplyDeferred → original update tuple → flush. 04 owns Nani systems and gameplay_startup; this candidate does not write those files.

## Observed original algorithm

Build missing:
- Iterate only entities without Handle<Vao>, even if an existing component is stale or points to another mesh. Do not repair or replace an already-present component in this system.
- AssetMeshLibrary::get then Option::is_some_and checks empty indices: existing empty mesh logs trace "Mesh is empty. Skip building VAO" and is skipped. A missing CPU mesh still enters AssetVAOLibrary::add, which supplies the actual error; do not silently skip it.
- Successful add queues the real Handle<Vao> insertion through Commands. Add failure logs at warn level "Couldn't build VAO: " plus the actual error, without publishing a handle. Next frame's Without query naturally retries. No empty-mesh VAO removal or entity component deletion occurs here.

Rebuild:
- Empty modified vector returns without marking the mesh resource changed.
- Process modified_meshes in its current order, including duplicates. A mesh with no VAO logs the original warning at warn level and is dropped from the retry list.
- For a mesh with a VAO, call the current canonical library rebuild_vao. Successful entries leave the queue. Failed entries are pushed to a new queue in the same order, including duplicate failures, and log at warn level "Couldn't rebuild VAO: ". Replace the original vector with exactly that failure queue. Do not sort, deduplicate, pre-skip missing CPU meshes, or add current-entity scans.
- This system has no Query/Commands, so it does not delete stale Handle<Vao> components. Empty-index behavior belongs to the existing AssetVAOLibrary::rebuild_vao implementation, which remains unchanged.

Native anchors: build add@140864961, empty path@140864a9e onward, command insertion payload@140864a40..4a8a; rebuild no-VAO@140864d60, rebuild call@140864e2f, failure push@e3d..e6a, failure-vector replacement/change tick@f2e..f76. Log formatter exact Display-vs-Debug is not yet separately named; message/level/actual error and retry semantics are proven, not a claim of byte-identical formatting.

## Current architecture adaptation

Expose runtime::{build_missing_vaos, rebuild_vaos}. Their data/filter/access modes match the observed PDB; device access uses the existing required Res<AssetLibraryRuntime> and its same canonical buffer-device Arc. Do not construct a new graphics device or parallel VAO library. Required production resources must not silently no-op.

Split current runtime registration into:
- add_asset_library_post_update_systems: existing reclaim, reload and geometry systems stay in PostUpdate and retain their relative order. No VAO update in that phase.
- Existing add_asset_library_systems remains an explicit single-schedule compatibility entry for standalone fixtures; it composes the original two VAO systems in the former Rebuild slot. Its old optional resource-availability behavior is retained by registration run_if guards, not by weakening the required production systems. Production no longer calls this combined entry.

GameLoop has one idempotent per-App installer used by GameFrameDriver::new and before moving the gameplay SubApp to preparation. It installs the post-update subset plus `(build_missing_vaos, rebuild_vaos).chain_ignore_deferred().before_ignore_deferred(nani_maintain_instances_ecs).in_set(GameSystemSet(44))` in Update. Both independent loading and gameplay Apps own their own one-time registration. No changes to reclaim/reload/geometry phases or their algorithms.

The exact ApplyDeferred in 04's Nani chain flushes both the prefix VAO insertions and maintain's instance-key commands. No automatic flush is introduced between the two VAO systems or before maintain. No prefix registration is added to 04's gameplay_startup helper, preventing duplicate execution.

## Input/output/error/lifecycle and performance

Input: current ECS mesh-handle components, canonical CPU mesh library, its modified queue, same device and VAO library. Output: actual uploaded canonical VAOs, deferred component insertion, and failure-only retry queue. Consumers: same-frame set44 maintain/explicit deferred application/update/flush, then existing draw-list and physical renderer tables. Deleted CPU meshes still go through the existing PostUpdate reclaim owner. This prefix does not move the deletion/hot-reload owners or claim their complete timing equivalence. PostUpdate reload_meshes now appends modifications after the current Update44 VAO/Nani prefix; those changes are rebuilt on the next Update44 rather than the old combined helper's same PostUpdate. PostUpdate reclaim likewise now runs after this frame's Update44 VAO/Nani work. These explicit current-host latency/relative-order consequences remain outside a full original asset-maintenance phase recovery and must be tested, not hidden by moving reload/reclaim without evidence.

Build failures leave no entity VAO component; empty meshes are retried by the missing-component query until usable, without allocating. Rebuild failures remain queued exactly; missing-VAO entries are consumed with a diagnostic, not treated as successful rebuilds. Missing required production resources cause ordinary Bevy validation failure. No new success flag, default VAO or fake producer is introduced.

No per-frame full scan of entities already carrying VAOs. Rebuild avoids the current sort/dedup pass and processes only actual queue entries; duplicate side effects are preserved as required by the original. A new failure Vec is the observed original allocation behavior, not a claimed optimization. Registration guard is checked only at App construction. No CPU/GPU performance or original differential test has been run by this owner.

## Verification plan for root

CPU schedule regression using current AssetVAOLibrary and a recording physical buffer device: (1) same-frame missing entity receives its real handle only at explicit ApplyDeferred after maintain; maintain runs before that flush and the subsequent update consumer sees the handle, (2) existing/stale component is excluded, empty mesh skipped, missing CPU mesh not published, (3) duplicate rebuilds preserve order and failure queue duplicates, missing-VAO entry is dropped, successful retry clears queue without changing identity, (4) registration twice on same App does not execute prefix twice, separate loading/gameplay Apps each register once, (5) PostUpdate contains no VAO update in production installer and reclaim/reload/geometry remain in place. Existing standalone asset/geometry/loaded-logo fixtures remain compatible; any assertion encoding former all-entity repair behavior must be reconsidered against this evidence.

Root owns Cargo/GPU. Tests and actual nonempty VAO→Nani→draw/present runtime are pending; static schedule consistency alone does not prove gameplay output.

Native log levels are separately verified in the full instructions: message records use2 (Warn) for add/rebuild/missing-VAO diagnostics and5 (Trace) for existing empty meshes; target string length0x2a is the original backend::vao_update target. No operational branch is inferred from the word WARNING alone.

Loading-App boundary: no Nani maintain/barrier may exist in that App; its queued VAO insertions must be applied by the real Update schedule completion before PostUpdate/render. The generic driver's required resource preconditions and per-App installation marker need actual CPU schedule tests; gameplay's explicit shared barrier is a different scoped fact.
