# Decorator cache and visual producer, phase 2

## Changes and evidence scope

Original executable SHA256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

The current map was previously implemented as a resource-internal double buffer whose consumers read the preceding frame. Original `140a1aeb0` swaps two independent map arguments; `14081569e` passes the current resource and its distinct SystemParam `Local<CachedDecoratorTransformStorage>`. Original insert/lookup read the current map. The nine-parameter PDB signature corroborates this ownership. The central agent applied `cache-owner-and-fixtures.patch`; no legacy utils/rhapsody interface was restored.

The cache system now retains that original Local scratch and current-frame visibility. Wall destinations use the actual public curve, opposite side normals, or rectangle corner position and normal. The terrain branch reads the destination yaw and original mesh bounds; rotates +X and normalizes XZ; translates the bounds; samples the four axis endpoints in original order; computes the smooth minimum/mean blend plus highest-minus-half-height constraint. Only a missing or >0.001 changed transform marks terrain decorators modified. A sufficiently close transform reuses the exact preceding value.

Non-stair roof destinations use the canonical Roof and RoofTriggerVolume. UV lookup uses the original floor argument; failed UV/missing association retains the previous complete transform or, on the first frame, the original explicit -100 sentinel. A stale actual entity still fails the query. Rotation retains the native type predicate, yaw, rectangular rank guard/position+Y section closest-edge normal, circular <0.01 radial fallback, and base-times-normal quaternion product. Shared Roof section/noise/rectangle/glam implementations are reused. The raw destination boundary retains original normalization of every tag outside 2..4 to the wall branch and its original parity.

## Native versus Rust validation

`terrain-native.py` executes the original bytes, without rewriting the arithmetic:

- `140816282..140816338`: quaternion-rotated +X and normalized XZ, 2400 yaw-quaternion cases.
- `140816362..140816653`: all four sample coordinates/order and the entire support blend, 2400 cases, including each terrain lane's NaNs, infinities, signed zeros and clamp boundaries.
- Only the terrain sampler call is intercepted and supplied explicit heights; the sampler algorithm and ECS traversal are outside this oracle.

`terrain-harness.rs` directly includes the actual production `cache_terrain_support.rs` and links the pre-existing unmodified glam 0.29.3 rlib. It passed all axis components, all sample coordinates, and all finite/infinite/signed-zero heights bit for bit. NaN classifications match; NaN payload bits are deliberately excluded. This is not a native oracle claim for the whole cache system or roof branch.

Additional actual-source roof-orientation probes now pass: 2400 normal cases including 620 native rank panic gates; all 256 yaw predicate tags; 2400 base-times-normal quaternion products. Every finite result and query coordinate matches bit for bit, while NaN payload bits are excluded. Roof section and rectangle-normal helpers are explicit dependency boundaries of that native slice. The UV fallback probe generated 512 original golden rows including the real original Transform::default; its direct ECS comparison awaits the central Cargo receipt. The whole cache system still has static traversal evidence and canonical dependency reuse beyond these bounded native slices. Its ECS and full visual tests are authored in `tests/terrain_clutter_visual_chain.rs`; only the central agent runs Cargo. Consult the central test receipt before claiming these tests passed.

The four actual ECS cases cover:

1. Nonempty terrain storage → same-frame cache → clear → real clutter generator → fullvisual → real collider and Nani descriptor/Transform components; unchanged Arc/entity retention; terrain-only height change triggers rebuild; decorator removal retires all owners.
2. Roof UV/floor/yaw and previous-transform retention on a UV miss, plus the actual first-frame missing-roof sentinel.
3. The 512 original UV fallback golden cases pass through the actual nine-param cache, including arbitrary finite preceding quaternions/scales.
4. The real generated visual enters actual Nani maintain, ApplyDeferred, and solid-vertex-color update; actual active key/64-byte row changes and removal are checked with an explicit CPU buffer/mesh-atlas boundary. It does not publish to or claim a real GPU.

## Host registration evidence

`plugin-build.asm`, `registration-trace.json`, `registration-manifest.json` and the small constructor asm bind original typed constructors to their actual original strings and nested configuration calls. `1403451af` calls outer chain constructor `1400d52f0`; `1403451b4` writes set 25, using GameSystemSet vtable `14292af98`; `140345253` calls `SubApp::add_systems<Update>` at `1403276f0`.

Within that group the current cache is index 2, clear is index 5, generator tuple is index 6, collider-only is index 7. Generator tuple `140529c20` calls clutter converter `1402ad350`. Its later fullvisual group `1403d6ce0` installs instantiate-blueprints, brick/dormer assembly, then copy-blueprints, with chain-ignore-deferred semantics. Cache also appears in other original pre/post-edit subgroups; the narrow candidate does not claim those whole groups are restored.

`host-visual-registration.patch` is a central-review candidate only. It installs the recovered subset at Update/set25 after decorator event/UI work. The ClutterMeshes-exists condition is an explicit reconstruction asset-readiness boundary, not a claim that the original had this run condition. The other original generator/stair/dormer systems are not replaced by an approximate old all-system plugin.

## Remaining gaps

- Type63 on terrain/roof needs the original directed outgoing orientation and `AdjustedStairEdge` owner. Current StairsState is the actual undirected public structural graph; selecting an arbitrary neighbor would change behavior.
- Stair-attached destination4 reads the actual 0x150-byte assembly request and full ladder/step interpolation metadata. The current canonical request has only positions/heights/widths/tangents/segment; its real producer and additional fields remain absent. These entries are explicitly not assigned a fabricated transform.
- Other generators/auto-clutter/dormer/brick/lighting phases remain separate gaps. This delivery closes only the recovered clutter path and the specified cache branches.
- Real asset startup, real physics interaction, GPU submission, and complete in-game feedback require the central application validation. CPU/native slice matches alone do not prove them.

## Efficiency lessons

Start with canonical owners, PDB SystemParams, then original callers. A plausible wrapper can silently add a frame of latency or discard original dirty tracking. Recover the smallest full geometry branch and its side effects; validate the actual production source against original bytes before broad host wiring. Keep shared rewritten geometry, random libraries and resource identities; do not back-port generic dependency implementations. Distinguish static reconstruction, native slice evidence, actual Rust arithmetic, ECS integration, and real game/GPU runs in every progress statement.

## Next-batch stair evidence

No stair request producer or schema was changed. Six newly exported original functions and exact byte/decompilation manifests are retained in [stairs/HANDOFF.md](stairs/HANDOFF.md). Two request producers now have complete decompiler exports; derive_stair_node_params still has unresolved indirect jump-table targets despite the decompiler completed status. These are analysis evidence, not additional gameplay closure.

## Final independent source audit before central Cargo

The visual agent performed a read-only audit of current-map/Local previous ownership,
wall side/corner branches, original roof type/rank/circle gates, terrain sample and
floating-point order, old-transform reuse, and dirty-set insertion. One discovered
raw-tag boundary discrepancy was corrected: original tags outside 2..4 use the wall
branch and retain their parity. The cache source is frozen at the SHA in the final
source-manifest; no stair producer source or schema was edited.

The save-chain agent independently identified the real SharedAssetShaderLibrary
identity required by the current App. Its narrow blueprint consumer and fixtures
now borrow that actual shared owner. It also replaced the test's handwritten
tracking drain with the actual corrected global DecoratorStorage tracking consumer,
and changed the visual test schedule to chain_ignore_deferred so fullvisual-to-copy
keeps its original deferred boundary. This CPU fixture does not claim that the
complete original validate_blueprints phase or whole App's frame boundaries are
installed. Central Cargo results remain pending until its receipt is recorded.

The main host must not mount the generic clutter subset while destination4
positive cache/assembly remains missing: the real clutter generator also accepts
that destination and requires its actual transform. The registration patch is
retained as a reviewed subset candidate, not proof that a complete gameplay App
was run. Mandatory real asset owners and their canonical shared identities must
be ready together; a ClutterMeshes-exists check alone is insufficient.
