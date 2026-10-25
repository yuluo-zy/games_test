# Fullvisual body candidate v1

This source candidate continues the reviewed body design `d2bb88e6...` / independent review `83fb4932402468146d62324a67620e20f053bf5e5ebe624856a23cb4d63b0e24`. It rebases exactly on the integrated equality/fields source `506501c8e5f506ab91bc8a78fd05d04c4fcc79ada0396f9a747cd77289618b95`. No frozen earlier package or foreign source was changed. Root alone integrates, builds and runs GPU tests.

## Implemented business body

The public `instantiate_blueprints` retains all fourteen actual SystemParam types, including the real AssetTextureLibrary scheduling dependency. It transfers the original tracked wrappers into one production body. The body is itself a real thirteen-parameter ECS system used by tests; the unused texture parameter alone is outside it. There is no second diff algorithm or fake texture device/resource in the test path.

The body clears the same state's actual cloth Vec, calls the canonical collider helper, creates removal work from previous blueprint keys, and processes every current value. Hole/candidate/cloth output occurs before Arc/value equality. Changed values remove old persistent state and enqueue its despawns, clone and build the actual builders using the outer map DecoratorId, seal, then check the original optional outline Transform. Old tracker entries are removed using the previous source, then new entries are added using the current source. Removed previous IDs lose their persistent entities and old tracker outputs. The actual OnWallDeleted reader runs at the tail and cleans both canonical trackers.

Whole-blueprint value equality covers the exact source address, ordered builder list, holes, mixed float/integer canonical dormer records, both three-state positioned-mesh slots with Arc identity, canonical window-candidate list, ClothInfo/Chimney/Door box values, outline transform values and the two observed flag bytes. Box equality is value equality; mesh equality is allocation identity plus ordered pose. No global always-changed shortcut or whole-frame respawn is used.

Root's performance correction is included only in this new candidate: BrickColor and WallFlag packed comparisons are lazy closures invoked at their original top-level positions in the native builder equality's `&&` sequence. They are no longer computed before translation/rotation/scale can already return false. This changes work avoided on an unequal pair, not the recovered equality relation. The earlier frozen Eq package is unchanged.

## Actual outline mapping and legacy boundary

The original call `0x14082881a -> 0x1408251f0/125B` is typed `Option<utils::components::transform::Transform>::eq` and compares the same ten rotation/translation/scale f32 components as the current canonical CachedDecoratorTransform. Its presence is checked after build/seal. The actual clutter generator calls `set_cached_transform`, writing `blueprint.transform: Some(CachedDecoratorTransform { ... })`. That is the field tested here. The old `outline: Option<(f32,f32,f32)>` compatibility tuple is explicitly not accepted as this option and cannot make a missing canonical transform valid.

The old scalar chimney and byte cloth setters are not native boxed outputs. They remain explicitly legacy and do not fabricate typed Some values. Their old producers still have missing source/mesh/transform contracts and are not accepted as recovered generic scene producers. Actual typed output fields are separate `cloth_info`, `chimney_assembly`, `door_stairs_assembly` and `window_auto_clutter_candidates`. The source does not infer a default storage address when a real side effect requires one.

The old door-stair setter is different: its exact native `add_door_stairs@140825830/116B` is retained in this package, including its 32-byte copy/replacement contract. All current sys-decorator source references to the old field were its own setter; external callers use that method. Therefore the existing `add_door_stairs([u32;8])` signature now stores one actual `Box<DoorStairsAssemblyParams { words: params }>` in the canonical field. The unused duplicate `Option<Box<[u32;8]>>` field is removed. This does not reinterpret float bits, default a value or allocate both a legacy and a real box.

## Native None projection versus actual build

The independent Eq batch projected native None markers: BrickColor/WallFlag/Smoke tag2, solid selector9. The now-integrated builder companion `59d915f2...` applies the same three tag2 gates in actual build, so those fields cannot be hidden as equal while still inserted by build. The active payloads are untouched.

For solid9, the native plain and emissive albedo branches check it at `0x1412a9fd7..9fe3` and `0x1412aa5ea..5f6`, failing **before** spawn. The current same builder source likewise unwraps and checks the selector before its Nani spawn: both None and Some9 fail in either active branch. Other shader branches do not read that selector, and projecting its native None marker has no build-output effect there. A change from a successful active-branch Some0..8 to9 makes Eq false and reaches the real rejection. A previous successful active-branch None/9 is impossible in the original fullvisual→copy sequence; same-Arc snapshots are not safely mutable in both maps. This requires the separately reviewed actual phase contract. It is not approval of arbitrary externally injected previous maps or out-of-domain raw selector values. The actual recovered clutter producer is narrower still: canonical loader metadata supplies only None or checked Some0..8.

## Bevy mutation/tick contract

No mutable wrapper is eagerly dereferenced at the public entry. Thus an empty current map does not incorrectly mark WallHoles, WindowCandidates or either tracker changed. VisualState is marked before the unconditional first-Vec clear; the real world/collider owners are passed mutably into the actual collider helper. Per-current candidate output marks its actual owner even for an empty source list, as the original body does. Other owners are marked on their real side-effect branches.

The clear-blueprints wrapper now uses the original ResMut<DecoratorStorage>, marks it before the immutable assertion, and never clears its authority map. Original PC `0x14082633e..351` establishes this. The blueprint owner's changed tick is only written when there is an attempted affected-ID removal, including a missing ID; native writes are at `0x14082657a..581`, `0x140826779..780` and `0x140826a03..a0a`. Empty bucket visits do not mark it changed. This is a narrow correction to the existing wrapper; the retained method still performs the already reviewed actual removal set.

## Required canonical companions

* WindowCandidate/resource: actual integrated core collision source `5cda7a4ed55acffd6c26c1d3e78ede79b945fd277549f2bca3f6808b6498e184`.
* ClothInfo/first Vec: actual integrated visual-state source `976977a3d40ff291a75d8e9b996555687058c19a7f9768ec3d122c91633da502`.
* Builder three None gates: actual integrated builder source `59d915f2270c25781281b62a8afce9e1fe43bb1a4bc27b49ce2c5392991a00a9`.
* Tracker and both Params: exact three-source candidate under 05's `TG-bd3d82a9ddc435cfb2/deco-trackers-20261020/CANDIDATE_MANIFEST_V1.json`; add/remove/on_wall_removed are real methods, Params live in their original two assembly modules and are reexported by the diff module. No local replacement type/resource is supplied here. Root and 07 must approve and integrate that same dependency set.
* Scheduling: 06's actual outer member14 fullvisual/copy chain. Preview collider-only member7 and validate's modification-tracking clear at member9 remain independent actual stages. This package edits no registration or Cargo file and does not invent a final-frame cleanup phase.

## Verification and remaining limits

Six additional tests use real canonical owners in one Bevy World and execute the very same production body/SystemParams/Commands: Arc-identical and different-Arc/value-equal frames collect all three outputs without respawn; changed and removed IDs replace/despawn actual ECS entities; missing canonical outline fails after build/seal/collider state; unvisited mutable owners keep their changed ticks; clear marks storage but only touched blueprints and rejects immutable storage; actual wall-delete events clean both real tracker owners. The fixture intentionally does not supply an artificial texture library: it executes the production business helper, while the public system still requires the real texture scheduling resource. It tests ECS/collider/lifecycle behavior, not atlas resolution, GPU upload, presentation or a completed Game frame.

All eleven cumulative new tests (five Eq plus six body/wrapper tests) are unrun. Rustfmt parse/format succeeds; no source typecheck is claimed. The actual dormer writer/upload path remains unrecovered, five native-helper/current-sidecar subset mismatches remain a version-provenance boundary, non-clutter legacy producer migrations and tracker geometry/audio consumers remain separate work, and the original physics debug-log owner is still absent. Implementing this consumer does not make those inputs or the full application ready.
