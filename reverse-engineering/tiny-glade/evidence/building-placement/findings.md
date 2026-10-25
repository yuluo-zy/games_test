# G09→G10: B actual plain-wall placement input

## Implemented boundary

`reconstruction/building/decorator_placement.rs` recovers the private game reduction in `Mesh::aabb_bounds`/`collision_bbx`, `WallSpaceCoord` from/to world, `DecoratorLibKey` byte order, and the original four plain-family rank→collision-asset-name policy. `reconstruction/crates/tg-building-runtime/src/decorator_assets.rs` reuses Serde JSON to read shipped float3 `Vertex_Position` buffers and populates actual collision records. It provides `load_b_plain_wall_assets(assets_root)` and explicit `load_bindings`; errors never produce fabricated dimensions. The version-specific entry requests all four plain families, and does not reproduce the original gothic availability gate or renderer/interaction/outline initialization.

The first-batch caller supplies a real world position, rank, and family intent, which selects a plain subtype. A subsequent complete CPU wall subtype consumer now replaces this selection with actual roof/publication-dependent half/plain behavior and original door/corner precedence; see `half-dormer-findings.md` and `B-subtype-verification.json` (1,326 cases). Full non-wall roof attachments and original authored attachment creation remain separate consumers.

## Three-version ownership and dependency filtering

See `three-version-provenance.json` and `three-version-loader-next-provenance.json`. Original game modules are `country-core/render/mesh.rs`, `country-core/resources/walls/decorator_{collision,subtype}.rs`, `utils/wall_space.rs`, and `systems/decorator/load_decorators.rs`. The `decorator_collision.rs` source checksum agrees across A/B/C. B/C `wall_space.rs` and `derive_decorator_subtype.rs` agree; the loader differs in all three versions. C adds a published `reload_decorators_after_mod_swap` entry, so B names/default loading must remain explicitly versioned until C policy is compared.

`DerivedDecoratorSubtypes`/`DecoratorMeshLibrary` Bevy SystemParam `init_state` at B 140b26e40 is dependency registration mechanics, not an asset-name table. HashMap hashing/probing, Vec/Arc ownership, Serde parsing, and ECS traversal are reused or bridged at declared boundaries. They have not been translated into purported core gameplay code.

## Native proof

`B-placement-verification.json` records **1,668 cases, zero failures**, with the untouched B RNE digest, candidate source/binary hashes and all 12 actual asset hashes. Cases: 12 actual mesh bounding boxes, 512 random finite vertex sets, 512 world→wall conversions, 512 wall→world conversions, 12 original key byte-order cases, 12 original loader rank→name branches, and 96 composed original mesh→WallSpace→four-family raw-hole cases. Arithmetic comparison is bit exact; no tolerance or expected collision dimensions are injected.

The bbox bridge supplies a successful float3 `Vertex_Position` slot after the original prologue, skipping only std hashing/probing. Original private min/max, center, extent and collision-record construction execute. Loader observation begins at the original rank dispatch after availability/resource acquisition and stops at the first selected lookup. It supplies rank and valid resource/frame pointers; original instructions select the name. The composed oracle executes the original bbox, from/to WallSpace, and established original raw-hole producers. This is a composition of actual game instructions, not execution of the entire original application.

Raw/source Curve2 is mandatory: the `WallSpaceCoord` helpers read points/normalized point parameters/source length at the start of PublicWallState. PublicWallState::wall_space separately reads +0x258 shape and +0x28c published raw length. These helpers do not consume a remapped 3D-U curve. The raw publication resource supplies this Curve2. Wall local Y is the actual world Y. The original negative-distance to-world arithmetic is retained, including its unusual subtraction of a negative distance.

The runtime reader and actual World integration are validated separately by the parent. A package `cargo check` reached the runtime but was initially blocked by a concurrently edited roof consumer calling missing `roof_state::pivot_3d_observed`; no asset-module type error was reported. Standalone placement probe builds offline in release mode.

## Critical next input recovered during this batch

Original B `is_half_dormer` 140814470 uses:

1. Circle/rectangle wall shape only (+0x258 ≤ 1).
2. Optional actual Roof: flat roofs return false; gable roofs exclude the two actual gable rectangle sides, using wall coordinate→world and closest edge.
3. Original source Curve2 wall coordinate→world.
4. Published **mapped Curve3** at +0x88, searched by X/Z with original thresholded 3D semantic, producing segment index and local fraction.
5. Actual per-segment arch heights at +0xe0/+0xe8; linear interpolation in the original instruction order.
6. Actual plain collision-library entry selected from rank and decorator dtype 0x38/0x39/0x3a (plain subtypes 0/6/22).
7. Original collision vertical bounds and world Y, compared to interpolated wall height. In the observed instructions the comparison uses `world_y + ((max_y - min_y) + ((min_y + max_y) * 0.5)) * 0.5`. This unusual formula is evidence, and must not be simplified into an assumed bounding-box top before native differential verification.

The relevant original disassembly is saved under `140814470-original-disasm.txt`, `14126a450-original-disasm.txt`, and `140c9cde0-original-disasm.txt`. `140c9cde0` merely inserts the computed Y into a Vec3; it is not a hidden alternative extent computation. Actual door detection at `140b34f20` is qword != 0; subtype derivation is not just an arbitrary family enum. Parent unified exports are in `evidence/building-integrate/B-placement-next`.

This wall consumer has now been implemented using real mapped outline/arch cache and actual Roof branch state; see `half-dormer-findings.md`. Roof UV→position and RoofTriggerVolume construction remain a separate required upstream package (B 1408e46d0 and 1421a3220, C 140a04ed0 and 1428220a0), with bounded C checks using C addresses only. A complete roof/decorator claim is not supported by the current wall chain.
