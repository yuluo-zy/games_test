# Canonical Cloth instance binding design v1

This is a bounded B-only design, not a source candidate or a statement that the cloth scene producer is reachable. C parity, whole-App construction, CPU execution and GPU execution remain unverified. The existing nine complete ECS mappings and restricted Roof-bearing branch are preserved. The other unavailable domains are not registered or bypassed.

## Three distinct values

`DecoratorVisualState::cloth_infos` contains the already integrated `ClothInfo`: Transform, WallId, two ordered floats and DecoratorId. Its native allocation is80B with68B effective fields. It supplies connection candidates; it is not an instance payload.

The Nani update consumes canonical `country_core::components::ClothlineCloth`, an observed12-byte value. `WallFlagCloth` is a separate20-byte raster parameter. This design neither aliases those types nor copies their fields into each other.

The complete original update14207c0a0/3253B is PDB3098:184028. The existing set44 tuple evidence identifies update member13, kind20, with this exact original domain:

```text
Query<(&NaniActiveInstance, &Transform, &ClothlineCloth, &NormalMap),
      With<InstanceDataMarker<ClothInstanceData>>>
Res<AssetTextureLibrary>, Res<GpuWorld>
```

There is no Changed filter or additional marker exclusion. The required payload Query is retained. No Option<ClothInfo>, WallFlag parameter or default row is substituted.

## Canonical component and actual builder value path

Original update14207c938..96a copies the first8 bytes and tail4 bytes of ClothlineCloth into the final instance row; the Query advances the component by12 bytes. The real garment producer1407e6f16..28 writes its present discriminator, two derived float words and a derived counter/id word. The complete builder1412a9b30 and transient variant1412ab270 copy all12 bytes when the flag at self+80 equals1. These observations fully support a non-owning three-word value; they do not recover original author field names or its Rust declaration.

Proposed current-architecture representation in the already delegated `components/drawable.rs`:

```rust
#[derive(Component, Clone, Debug)]
pub struct ClothlineCloth([u32; 3]);
impl ClothlineCloth {
    pub const fn from_words(words: [u32; 3]) -> Self;
    pub const fn words(&self) -> [u32; 3];
}
```

No Default, Copy, Resource, guessed boolean, enum labels or ABI/layout promise is added. All32-bit patterns can be represented without unsafe conversion. The native clone_behavior14084cc00/13B selects the existing Bevy clone callback specialized for this exact original type; this justifies Clone and reuse of Bevy, not reconstruction of its clone algorithm. A root-owned one-line re-export at components root may preserve the original public module path. It must re-export this single type, not define a second component.

The builder retains its existing optional `(u64,u32)` storage for compatibility with precise fullvisual comparisons. Its present case inserts `ClothlineCloth::from_words([a as u32,(a>>32) as u32,b])`; no payload byte changes. `pending_core::ClothlineCloth` becomes a public re-export of the canonical type. The current no-argument `clothline()` helper writes unproved zero words and has no visible production caller. Replace that convenience API with an explicit payload argument; do not retain it as a successful default producer. The restored actual consumer will supply the original derived words. This API spelling is an engineering adaptation, not an assertion of the original Rust signature.

`NormalMap(pub u64)` already exists at `components/rendering.rs`, exported from `components/mod.rs`. The builder currently defines a distinct local `[u8;8]` component. Remove only that local definition and publicly re-export the existing canonical type; insert `NormalMap(handle)` with the same complete u64. It is a disk-texture NameHash, not a GPU handle or descriptor index.

## Texture owner, row update and scheduling

The original update locks owned subset20 before examining its Query (14207c334..339). It searches the supplied normal-map name, panics on missing name (c9cf..9db), calls the actual texture library handle_to_bindless_index at c7fc, and appends the returned descriptor index as word15. It copies ClothlineCloth words to12..14. The existing `instance_updates::build_cloth` already implements the16-word row and transform/sort-key math; it must remain byte-for-byte unchanged in this binding package.

The adapter takes one `GpuWorld` subset lock per system, including an empty query, and reads the same canonical Transform, ClothlineCloth and NormalMap components. Resolve `NameHash(normal_map.0)` through the actual `AssetTextureLibrary::get_handle_and_texture_by_name`; absence must fail. Pass its actual AssetIndex to the same library's `handle_to_bindless_index`, preserving descriptor-capacity/host failures. Do not use the asset index, texture id, u64 hash, a zero or fixed maximum as the descriptor index.

The new ECS adapter then calls existing `build_cloth` and `write_mapped_update` with the actual key. Dense-index cache repair continues to bypass change ticks as in the integrated mappings. The existing same-world GPU subset and flush/count producer remain authoritative; no row Vec, mirror GpuWorld, staging component or separate buffer is introduced.

Extend maintain's host completeness preflight only for actual descriptor-bearing Cloth entities: unique canonical mapped marker, Transform, ClothlineCloth and NormalMap must all exist before new key/count publication or changed-descriptor processing. Keep every previously supported guard and key-kind identity check. An empty registered Cloth subset is a real zero count; do not fail merely because the subset exists. Unsupported actual other-kind descriptors still fail. The original required payload Query also remains visible as such; the descriptor preflight is not claimed to inspect arbitrary descriptorless entities.

Install the new updater in the existing helper's sibling update tuple, at original relative member13 position (after available BalconyDoor, before available Uber). Keep maintain -> explicit ApplyDeferred -> unordered update tuple -> flush with chain_ignore_deferred. No extra implicit barrier, new gameplay_startup installation, VAO prefix duplicate or whole20 empty registration is introduced. The root-integrated per-App VAO prefix is a separate prerequisite and is unchanged.

## Real producer reachability and limits

At the source snapshot, the current generate_clothline system still builds and discards quads, and no production caller writes a nonzero clothline payload or normal_map field. The original main's actual garment builder is proven, but its numerical/cache/geometry projection is still a separate claimed consumer task. Therefore this package can establish the real builder API -> canonical ECS -> existing row -> flush binding once its tests run; it cannot alone establish whole-scene garment reachability. The two window ClothInfo producers and global lifecycle are separate upstream consumers/producers. Pending user modules may contain upper registration; this report does not infer first-frame failure from missing visible registration.

The restored consumer must retain actual texture hashes0xb9fbcd499788b2f1 /0xe21f7890eabecf74 and original mesh branch values until their name mapping is independently established. This design does not guess asset names from those hashes. Native0..8 solid selectors and None9 are unrelated to this three-word payload.

## Candidate surfaces and meaningful validation

Only current delegated drawable.rs, visual_entity_builder.rs and nani/systems.rs need implementation changes, plus a root-coordinated components-root re-export if required. No new module or foreign rendering.rs source is needed. instance_updates.rs numerical functions, world.rs, protected critter sources, fullvisual Blueprint, runtime/game_loop, graph generation, dependency manifests and Cargo files remain outside this package.

The next source candidate must include exact before/candidate/patch hashes and tests using the real builder Commands and same World:

- Nonzero and negative-zero/NaN bit-pattern word preservation through explicit payload and canonical component identity, with a nonzero NormalMap u64; no default constructor.
- Real matching Cloth atlas part and real AssetTextureLibrary named asset with a non-identity descriptor slot. Maintain -> ApplyDeferred -> update -> flush must produce the16 expected words, one actual requested count and the same canonical key/subset.
- Missing canonical ClothlineCloth, NormalMap, Transform or mismatched/multiple marker fails maintain before key/count publication. A missing texture name fails the actual updater instead of publishing a zero index.
- Repeated updates reuse the actual texture descriptor and key, update changed payload words, and removal releases the same active instance. Empty query preserves the original subset-lock behavior.

These are CPU boundary tests using existing test-device machinery, not a substitute scene or a GPU claim. Root alone compiles/runs them. New tests, source candidate, typecheck and execution are currently absent. Do not mark the tenth complete scene family or the cloth consumer done on design approval.
