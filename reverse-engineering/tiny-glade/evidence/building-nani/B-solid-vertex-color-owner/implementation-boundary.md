# SolidVertexColor CPU consumer — implementation, unverified

Private owner: country_core/render/nani/systems.rs, B 0x142070ac0.
Original writer call: 0x14207124e → 0x14156de90. Read-only static export is in this directory. The Ghidra database is patched and is not the original execution oracle.

Input Transform48: quaternion0..16, translation16..28, scale28..40. Writer record64: scaled Mat3 columns36 bytes, translation12 bytes, two boolean words at48/52, eight zero tail bytes. Bounds center is translation; radius is max(abs(scale.z), abs(scale.y), abs(scale.x)). glam owns quaternion/matrix operations; GPU/container/Bevy algorithms are not reconstructed.

New core: reconstruction/building/nani_solid_vertex_color.rs. Runtime uses a separate typed SolidVertexColor bundle, atlas snapshot and create/update/remove changes. The two flags require actual observed component/query outputs; absence remains pending. The gate's unrelated observed bundle word must not be used as either flag. Full original Query/UiSignifier policy and actual GPU allocation/upload/removal remain open.

Per current user instruction, no build, probe, oracle or runtime validation was run for this new implementation. Existing floor payload proofs do not accept this solid payload. CPU changes are not GPU activation confirmations.

## Subsequent field-owner recovery

component-query-types.json records the original FunctionSystem Query: ActiveInstance, Transform, Option<SemiTransparent>, Option<FlatLighting>, With<SolidVertexColorMarker>, Res<GpuWorld>. Native Query fetch/writer uses SemiTransparent presence for word48 and FlatLighting presence for word52. Runtime now queries actual optional component presence, replacing the initial explicit unresolved flag DTO. No NaniActiveInstance is manufactured; CPU staging remains pre-backend. Marker types project presence only, not unobserved payload/layout or original full component registration. No validation executed.
