# Dormer ownership and deletion contract V1

Status: source design for canonical DormerTiles and the B maintain_dormer_tiles consumer. Geometry implementation is not approved or complete by this contract. No source candidate, production change, Cargo, GPU or executed tests in this package.

## Identity and scope

Original B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03, matching PDB 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Live claims TG-8d67366f4f17082525 (canonical resource), TG-85096a7d1ed73fb01d (B/C spelling maintain), TG-971fce9dfe7b86b81c (assembler) belong to scene-clutter-producer-20261020. Root is the production writer. A-only misspelled mantain task is excluded. The separately authorized gothic helper TG-e2441208d52bdbc7bc has now been normally claimed after live null inspection; its algorithm remains a subsequent design gate.

This is a semantic Rust representation, not a native HashMap ABI or a claim that B and C algorithms are equivalent. The resource B/C file checksums match; that does not establish equivalence of its callers.

## Two distinct output paths

1. Original visual generators construct full 48-byte DormerHoleSsbo records and append them to BP semantic Vec cap+e0/ptr+e8/len+f0. Current fullvisual collects the values every eligible invocation, including same Arc / full equality paths. The uploader builds the 225-cell accelerator and writes its counts / full fixed table. These records are not the input to roof tile tessellation.
2. generate_dormer_roofs@1407cb1a0 (14207 B) reads DecoratorBlueprints only as an ID-presence gate (1407cd250..1407cc590). It reads authoritative Storage/info, CachedDecoratorTransforms, PublicWalls and Query<&Roof>, builds geometry using cottage/gothic algorithms, and writes the existing SparseInstanceBuffer<RoofTileSsbo> through the same AssetSsboLibrary. Local<CacheDormerPositions> is a decorator-to-(wall,cache) history, not a second GPU allocation owner. No 48-byte BP Vec is read by this function.

The true typed nested schedule constructor @140492d80 spells ten generator parameters: DerivedDecoratorInfoResources, Res<DecoratorStorage>, Res<DecoratorBlueprints>, Res<CachedDecoratorTransforms>, Res<PublicWalls>, Query<&Roof>, Res<AssetSsboLibrary>, Res<UiSignifierStream>, ResMut<DormerTiles>, Local<CacheDormerPositions>. Its second chained function is ResMut<DormerTiles>, EventReader<DeleteRoofCmd>. This is the existing nested first member of the middle tuple; no additional BufferHost or duplicate registry is introduced.

## Canonical resource and real allocation ownership

Proposed canonical source representation in country_core::resources::roofs::dormer_tiles:

    #[derive(Resource, Debug, Default)]
    pub struct DormerTiles {
        pub allocations: std::collections::HashMap<WallId,
            SparseInstanceBufferAllocation<RoofTileSsbo>>,
    }

Reuse canonical WallId, canonical resources::roof_tile_ssbo::RoofTileSsbo (the roofs path is its existing re-export), and existing render::sparse_instance_buffer owner. No Clone implementation: allocation owns a non-cloneable handle and a weak retirement link. Removing old marker / Clone is necessary to represent this real owner; all crates source search found no marker construction or DormerTiles clone consumer.

Evidence is not inferred solely from a generic symbol: original generate body loads Roof first qword at 1407cc217..21b, hashes it, probes/inserts a 24-byte entry (key + weak + allocation pointer), allocates an actual 0x20-byte Allocation with empty u32 Vec and mutex at 1407cc2fe..330, obtains the weak reference from this exact RoofTile buffer +60 at 1407cc334..379, and writes it into the same map at 1407cc3e6..3f5. Existing entry reuses the same allocation. Roof::wall_id is its first qword; DeleteRoofCmd canonical payload is WallId. Original RawTable remove at 140053320 has an exact alias tuple<WallId,SparseInstanceBufferAllocation<RoofTileSsbo>>; other ICF aliases such as callback maps do not change this actual consumer.

generate per dirty wall takes the actual allocation writer, selects its tail chunk when present, then clears it (1407cc406..1407cc4a4). The planned assembler will use the existing buffer.new_allocation() and writer/clear APIs and preserve clear-before-sequential-push / partial-failure semantics. No replace_all atomicity is added. Complete geometry and failure placement remain the subsequent assembler design, not permission to put an empty allocation into production as successful geometry.

Default is an empty std HashMap with RandomState. The original resource default has only inline instances (module141 records91876,129132), containing std HashMap::default / RandomState initialization; no independent native void() constructor exists. Current marker comment about a one-glue resource does not describe the value owner. Resource unload/drop must drop entries, reaching the original allocation retirement mechanism. Current source already has the same SparseInstanceBufferAllocation::drop weak-upgrade -> waste queue behavior; we reuse it.

## Exact deletion consumer and change tracking

Restore public ECS function:

    pub fn maintain_dormer_tiles(
        mut dormer_tiles: ResMut<DormerTiles>,
        mut deleted_roofs: EventReader<DeleteRoofCmd>,
    )

For each actual deleted_roofs.read() event, call drop(dormer_tiles.allocations.remove(&event.0)). This obtains mutable resource access only inside each event body. Empty reader does not dereference ResMut or mark the resource changed; any actual event marks it changed even when key is missing. Missing/repeated IDs do not panic and do not synthesize values. No clear of the entire authority and no manual event array / cursor / output pointer is retained.

Complete B body@1407ce920 length334: 1407ce930..99b selects the real Bevy two retained event buffers and prepositions the ManualEventReader cursor, preserving missed-event semantics. Each selected event adds8 to eventinstance to get its actual command payload (1407cea06), increments the reader cursor (1407cea0a..0f), writes the ResMut change tick from wrapper+1c through wrapper+10 (1407cea12), hashes the WallId command (1407cea16..1c), removes same canonical map (1407cea21..32), then if the returned allocation weak is nonzero drops it (1407cea37..53). Typed run@14015c360/625 and without@14042a860/764 bind those two real SystemParams and call the exact334 B business body. Original eventinstance stride24 is Bevy metadata plus 8-byte command, not an authored 24-byte tile element.

Current maintain source's DormerTilesDrawState / DormerTileDrawBuffer / DormerTileElem, unsafe Send, raw cursor, id_slot and value are incorrect host inventions caused by treating ResMut fields / EventReader as an application state. Full crates source search shows only their definitions and own tests, no actual external caller. Replace that file's face with the true ECS function; no compatibility shim to fake an event stream. Existing lib module visibility can remain; registration is a separate root companion.

Allocation drop is not immediate GPU upload or physical deletion: existing weak retirement queues the allocation for the same sparse buffer's next normal reclamation. Preserve that ownership fact; no fake completed GPU result is returned.

## Required meaningful verification (not executed)

- Use the production ECS system with actual Events<DeleteRoofCmd>, ManualEventReader state owned by Bevy, and the actual canonical resource. Insert two real allocations produced by an existing SparseInstanceBuffer<RoofTileSsbo>, not hand-filled handles.
- Delete one wall, preserve the other, and observe exactly one real allocation in the same buffer.waste queue. Repeating / missing IDs leaves that retirement count unchanged.
- Empty event iteration must leave resource change detection untouched; a missing-ID event must mark it changed, proving mutability ordering.
- Multiple events and successive schedule runs must not consume the same event twice; exercise the existing event retention boundary.
- Dropping the resource retires remaining real allocations through the same weak owner; dropping an empty resource performs no fake GPU action.

The recording GraphicsDevice boundary may construct a real CPU sparse buffer for these ownership tests. It must implement the existing trait and must not become a second application BufferHost. No new Cargo dependency is needed.

## Explicit remaining gates

Full14207 B generator cache projection / mixed ordered equality, original branch domain including subtype0 rank3 bay windows and subtype1/2/7/8/23, cottage10926 B and gothic5713 B tessellation, callback1362 B packed64-byte RoofTile values, original RNG owner/version, exact growth/error branches, public current geometry helper APIs, and actual system installation are not completed by this contract. The current generator's empty BP overwrite and current gothic empty Vec remain wrong; do not register them or advertise nested roof geometry as ready. Actual seven record producers also need complete input/domain restoration rather than appending records from default shapes. The uploader source review and integration do not discharge those gates.
