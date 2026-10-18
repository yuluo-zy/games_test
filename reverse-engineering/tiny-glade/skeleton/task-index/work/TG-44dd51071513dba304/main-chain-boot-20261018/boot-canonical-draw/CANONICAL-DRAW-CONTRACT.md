# Startup / clearing draw consumers — original B, canonical ECS owners

This is an uncompiled candidate package. No production source, Cargo, or GPU execution was performed by this worker. `canonical-draw-candidate-manifest.json` identifies all five before/candidate/patch hashes and the current-source checks. Apply the complete `drawable.rs` patch from this manifest; the earlier maps-only patch is superseded.

## Actual inputs and identities

| Consumer | Actual owner / field | Original meaning |
| --- | --- | --- |
| ivy leaf/flower; placement eye; tree trunk | `MainCamera::eye_position()` | CameraMatrices inverse-view translation, not a synthetic eye Resource |
| snow callback A env3 | `Weather.snow_amount`, `AppMode::native_tag()`, `MainCamera::mode_byte()` | Weather +0xc, mode tag 0x13 and camera byte +0x4d0 |
| tree trunk / canopy | `Weather.wind_strength`, canonical `TreeCanopyMaxCullDistance.smoothstep` | Weather +4; canopy +4, not the progress scalar at +0 |
| tree canopy lens | `MainCamera` body selected by its mode, then f32 +0x104 | Original six-entry table at 0x142ac0cf0: 8c/144/1c4/264/31c/3a4 |
| glade and flag/water mutator predicates | `ActiveSession.meta.glade` / `.mutators` | The Vec at +8 is SessionMeta mutators, not SessionId bytes |
| rocky terrain | `RockyTerrainMeshVertCount.0 as u32` | Original callback truncates the actual native-size count to its draw u32 |
| three per-entity texture names | `components::drawable::{AlbedoMap,AlphaMap,NormalMap}` | One canonical ECS Component per original type; stored u64 texture name |
| sheep/duck previous transform | `components::drawable::PrevTransform(pub crate::utils::components::transform::Transform)` | Original complete Transform wrapper, same identity in both callback halves |
| flag cloth | canonical `RoofFlagCloth` and `WallFlagCloth` | Complete observed payloads, same aliases across A/B |

The game-owned Transform in `country-core/src/utils/components/transform.rs` deliberately retains `rotation/translation/scale` and bridges to the shared utils Transform (`r/t/s`). The candidate uses that existing architecture; it does not replace either type or confuse their field APIs.

Historical pure-body DTO names such as `season_param` remain source-compatible, but their production inputs are actual CameraMode/lens values. Camera mode 3 is Walk. The six-entry table is retained exactly: its following bytes are string data, so extending it to the thirteen camera transition tags would invent native values. Safe Rust bounds failure for a mode outside the original six bodies is an explicit limitation, not an additional original branch.

SessionMutator has precisely the three original discriminants: Halloween2024=0, Halloween2025=1, Xmas2024=2. Existing callback pure-body functions still receive an owned byte sequence, populated only from this typed vector. No ECS mirror Resource, raw cast, UUID reinterpretation, or fallback session is introduced. Empty vectors preserve the original vacuous all-nonzero predicate.

## Native proof and order

The package's `native-manifest.json` binds the complete decoded original-RNE bodies and SHA256s for tree trunk 0x1409c9830/2841B, tree canopy 0x1409ca350/2400B, water planes 0x1409d9bc0/2487B, and the cloth env21 0x1409b8680/2467B. `canopy-lens-table.json` retains the original table bytes.

Tree canopy reads camera mode and the selected lens at 0x1409ca41a..42d before its Weather lookup at 0x1409ca43e. The adapter computes those locals in that order. Water's global-disabled branch discards before any ActiveSession lookup; the candidate restores this early branch instead of demanding a session for a discarded draw.

The drawable-family `component-clone-native/native-manifest.json` binds complete original custom clone functions: PrevTransform 0x1412efb60/97B, WallFlagCloth 0x1412efe10/109B, RoofFlagCloth 0x1412f0080/58B. They establish the full valid payload copies: Transform 3x16B; wall i32 + three f32 + byte gate; roof u32. Wall padding has no invented field meaning. Current Bevy 0.16 Component derive selects Custom when Clone exists, Default otherwise. Thus the three texture map Components deliberately have no Clone/Copy derive, matching their original registry Default behavior; the three original Custom components derive Clone.

## Integration and limits

1. Apply the prior Forest resource patch for canonical `TreeCanopyMaxCullDistance { progress, smoothstep }` (`TG-ab197f381ed7615ce6/boot-leaf-20261018`) before compiling these callbacks.
2. Apply all five patches from this manifest. Legacy View names for the canonical components are reexports, preserving the same TypeId. Historical resource DTOs are test-only and do not derive Resource. Relevant tests now build actual MainCamera, Weather, AppMode and ActiveSession owners.
3. Run the bounded callback and drawable tests centrally, then actual startup/clearing rendering. These candidates have only formatting and `git apply --check` validation so far.

This is not a claim that every draw input is recovered. Other critter, roof-highlight, editing and UI schemas remain outside this bounded package; no default `*_View` resources are inserted to conceal them. Complete session clearing/forest producer integration has its separate handoff and foreign prefab-event dependency.
