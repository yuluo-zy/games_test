# Window drain → canonical manager → actual animation design V1

Design only, original B. No source candidate in this delivery and no CPU/Cargo/GPU run. Core Window schema 5cda is already production-integrated by root. Actual 24B AutoClutter schema/owner is separately frozen in AUTO_CLUTTER_SCHEMA_V2.md; independent reviewer has indicated drawable placement is semantically appropriate, final schema verdict remains separate. Root alone writes core/registration/production and validates.

## Authorized source boundary and single owners

Four source families: on_windows TG-1d83d856c7a8e81b88, autoclutter/mod TG-1c1af6eb0d33100902, manager TG-6aae423fc55bc9aaad, animation TG-bafb100e12b3abd97d; same owner window-candidates-20261020. Core AutoClutter comes only from components::drawable via root's existing claim; WindowShutter comes only from clutter::autoclutter. No write to foreign components/mod.rs. Root's lib owner installs the real resource and schedule dependencies. No GPU implementation change is requested.

The current local AutoClutterObject key-only ECS component, ExistingAutoClutterKeys and fixed-seed ManageRng are not original authoritative components/resources for this manager; actual manager will no longer query/use them. The current local AutoClutterAnimation component will be removed from the actual animation path, not retained as an active mirror. Existing registrations may be removed by root's lib companion once actual imports are updated. The separate core ui_clutter::AutoClutterObject opaque 0x68 generic-keymap value is not the 24B component and will not be renamed or duplicated here.

## Actual input and typed output adapter

Replace OnWindowInput/OnWindowHost/OnWindowCandidate (currently have no external producer) with the already integrated collision::WindowAutoClutterCandidates. System parameters become mutable AutoClutterState, immutable canonical ClutterMeshes, mutable WindowAutoClutterCandidates. Iterate drain(..); the input length is zero while iterating and the allocation can be reused. All six enum variants are legal, no synthetic native sentinel/tag6 enum is added. Bounds-invalid flowerbed index retains checked indexing panic as original.

Resolve mesh from the actual loaded ClutterMeshes resource: WindowFlowerpot, indexed window_flowerbeds, WindowShutter, DoorBell, DoorKrans, DoorKransAutumn. Neither a successful empty/default ClutterMeshes nor fake prefab string is supplied. Candidate rotation is the full Quat, direction=rotation*Vec3::Z, translation is candidate.translation, key is full candidate.key, wall is full candidate.wall, target_scale=1.0. The candidate scale vector is present in equality/schema but this original consumer does not use it for spawn target_scale. No xzy swizzle, truncated key, forced w=0, or default ColorId.

The current legacy AutoClutterSpawn lacks a typed metadata field. To avoid editing all other producer file families merely for an added struct field, extend its existing Rust-only SpawnShared enum with a Mesh value variant containing a small resolved metadata snapshot: actual name_hash, radius_xz, native_group, optional NaniSolidTexture, plus optional ColorId for the shutter route. The snapshot is copied directly from the actual selected ClutterMesh, not a second resource/library or synthetic mesh/handle. The canonical loader remains the owner; metadata values survive only in the existing per-frame output Vec and are cleared at reset. Original B stores a reference at output+38; this safe semantic adaptation is explicitly not a native ABI or enum-tag claim. Preserve the existing enum cases for other producer compatibility. This variant is named Mesh rather than Window so future real producers can supply the same proven input without a second registry.

Window source selector remains the existing marker field: ordinary2; shutter !side&1 yields0/1. Shutter color is Some(ColorId(native_u32_04)), ordinary None. Other legacy scalar fields retain known source/wall/position/direction values where represented; unconsumed native padding is not copied or compared. Remove inaccurate old comments claiming metadata+38 is a wall id or quaternion is position.

## Legacy metadata audit and honest nonempty gate

This batch accepts real typed metadata; it does not silently reinterpret all current legacy variants as unsupported solely by name. The actual current writers and original branch evidence are:

| Producer / current task | Existing value and actual missing input | Original route / bounded evidence |
| --- | --- | --- |
| birdnests / TG-2f98fbd22cea8444ea | ArcStrong(cell_key); cell key is not an owned Arc or metadata receipt | full 1422c9c70/3143B has tag4 strong-owner source and selected metadata; current u64 is insufficient to recreate an Arc |
| water setdressing / TG-652b65964d3a98928c | Prefab("lantern"), marker5, host id; direction discarded; mesh selection absent | full 1422d2fa0/2135B chooses metadata from real ClutterMeshes+100/+108/+110 (Reed/Lilypad/FallenLeaves) based on started/hash branch; "lantern" is hash salt, not mesh name |
| tree setdressing / TG-a1aa5940abf2d79ade | Prefab("tree"), marker5, host id; branch/metadata absent | full 1422d3800/2594B selects +100/+108/+110/+118 (Reed/Lilypad/FallenLeaves/FallenPetals); "tree" is hash salt; real tree/drag/corruption state controls selection |
| process / TG-5a9a6f4062d0ae1e59 | source kind3/4/5 becomes empty String or raw integer; pointer and some key bits are discarded | full 1422ce260/2375B has source enum/Arc/reference handling, common metadata+38; current output cannot safely recover its erased owner |

The four current tasks are unclaimed at capture and remain read-only dependencies; their full bounded B ranges and current before hashes are retained. No integer is dereferenced as Arc, no string is used to infer the missing branch, and no existing real resource is replaced. After future producer recovery, Mesh metadata can be supplied through the same adapter. Until then, an actual nonempty output without resolved Mesh metadata explicitly panics with a stable ORIGINAL_IMPLEMENTATION_PENDING diagnostic including source kind/key. Preflight the entire nonempty output Vec before mutating existing ECS or queueing commands, avoiding a mixed partial-success frame. Empty outputs are legitimate and still mark prior actual AutoClutter entities for deletion; this is not an injected empty resource.

This is an explicit remaining blocker of full clutter. It does not permit a whole-game success claim or disable unrelated producers. Their recovery needs root's normal claims separately; this batch does not steal their source families.

## Canonical manager algorithm

Use the actual PDB five-parameter interface from AUTO_CLUTTER_SCHEMA_V2: Res<AutoClutterState>, mutable canonical query with Without<ClutterMarkedForDeletion>, Res<GameTime>, Commands, sheep Transform query filtered With<Sheep> and Without<AutoClutter>. Use standard HashMap<u64,&AutoClutterSpawn> with RandomState as the native generic target-map owner; duplicate source keys retain last inserted value. No persistent side key registry is needed.

For each existing live AutoClutter: add full key to current-live set; if absent from target map queue canonical deletion marker. If present, copy output translation; compute orientation with right=Y.cross(forward).normalize(), up=forward.cross(right).normalize(), Quat::from_mat3(Mat3::from_cols(right,up,forward)). Do not normalize_or_zero or substitute identity in degenerate cases. With WindowShutter, replace base_rotation and queue full ColorId from its source payload; otherwise update Transform.rotation. Preserve existing animated Transform.scale. Do not change an existing descriptor/marker merely because the key matches, consistent with native stable key semantics.

Initialize fastrand from utils::rng::init_rng(GameTime.frame_count). Iterate remaining unique target values, skip keys already live. Check the original sheep guard for every candidate: full 3D distance must satisfy mesh.radius_xz +1.0 <= distance for every real sheep Transform; unordered/NaN fails the guard. Do not replace this with an xz-only test or count limit. New target0/1/2 uses delay=(1-r)*0.0+r*0.1, initial progress0, output target_scale, actual mesh native_group, and exact proven +15 predicate. Do not call the previous manage_scale function.

Queue actual AutoClutter plus NaniBundle. New Transform uses computed rotation, output translation and Vec3::ZERO scale. Named mesh descriptor is actual metadata.name_hash. If solid_texture=None, ordinary selector2 uses SolidVertexColor kind3; shutter0/1 uses SolidVertexColorWindow kind4 and WindowShutter {base_rotation,side:selector!=0} plus the full ColorId. If Some(texture), the four original name hashes (pumpkin_w_face/cart_w_pumpkins/crates_w_pumpkins/barrel_w_candles) use SolidAlbedoMapEmissive kind16, others SolidAlbedoMap kind15, plus actual texture component. The branch is implemented even if current fixed Window metadata mostly has None. Do not use helper palette output in place of actual metadata.solid_texture.

Window selectors fail the original Convertable gate ((selector&6)!=4 && selector>2); no unknown 96B ownership type is manufactured. No Plant kind10 branch is claimed for this Window-only accepted selector domain.

## Actual animation and sound lifecycle

Use the actual four-parameter PDB interface: mutable query(Entity,AutoClutter,Transform,Option<WindowShutter>,Option<ClutterMarkedForDeletion>), Commands, Res<GameTime>, EventWriter<PlayClutterSoundCmd>. The current local animation component/query is removed. Keep the query types common with manager; no wall-index shadow.

Per row: marked→delta=dt/-0.3 and appearing=false. Otherwise if timer>0, subtract dt, delta0; when new timer<0 emit PlayClutterSoundCmd {kind:native_group,position:Transform.translation,context:0}. Otherwise delta=dt/(shutter?0.4:0.3), appearing=true. Express timer branch as >0 so NaN follows the original unordered divide path. Progress update is ordered lower/upper saturation of progress+delta; preserve NaN and signed zero per native MAXSS/MINSS policy, do not use f32::clamp/min/max with different unordered behavior.

Without shutter: Transform.scale=splat(BackOut.apply(progress)*target_scale), on both appearance and deletion. Despawn only when marked and progress is ordered-equal0. With shutter and appearing: use BackOut, sign=(side?+1:-1), angle=((eased*sign*60)-((1-eased)*sign*50))*degrees_to_radians; Transform.rotation=base_rotation*Quat::from_rotation_y(angle), Transform.scale=ONE. With shutter and marked: scale=splat(CubicInOut.apply(progress)); ordered progress0 despawns. Do not accumulate rotated base, rotate around Z, reverse multiplication, discard overshooting easing, or instantly despawn newly waiting unmarked rows.

Original constants are directly read in animation-constants-v1.json. Calls 142923fd0 and142923e80 are PE-import thunks to sinf/cosf respectively, as retained in animation-crt-thunks-v1.json. The temporary quaternion and three sign masks prove base*Y multiplication. utils EasingFunction::apply original737B and current native-oracle tag mapping bind original tags5/1 to BackOut/CubicInOut; reuse the existing actual easing implementation, no hand-rolled new curve.

Component native_flag_15 is preserved by creation but not read by this animation; context is hardcoded0 in original. Sound uses actual existing registered core event, no fabricated audio callback.

## Reset, collection and registration companion

In the owned mod.rs wrapper, restore GameTime as first parameter, clear only existing per-frame output Vec, set epoch=frame_count as u8, clear actual InterShapeBrickStitches.0. Leave canonical Window input to its real drain. Rely on Rust Drop for genuinely owned values; do not claim current legacy ArcStrong(u64) obtains real Arc Drop from this reset.

Root's lib owner initializes exactly the canonical WindowAutoClutterCandidates resource and connects reset before outputs, blueprint current collector before Window drain, outputs before manager. Current relative animation-before-manager placement is retained pending original schedule/set evidence; no claim that original manager must run before animation is made. New scale0 can start animating the next actual scheduled tick. Deferred commands must be applied according to actual Bevy chain dependencies. Earlier schema text prescribing manager-before-animation was only a scheduling suggestion and is superseded by this explicit boundary.

Same-Arc blueprint collection remains outside the entity rebuild gate (original 140827f9e→140828474→140827fb1→140828050); repeated collector→drain→same-Arc collector contributes each frame. Root/02 own that fullvisual caller; this batch does not duplicate its resource.

## Proposed focused source tests and acceptance limits

Tests should run actual systems in a World/Schedule with real mesh metadata values and actual components, not manually fabricate a final Nani receipt. Cover: six-variant drain routing including checked flowerbed index; quaternion w/nontrivial XYZ plus independent translation; full u64 stable key and WallId, full u32 ColorId; no fake prefab; duplicate key last-value semantics; creation attaches actual Named desc/marker/AutoClutter and ZERO Transform scale; rerun updates same entity while preserving animated scale; missing key marks and actual animation later despawns; actual timer crossing emits real sound event once with context0; nonidentity shutter base and Y multiply order; sheep near/far/NaN guard; real input consumed once and same-Arc producer recollects separately through02's caller test. Reject any nonempty erased legacy mesh input explicitly. No test should set scaleONE to make a render appear.

Source tests are to be written after design acceptance but all execution remains root. An actual nonempty OS world/GPU frame and all non-window producers remain unverified. The plan recovers a specific Window route and does not close unknown original tag0 producer, full Convertable ownership, or renderer support for every possible subset.
