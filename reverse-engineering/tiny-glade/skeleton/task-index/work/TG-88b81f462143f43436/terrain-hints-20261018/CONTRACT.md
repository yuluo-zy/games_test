# B terrain-focus producer recovery

Exclusive task TG-88b81f462143f43436, owner terrain-hints-20261018. Only work-directory candidates were written; root applies shared registration and production changes. Target B release RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 and matching B PDB. No C address is queried. Existing hand-written types, helpers, UI registry, input, camera, collider query and mode consumer are reused; no parallel selection architecture or native ABI recreation is introduced.

## Original business evidence

ui_edit_focus0x1409771f0 has PDB code_size3411 and final SystemParam tuple(EventWriter<NotifyHintSystemTerrain>,Local<TerrainFocusModeOrigin>). The original entry's action tables and raw bytes are pinned in original-bytes.json. Exact full PDB signatures and original assembly are retained beside the candidate.

For AppMode::Terrain(native10), right_click digital slot1 JustPressed or Repeating timer<=0 and hover low bit set enter edit_focus_canvas. With no current UiNav focus, an active ui_cancel slot18 preempts it. A current nav focus bypasses that cancellation check. Widget volume tag5/size.x=f32MAX, empty sounds, occlude_by_world=false are written before the collision query. The real query is RaycastWorld.raycast_simple with mask0x3f43 and maxTOI f32MAX, using CameraCursorRay.

A native miss sets geometry distance to f32MAX; a hit computes sqrt((dy²+dx²)+dz²), retaining that f32 order. CursorTerrainRaycast::position returns ZERO if its tag low bit is clear, so that state does not imply rejection. Only geometry_distance>terrain_distance and HoveredTerrainStrokeId.tag==1 select the terrain stroke. Equal and unordered comparisons fail. Native target id reads the low u32 of existing id. Original0x140977ce6..d75 writes RequestAppModeChange(Editing(Terrain(id))) first and then Terrain notification byte2; it is not derived from later mode success.

## Bounded implementation

candidate.patch preserves both existing conversion helpers and adds only this actual terrain-tool selection producer plus its local pure distance/guard helpers in the current ui_edit_focus module. It uses existing typed resources/events. Optional host resources defer until real owners provide them; they do not synthesize a raycast result or input. It performs the current real collider query. Existing GeoSource terrain-to-None behavior is unchanged. No fake mode-change notification is added.

registration-only.patch is a minimal shared-integration proposal: init existing RequestAppModeChange events and add the producer after terrain hover in the existing tool chain, already after InputSystemSet::Derive. Root owns that registration file and may adapt to current drift. Where a mode-manager schedule is installed, this producer must execute before its request consumer; the supplied test establishes that local order explicitly. Current startup/run_game does not install configure_mode_manager, and this patch does not claim that missing broader global schedule is repaired. The system's output remains the existing real RequestAppModeChange; root can integrate that current consumer independently.

## Original validation and regression

native_terrain_focus_producer.py executes the full original function entry for20 cases. Only UiState insertion and collision lookup are resolved dependency boundaries; original selection, action/nav/hover/cursor guards, distance math and native event writes execute. Preallocated queues avoid growth. Cases cover all5 action phases, repeating negative/zero/positive/NaN timer, no hover, nav cancellation, cursor absent, nearer/equal/farther geometry, unordered geometry/terrain and signed zero. Native request/hint event payload and order, widget params and ray mask are captured. This is original evidence, not a recovered Rust reimplementation oracle.

The Rust candidate contains all20 raw-bit cases and an ECS regression using the real empty RaycastWorld, current UiState/MainActionSet resources, actual producer, RequestAppModeChange and existing mode_manager consumer. It asserts focused terrain17, hint2, original widget params, and no repeated notification on a held already-focused frame. These Rust tests have not been compiled or executed by this worker; root performs unified verification. git apply --check passes against the recorded source base.

## Remaining boundaries

This is the original Terrain-tool-to-terrain-selection endpoint, not the complete3411-byte ui_edit_focus. Other tool/editing subject selection, wall validation, right-click auto clutter, canvas click behavior, cancel outputs and origin-based exit remain outside this bounded producer recovery. No new Local<TerrainFocusModeOrigin> is needed on this entry-only path: original origin is always set true before it and affects only the excluded exit paths. The existing helpers remain reusable for those future original branches. Native collision leaf/GPU/frame provider correctness and complete renderer/global scheduling are not newly accepted here. All earlier gap observations remain historical rather than an assertion that the new bounded path is absent.
