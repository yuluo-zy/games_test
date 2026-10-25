# G06 parallel implementation handoff — unverified

Owned runtime file: reconstruction/crates/tg-building-runtime/src/gates.rs. No cargo/probe/oracle/test execution after the user's implementation-first instruction.

Parent wiring required: pub mod gates; gates::register(app); BuildingStage::Gates after WallArches before generic Nani CPU staging. building_gates core alias is already parent-owned. No Cargo or shared lib/nani/CLI modifications by this worker.

ABI correction: the old core GateNaniProjection.observed_word_30 is the **NaniInstanceMeshDesc enum discriminator 0 = Named**, at bundle+48 (command+56). It is not a separate Solid component and not either GPU flags word. Native bundle is ObjectTransform48 followed by NaniInstanceDesc20: quaternion@0, translation@16, scale@28, padding@40..48; descriptor tag@48, Named definition u64@56, subset@64. Runtime writes only the meaningful first40 transform bytes and uses Named(definition_id)/subset3 + real SolidVertexColorInstanceDataMarker. Solid upload flags require their separate real query/signifier owner; runtime does not default them.

CPU mesh handles are explicit GateCpuMeshHandle, never GPU IDs. Actual position/normal/color/UV/index buffers live in GateCpuMeshAssets and are removed with owner entity removal. Shader choice is the original key; no shader handle or GPU SSBO slot is invented. Separate support entities mirror the CPU boundary before the native SSBO pool, and are not added to the native Gates door/post map.

Lifecycle maps pending reconciliation: Runtime RoofGeometryChanged is the available generalized equivalent for original SwitchRoofType / subsequent edits. Original rebuild events preserve empty entries; WallDeleted/DeleteRoof remove entries. Source/path/pending changes additionally withdraw old CPU outputs. Parent's original event mapping and scheduling should be reviewed at final reconciliation.

2026-10-11 follow-up: Parent removed the incorrect standalone word30 component and supplies actual SolidVertexColor bundle descriptor/transform/marker. Core documentation now names the old compatibility word as the Named discriminator and Raw69 publisher binding. This documentation change alters source hashes; historical CPU reports retain earlier-stage scope, and no current proof was rerun under the user's no-validation instruction. Runtime Gate construction dispatch already covers the negativeY>.5 interior-post branch. Failed per-wall command batches withdraw earlier ordinal outputs and block the remaining ordinals of that wall. No tests source was added by the worker after the parent's latest clarification.

## Fence continuation (implementation only)
- New `building/fences.rs` and runtime `src/fences.rs` are UNVERIFIED. Perchance 0.4.0 is reused; parent added the dependency and owns final lock resolution.
- Shared gate private Curve3 visibility was opened for fence reuse; no accepted gate algorithm was rewritten. Added sample_position preserves the original difference-based interpolation. Gate source hashes changed, old reports remain historical scope.
- Fence threshold and resampler helpers are being exported by parent; pending paths must withdraw World geometry until their actual semantics are implemented. No fixture random draws/posts/CPU records are accepted as authoritative input.

- Fence threshold/resampler gaps now implemented from B-helpers. Stable API requires actual raw Curve2, prepared XYZ + segment_values and Public284/296; no explicit seed/extent/draw inputs. Remains unverified. Parent reconciles Fences stage/core alias; no GPU pool/opaque MaterialStats bytes manufactured. GateCurve3 storage is shared while fence constructor preserves its original Z/Y/X reduction order.
