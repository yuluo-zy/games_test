# B original Insta event identity and payload evidence

Read-only evidence for history lifecycle owner. No production source, existing claim or Cargo changed. B SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; matching PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911.

## Identity and real chain

`ui-convert-interface.json` binds B `ui_convert_autoclutter_to_decoclutter` 0x14099ca60 to tuple `(EventWriter<StartPlaceDecoClutterInteraction>, EventReader<EvRightClickedAutoClutter>, EventWriter<InstaCreateDeco>)`. The Insta writer starts at tuple+0x48, is copied to rbp+0x10 at 0x14099cace..cadc and passed as argument 7 to `handle_rmb_autoclutter` at 0x14099d89b..d8c0. The latter sends at 0x140977092..711b. It does **not** send EvRightClickedAutoClutter. That original input reader has 16-byte EventInstance stride (8-byte ID payload + native EventId), whereas the InstaDeco instance is 72 bytes. Current production reverse docs misidentified both identity and its trailing EventId as an entity.

`insta-consumer-producer-interfaces.json` binds B `ui_focus_stairs` 0x14079c6d0 to `EventWriter<InstaCreateStairPoint>` and B `ui_place_decorator` **0x1407a9c10** to both Insta readers. References to fresh `0x1407adb20` (show_decorator_icon) cannot justify deleting those real events. The actual placement reader iterates 72-byte Deco and 120-byte Stair EventInstances; see `ui_place_decorator.asm` 0x1407aa0f8..aa250 and 0x1407aa274..aa3ab. History finish observes arrival, but these are commands with real downstream payload, never unit notifications.

## InstaCreateDeco

Payload size 0x40, native EventId at +0x40, instance stride 0x48. Writer counter Events+0x40 is read, appended at +0x40, then incremented. EventId is absent from the Rust business payload.

| Native offset | Defined producer meaning/source |
|---|---|
| +00..18 | 24-byte destination union, copied from `AutoClutterConvertableToDecoClutter` +18..30. The real query resolves that component at 96-byte stride. Semantic enum variants need further recovery; retain all 3 words. |
| +18..29 | Inactive extra enum payload / padding, **not initialized by the producer**. Do not create business fields from these stack bytes. |
| +29 | Extra enum discriminant 3, empty branch. Current typed `Option<DecoratorExtraInfo>` can represent the absence without native padding. |
| +2a..2c | Alignment padding. |
| +2c | f32 rotation, original quaternion/yaw path result. |
| +30..3c | Transform.translation Vec3: helper RCX is Transform, entry first 16B quaternion, RSI=RCX then +10 before the writer. |
| +3c | ClutterMeshTy::from result byte. |
| +3d..40 | Alignment padding. |

Consumer 0x1407ab2d1 reads rotation; 0x1407ab3fe..b413 transfers extra record; 0x1407ab47c..b493 copies Vec3, +3c reads type. Destination fields are branch-dependent at 0x1407aa1fc..aa24b and 0x1407aafa1 onward. Suggested current-architecture type: destination `[u64;3]`, optional existing extra, f32 rotation, Vec3 position, existing ClutterMeshTy. No fabricated entity or padding. Current AutoClutterRecord lacks this union and must receive actual source data, not synthesize it from an entity ID.

## InstaCreateStairPoint

Payload size 0x70, native EventId +0x70, stride 0x78. This evidence closes every writer copy and its downstream identity; some semantic names remain unresolved. A lossless type may retain a fixed-size opaque **business payload** until its semantic model is recovered. Never call the trailing EventId an entity and never default unknown active data to zero.

Writer 0x1407a0db8..e94:

| Payload offset | Native source |
|---|---|
| +00..08 | qword local rbp+668; destination selector set at c71..c93 (3→1 special case). |
| +08..10 | R14; paired destination payload selected at c8a/c9a. |
| +10..38 | 40-byte destination/detail block assembled c97..cbe and copied from rbp+00..28. |
| +38..44 | 12 bytes rbp+490..49c, originating current focus data first Vec3 copied at d54..d67. |
| +44..48 | f32 XMM8, adjusted in ccc4..cc9 only for special destination branch. |
| +48..4c | f32 XMM6 = current focus data+30. |
| +4c..54 | 8 bytes XMM7 from existing stair extra+8 or zero when absent. |
| +54,+55 | bytes existing stair extra+10/+11 or zero when absent. |
| +56..64 | overlapping copies from rbp+580..58e; those locals have mode/options and Vec3 copied at d84..d97. |
| +64..68 | existing decorator info+28, fallback 100003 (0x186a3). |
| +68 | literal 0x3f. |
| +69..70 | 7-byte copy rbp+240..247 (two overlapping 4-byte stores). |

The consumer preserves these fields when constructing its real decorator op: 0x1407aa405 checks +68 == 0x3f, aa40f..aa4c6 copies destination/extra/position/color data before `apply_decorator_op` 0x141276e30. Full placement/hover algorithms, creation side effects, startup/global ordering and semantic enum decoding are outside this bounded writer schema audit.

## FinishInputRecordingCmd is unit

The B `color_menu` exact original FunctionSystem tuple contains 16 event writers; FinishCmd is the last. Its native tuple field +1e0, tick pointer +1f0, tick +1fc are loaded at 0x14229169a..16b6 and retained in rbp+1e8/b8/2bc. Writer 0x142294871..48c9 appends exactly one qword EventId at `buffer[len*8]`, increments len and event counter, and writes no business payload. Its grow alias at 0x140562870 includes EventInstance<FinishInputRecordingCmd>. Thus a unit Rust event is evidence-backed here.

## Native validation and limits

`native_insta_writer_probe.py` executes original writer spans with preallocated Events and actual locals/registers. 12 cases per writer (36 total) pass exact defined-field, tick, EventId and stride checks; no game callback or writer is hooked. Shared harness hooks allocation/memory utilities, but none are reached with these capacities. The Deco fixture deliberately seeds inactive stack bytes and verifies only defined fields; their garbage values are not modeled as required application state. Stair uses raw bytes to prove byte transport, not infer validity of every random enum input. This is not a producer guard/placement behavior oracle. All saved asm instruction bytes independently match the bound original image in `original-byte-verification.json`.
