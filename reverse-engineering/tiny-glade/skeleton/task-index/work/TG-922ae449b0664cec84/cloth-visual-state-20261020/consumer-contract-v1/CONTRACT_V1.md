# Actual cloth consumer contract v1

This is a frozen evidence/input/output contract, not an implementation candidate or a declaration that the scene chain is closed. B original binary/PDB only; C parity unverified. Root has claimed existing generate_clothline.rs (TG-922ae449b0664cec84) and flag_connections.rs (TG-28b3a129d231927d1f) for owner cloth-visual-state-20261020. No foreign source write, Cargo or GPU work occurs here.

## Actual input, state and native range

Input is the same global DecoratorVisualState.cloth_infos populated from every current typed Blueprint Box before the fullvisual differential gate. ClothInfo is already root-integrated shared schema; it is not NaniClothlineCloth12B, WallFlagCloth20B or a flag-anchor list. Main consumer is original generate_clothline_visuals 0x1407e5190/11167B (PDB440:2044). Complete original body and typed iterator helpers are retained and direct-PE verified. Exact SystemParam signature catalog entry483a2972bd75d470c920edc0 has:

- ResMut<DecoratorVisualState>, Res<AssetMeshLibrary>, Res<AssetShaderLibrary>, Res<AssetTextureLibrary>, Res<UiSignifierStream>, Commands;
- Local<generate_clothline::CacheClothes>, EventWriter<PlayClutterSoundCmd>, Res<DecoratorStorage>, Res<RaycastWorld>.

Wrapper spelling is retained with original binding caveat; actual complete main body supplies the field-use/call witnesses. It reads firstVec at0x1407e544f..547a, maps through from_iter0x141261410 and fold0x141293730 to native CandidateInfo. CandidateInfo extends the value projection with flags/extra payload and geometry; do not rename a simplified FlagAnchor as that native type or guess its extra ownership.

Known required local state is not just a Vec of temporary quad positions: original main compares current decorator IDs against prior IDs for sound emission, maintains pair-set state and computes current connections. Exact CacheClothes semantic field/type design remains a bounded follow-up before source implementation. It cannot be replaced by an empty cache every frame.

## Exact caller domain differs from flag generator

Complete original main call0x1407e572e to greedily_connect supplies stack values at0x1407e56fa..5717:

- max-degree1, flag1;
- distance float codes0x41100000=9.0,0x40c00000=6.0,0.0.

Current ConnectParams::default uses max2,10.0/5.5/2.0 from a different generate_flag_connections caller. It is not a valid default for this cloth consumer. Original greedily_connect complete4312B is PDB438:18296. Its input source tag is read at+0x4d with source !=0, target ==1; wall identity is at+0x30. Current source comment claiming wall+0xc is not proof. Original CandidateInfo/edge ordering, per-node degree and domain filters must be preserved, including index identity when IDs repeat. The current helper's ID-based degree map and unsorted discarded edge weights are not accepted as equivalent without full original analysis.

Original bounded occlusion at0x1407e1d50..1d9c constructs a Ray with unit direction, passes XMM2=distance-0.01, mask0x30db, and calls actual RaycastWorld::intersects_any0x1408cd5b0. A collision behind an endpoint must not reject the edge. Current world API origin/dir/mask discards the maximum distance and its Rapier implementation uses f32::MAX; scaling dir cannot create a bounded ray. Root assigned the required finite-distance API to06/rhapsody_correctness_team, sole world writer. This consumer must use that real API; no always-false closure, infinite-ray approximation or same-file foreign edit. Existing non-native-physics false-return path is an unresolved host execution boundary, not an authorized mock scene input.

## Real entity/asset outputs

The actual consumer does more than return ClothQuad:

1. For accepted connections it reads actual DecoratorStorage info (calls0x1407e62d1/630a/661e), creates per-connection/end-point transform data and deterministic seeded placement state.
2. It spawns endpoint beam entities through real Commands (spawn_empty0x1407e6827, direct command payload follows), then calls the same global transient map add at0x1407e696c (`visual_state+0x48`). Payload/asset descriptor semantics still require the bounded body projection, not a fake mesh handle.
3. It chooses exact installed garment mesh names `decorators/setdressing_clothline_cloth` or `_2`, shader `nani_cloth`, and invokes actual WindowVisualEntityBuilder::build<Transient> at0x1407e7006 with same nested transient owner. The geometry, ClothlineCloth words and NormalMap values must be read from the original field writes; no zero/default payload can stand in.
4. It invokes original clothline0x1407e2510 at0x1407e712c for `decorators/setdressing_clothline_thread` with width1, its explicit endpoint Transform values and actual context. The native helper is2604B, PDB438:28384, not the decompiled text size.

ASSET_INPUTS_V1.json stores original PE string bytes and installed RON/four JSON identities. RON assigns thread/beam to SolidVertexColor and two garments to Cloth. These observations establish actual asset names/subsets, not that host loaders or upload consumers have run.

## Original clothline helper vs current implementation

The native helper consumes a mesh fat pointer, spacing, bool, width, endpoint transforms, id, asset-library/context arguments, sag input and optional style data through14 ABI argument slots. These are not fourteen recovered Rust declaration parameters: fat pointers/context flattening must be adapted semantically. Current comments claiming ten parameters or a pure four-corner generator are wrong as a source contract.

Its complete original body samples curved points, computes segment orientation/length, uses a basis-to-quaternion calculation, constructs a builder and calls real build<Transient> before looping. It does not return a Vec<ClothQuad> for an unspecified host to eventually consume. The basis-to-quaternion math should use the existing math dependency semantics, not recover a dependency derive/allocator algorithm. The bounded clothline caller here passes an absent optional style at0x1407e70a0; other flag caller styles are separate domain and cannot silently be treated as absent.

Current generate_clothline.rs instead collects modified flags, uses adjacent windows, passes sag/default values, accumulates quads, and drops them. Current flag_connections.rs generates source=true/target=true placeholders, hardcodes no collision, computes quads then drops them. Neither is accepted as scene restoration. Existing pure-helper tests do not prove native geometry, cache, asset outputs or actual ECS consumption.

## Same owner and lifecycle

All emitted entities belong to global VisualState.transient, with per-update transient despawn owned by the original clear_out stage. This original main does not prove new persistent map insertions, so this task will not invent persistent cloth entities. Persistent state must survive firstVec clearing/transient update according to its actual owner. Separate fullvisual Blueprint persistent lifecycle remains02's write surface. On glade unload the original whole-global-state default replacement and cache lifetime require actual host wiring; this contract does not expand to its foreign owner.

Audio uses the actual PlayClutterSoundCmd event producer and canonical UiSignifierStream, not a silent default AudioCore or dummy resource. Current event type exists with kind, position, context; original exact byte/context values and which cache transition emits them must be preserved. The complete native body includes real event writes rather than resource_exists skipping.

## Downstream gates, implementation sequence and scope

Before source candidate design can be marked ready, finish bounded projections for: CandidateInfo flags and float arithmetic; CacheClothes ID/pair transition semantics; exact deterministic per-connection endpoint/garment construction; texture/normal handle lookup and garment12B payload; real builder/asset entity ownership; exact system caller registration domain. The existing complete body/decomp navigation provides the next concrete evidence; no whole-PDB label search is needed.

Required integration dependencies are explicit:

- Root-integrated ClothInfo/global firstVec and02 typed fullvisual caller.
-06 finite-distance RaycastWorld API and real enabled collision execution.
- Actual mesh/shader/texture owners; missing names must expose failure, not MeshId-as-VAO/zero texture.
- Current Nani maintain preflight supports nine complete families plus restricted Roof; it intentionally rejects actual Cloth. Real garment writer reaches that gate. Builder's ClothlineCloth/NormalMap currently use local pending_core raw TypeIds, while original set44 query consumes canonical core components. Root authorized a later independent original-query/actual-writer/texture-owner binding contract; it is not included or bypassed here and is not WallFlag20B.
- Whole-App caller/host installation remains visible-source unproven; pending user modules/other root integrations can contain upper wiring. No first-frame panic claim is made without a constructed App.

First source candidate will be bounded to the two claimed business sources and approved necessary existing interfaces, with full before/candidate hashes. It must remove live quad-discard/manual-anchor paths for the actual cloth consumer and execute same-World nonempty typed input into real Commands/entity state. It must not add a mirror resource or count an empty list as success. Full-chain tests must include nonzero transforms/dimensions/IDs, multiple updates/cache transitions, finite collision near/beyond endpoints, real asset-name resolution/failure and exact emitted component identities. CPU tests, typecheck, Cargo and GPU remain root-run only.

This v1 freezes the authoritative input/output and concrete hard gates. It is not yet approval of an unimplemented numeric/CacheClothes projection; that next implementation design will be a new immutable addendum, leaving this contract and prior schema manifests intact.
