# Actual window-candidate drain: next owned scope (read-only)

Core candidate is frozen separately in core-candidate-v1. This note does not change the reviewed design or candidate manifests and does not authorize writing any consumer source. No Cargo, CPU test, GPU or target execution.

## Current source boundary

`add_autoclutter_on_windows.rs` currently consumes `Res<OnWindowInput>` by shared iteration. Its independent input holds a fabricated host metadata table, maps the original quaternion to `Vec3`, computes a reduced quaternion formula with w assumed zero, and emits `SpawnShared::Prefab("window")`. It neither drains the canonical singleton nor takes the real ClutterMeshes resource. `OnWindowInput` is declared and referenced only in this file; it is not a real producer bridge. Replacing only the system parameter or inserting an empty core resource would preserve false-success output.

`AutoClutterState.spawn_candidates` is the existing unique output Vec. Its `AutoClutterSpawn` is currently a shared struct for several native enum layouts and `SpawnShared` only supports `ArcStrong(u64)` and `Prefab(String)`. It has no typed way to retain the selected canonical ClutterMesh metadata (real mesh handle/name hash/selector) or distinguish the actual on-window source selector/payload from unrelated raw fields. The native on-window output has selected mesh pointer at +38, stable key at +40, position xyz at +48/+4c/+50, forward direction at +54/+58/+5c, scale1 at +60, and a distinct WallId in the outer spawn source at +20. These values must not be interchanged because the present struct comments call +38 a position and +40 a host id for another variant.

`manage_autoclutter_entities.rs` currently makes an entity containing only AutoClutterObject and ManagedAutoClutterPlacement; its own source explicitly leaves NaniBundle insertion unresolved. Its exhaustive `SpawnShared` match converts Prefab to integer0, so adding a real mesh enum case without adapting the actual consumer would either fail type checking or discard the data. No real renderer consumer of ManagedAutoClutterPlacement was found in current clutter/core components; the animation file only mentions a future host write in a comment. Thus a drain-to-state check is distinct from actual renderable ECS spawn acceptance.

## Legal ownership inventory

Actual claim files (all 258 current locks read; none mutated) give:

| File | Indexed task | Current claim |
| --- | --- | --- |
| clutter/src/autoclutter/add_autoclutter_on_windows.rs | TG-1d83d856c7a8e81b88 | no claim file |
| clutter/src/autoclutter/mod.rs | TG-1c1af6eb0d33100902 | no claim file |
| clutter/src/autoclutter/manage_autoclutter_entities.rs | TG-6aae423fc55bc9aaad | no claim file |
| clutter/src/lib.rs | TG-2cc8aec081d0811cbe | claimed by clutter-startup-20261020 |

Evidence for the exact lock bytes and file paths is core-candidate-v1/clutter-claim-inventory.json. Historical ownership does not prove a live author; root must perform normal claims for the first three before implementation. Registration belongs to the existing clutter-startup owner (notified), not this core source. Blueprint owner02 has separately confirmed it will use the same core types and collect before its rebuild gate.

## Smallest real drain contract

1. Restore the actual four-system-parameter surface: ResMut<AutoClutterState>, Res<UiSignifierStream>, Res<ClutterMeshes>, ResMut<canonical WindowAutoClutterCandidates>. UiSignifierStream exists at the original canonical module; the retained B body does not read this parameter and no new behavior should be invented for it.
2. Consume `.0.drain(..)` from the actual shared core resource. For every valid candidate, select the actual loaded metadata using kind: WindowFlowerpot27, flowerbed Vec[index] through original checked-get/unwrap, WindowShutter28, DoorBell51, DoorKrans52, DoorKransAutumn53. No host Vec or prefab string stand-in. Keep translation unchanged and compute direction with the full original quaternion `rotation * Vec3::Z`; do not reinterpret rotation as a position or drop w. Scale remains available for original fullEq although this output writer sets spawn scale1.
3. Retain key, WallId, mesh selection, and source selector/payload as distinct typed values in **the existing spawn_candidates owner**. For shutter the selector is `!side & 1` and native_u32_04 is preserved; other kinds use selector2/payload0. The native unused/padding lanes must not turn into equality fields. No extra pending Vec/resource/registry. A typed payload in the existing spawn enum/representation is a possible narrow adaptation, but the exact shape must be approved together with its actual managed consumer; simply adding `Mesh` and converting it to 0 is forbidden.
4. The manager must follow the actual selected metadata to existing canonical Nani/entity constructors. It must preserve the original mesh-demo omission, source/shutter/color and entity-dedup/deletion policy; the current ManagedPlacement-only branch does not supply this. Before changing that business path, bind the complete original manager range and exact existing component/bundle APIs. Do not rebuild the whole automatic clutter algorithm, change random counts, or claim an arbitrary raw shared u64 is the mesh owner.
5. Existing clutter-startup owner adds the resource initialization at real configure, reusing the original empty constructor contract. Keep the existing actual system registration once; connect order to real blueprint collector output. Do not register an independent second writer or clear candidates elsewhere. Every eligible unchanged blueprint contributes again on the next collector run; immediate repeated drain is empty.

Claim the three above as one producer→state→actual-consumer unit for this window route, and coordinate the single lib.rs hunk with its current owner. The core candidate alone is an interface prerequisite, not gameplay closure. Root may choose to split acceptance into typed drain/state and actual manager/Nani gates, but an unresolved gate must remain explicit and must not be marked successful.

## Existing native evidence and exact remaining evidence gap

- Authoritative current on-window evidence: this work's native-pack-v2/on-windows-consumer.{json,asm.txt}, complete B 1422cd7d0/1622B, identity/bytes verified. Two actual jump tables, constants and full Eq/drop/producer packs are frozen in MANIFEST_V2.json; no need to repeat them.
- Actual original initialization: native-pack-v2/clutter-plugin complete 140652050/3380B, insertion at 140652683..6ae. Canonical loaded mesh metadata and helper table are already integrated by the root/02 clutter mesh work.
- Retained older manager files: evidence/toolchain/clutter/decomp/manage_autoclutter_entities.c and disasm/manage_autoclutter_entities.json. They are useful navigation, not complete CFG proof: the JSON contains 1706 instructions ending at 1422d2661, whereas retained PDB procedure boundary is **1422d04e0 + 10301B**, ending exclusive 1422d2d1d. Current source's 8178-byte comment is stale. Bind the missing tail and verify the full direct original bytes before restoring the actual spawn route.
- B and C have equal source checksum for on_windows, but this only supports a bounded source-input fact; the manager/mod files differ across versions. This note does not claim C layout or behavior equivalence.

Proposed future checks (not executed): actual App resource identity; nonempty blueprint candidate→same resource drain→existing state payload→selected real mesh metadata; all kind routing and flowerbed bounds failure; shutter u32 high bits; WallId64 distinct from stable key; full quaternion w; immediate second drain empty; next same-Arc collect contributes again; actual entity uses matching mesh/instance metadata. No fake objects/counts/frame success may substitute for those checks.
