# Window consumer: canonical AutoClutter schema and owner proposal V1

Status: original-B bounded evidence and design request; no consumer source written, no CPU/Cargo/GPU run. Root alone integrates production. This does not supersede the frozen Window core schema or claim native source text / numeric TypeId equality.

## Scope and authority

The authorized consumer files are on_windows (TG-1d83d856c7a8e81b88), autoclutter/mod (TG-1c1af6eb0d33100902), and manager (TG-6aae423fc55bc9aaad), owner window-candidates-20261020. Components/mod.rs remains foreign TG-abf1305389c44cc264, meihost-20261018. No write to it is proposed. Root offered its existing legitimate drawable.rs claim as a possible canonical owner; the architectural mapping below is a design decision for independent review, not a lock workaround.

## B identity and bounded ranges

PE SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; PDB SHA256 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. evidence/native-pack-v3/report.json retains full continuous disassembly and matching PDB/byte checks for the ten requested ranges. In particular: manager 1422d04e0/10301B, animation 1422ccef0/2261B, AutoClutter registration 140857300/1776B, component command 1422cb440/266B, orientation 1422d2d20/640B. Old manager 8178B and animation 2238B comments are not used as complete bounds.

PDB generic command symbols identify country_core::components::AutoClutter and the actual NaniBundle and ColorId insertions. This establishes original logical types; recovered source placement remains a separate mapping.

## AutoClutter: complete value and ownership

Registration 1408575c9..5f1 has alignment 8, size 0x18, drop=None. Constructor is the queued value written at manager 1422d1b65..1bad and moved by callback 1422cb440. After the command header/entity prefix, the value is:

| Native offset | Value | Proposed semantic field |
| --- | --- | --- |
| +00 | full output stable key, u64 | key: u64 |
| +08 | random delay, f32 | timer: f32 |
| +0c | 0.0 at creation; animation progress | progress: f32 |
| +10 | output +60 target scale, f32 | target_scale: f32 |
| +14 | selected ClutterMesh native_group byte | native_group: u8 |
| +15 | source-selector predicate byte | native_flag_15: bool |

The +15 writer is `(selector.wrapping_sub(3) >= 4) || (selector.wrapping_sub(3) == 1)`. Its broad business name is not recovered. Native tail padding is not semantic. There is no Arc, asset handle, pointer, allocation, or custom Drop in this 24-byte component. The recovered Rust value need not promise native ABI/field offsets; a source-only component can derive Component, Clone, Copy, Debug. No fabricated Default is needed.

For this Window route selector is exactly 0, 1, or 2. Its delay expression is `(1-r)*0.0 + r*0.1`, seeded by init_rng(GameTime.frame_count), not a model-scale jitter. target_scale=1.0 is from the on_windows original writer. Native group is from the actual selected mesh, not guessed by candidate kind.

## Real consumers and lifecycle

PDB generic hosts with direct calls bind manager to:

`Res<AutoClutterState>, Query<(Entity, &AutoClutter, Option<&mut WindowShutter>, &mut Transform), Without<ClutterMarkedForDeletion>>, Res<GameTime>, Commands, Query<&Transform, (With<Sheep>, Without<AutoClutter>)>`.

The manager's map is keyed by full u64 output key. Existing matching entities update translation and either actual Transform.rotation or WindowShutter.base_rotation; current animated Transform.scale is retained. Absent keys get canonical ClutterMarkedForDeletion. New entities receive the actual AutoClutter value and NaniBundle with canonical core Transform, Named(mesh.name_hash), and real metadata-selected instance marker. Creation scale is Vec3::ZERO, because the real animation writes it afterward. It must not be changed to ONE to bypass an unimplemented animation.

PDB direct-call hosts bind animation to:

`Query<(Entity, &mut AutoClutter, &mut Transform, Option<&WindowShutter>, Option<&ClutterMarkedForDeletion>)>, Commands, Res<GameTime>, EventWriter<PlayClutterSoundCmd>`.

The full original body consumes timer/progress/target_scale/native_group/native_flag_15, writes actual Transform.scale and shutter rotation, sends the existing core PlayClutterSoundCmd when timer crosses zero, and despawns after deletion progress reaches zero. The current local AutoClutterAnimation substitute is a different ECS type and does not write Transform; it cannot be counted as this consumer. Animation source is outside the three current claims and needs an explicit root owner continuation before implementation. Its whole numeric easing/rotation algorithm will be frozen separately before source change.

The canonical public event already exists at country_core::systems::audio::PlayClutterSoundCmd. The canonical Transform is country_core::utils::components::transform::Transform; no second transform or animation-only shadow will be introduced.

## WindowShutter and ColorId

Original WindowShutter writer/callback preserves Quat base rotation (+00..0f) and selector side byte (+10). Proposed canonical clutter-local value is WindowShutter { base_rotation: Quat, side: bool }, placed in autoclutter/mod.rs, the original logical namespace, with all consumers importing this one type. The old manager-local {kind:u8,value:u64} is not a valid equivalent.

On_windows kind2 has selector `!candidate.side & 1`; thus the stored selector-side bool is the resulting selector 0/1, not an unexamined copy of the input side. Candidate native_u32_04 is passed to actual ColorId insertion; all 32 bits must survive. Preserve the already integrated Window candidate API spelling pending any separate versioned semantic rename.

## Convertable ownership: excluded by the actual Window gate

The original manager inserts AutoClutterConvertableToDecoClutter only if `(selector & 6) != 4 && selector > 2`. Window selectors 0/1/2 always fail this gate. Therefore this component is not required to close this Window route.

Its registration proves size 96/alignment8 and a nontrivial Drop function 14014b3b0; the broader source payload has ownership, including a tag4 Arc case. We do not manufacture an opaque 96-byte value, duplicate marker, default, or partial guessed struct. A full non-window reconstruction requires a separate constructor/drop/consumer contract. Current Window work explicitly does not claim that broader route.

## Recovered architecture proposal for independent decision

Current core components/mod.rs describes shared ECS data and declares/reexports drawable, rendering, trees, ui. drawable.rs already owns canonical NaniBundle, NaniInstanceDesc reexport, Transform-bearing drawable bundles, PrevTransform, AlbedoMap, AlphaMap, and NaniSolidTexture. AutoClutter is a value attached to Nani entities and consumed with the same Transform by animation and manager; it has no physics registry or resource-owner role. Placing its single authoritative recovered definition in existing components::drawable is therefore consistent with current component organization.

Proposal: original logical country_core::components::AutoClutter maps to recovered country_core::components::drawable::AutoClutter; root's existing drawable claim adds the one definition, and all actual consumers import it directly. No components/mod.rs reexport is required for this bounded route; no declaration is added to clutter or a second core module. This is a documented recovered organization, not a claim that original source declared it in drawable.rs, nor that Rust runtime TypeId numerics match B.

Independent reviewer should decide whether this placement is semantically justified. If rejected, root must obtain components/mod.rs handoff; code must not create a duplicate component to evade ownership. WindowShutter remains system_clutter::autoclutter's unique type.

## Reset and scheduling boundary

Full physical hosts 140ee5830/833B and 141ce1f10/961B have PDB inline attribution to reset_autoclutter_for_new_frame and actual 3-resource signature: GameTime, mutable AutoClutterState, mutable InterShapeBrickStitches. First host 140ee594b..59b1 clears the 0x68 output Vec and drops owned tag4 payloads; 59b6..59bb writes the low frame_count byte into epoch; 59c7 clears the stitches Vec length. Current InterShapeBrickStitches is the actual tuple Vec resource. No clear of canonical WindowAutoClutterCandidates occurs here.

Root registration must order reset before output producers, blueprint collector before on_windows drain, and manager before animation with deferred insertions visible where required. This note does not assert the current registration has that order. Same-Arc collector still appends every eligible current blueprint before rebuild/fullEq gates; drain once must be followed by next-run same-Arc recollection.

## Acceptance and remaining boundaries

Required future tests: full key and ColorId bits, same actual component identity across manager/animation; drain empties input and appends real mesh output; same-Arc collect→drain→collect repeats; actual NaniBundle marker/Named mesh desc/Transform ZERO creation; timer writes scale later; existing entity update preserves animated scale; missing key marks deletion; sheep minimum distance and NaN rejection. Tests are not run here.

Unknown original tag0 blueprint writer, full non-window Convertable payload, other legacy producer metadata, actual OS Edit/Reload behavior, and GPU rendering remain open. This schema and architecture review alone is not implementation or runtime closure.
