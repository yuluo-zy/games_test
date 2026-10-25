# TerrainEntities original identity and producer direction

Read-only B proof for root's narrow existing-module Resource addition. No new startup implementation, production edit, Cargo, original execution or JIT by this reviewer.

## Producer: real floor and rocky spawn results

Reuse the unchanged frozen147 startup_terrain140926bf0 full5382B PDB range. Mesh-name reference140926d43 points to exact5B `floor` at142aaef28; its shader ref140926d8d points to exact34B `/terrain_editing/floor.raster.hlsl`. First relevant EntityCommands spawn call140926db0 returns its Entity at result `[rbp+80]`;140926dc2 loads it intoRDX and140926e3a retains it at`rbp-40`. That same value enters the actual floor insertion bundle at140926e3e.

The rocky subsection loads the real Rocky asset and creates its shader from exact40B `/rocky_terrain/rocky_terrain.raster.hlsl` at142aaf008 (ref1409270d8). Spawn1409270fb returns its Entity from`[rbp+80]`;14092710d loads it intoR13, also used by the rocky entity's insertion bundle140927177. The optional developer middle branch does not replace the retained floor identity;140927124 reloads `rbp-40` intoRDX immediately before the rocky insertion/Resource command sequence.

Resource command1409271b6 loads thunk1412d8b00. Command record has the thunk8B, then1409271c1 stores RDX at+8 and1409271c6 storesR13 at+16. Thus the **Resource payload is16B: floor at+0, rocky at+8**, both from actual spawn results, not mesh handles, counts or fabricated defaults.

## Typed command consumer

FnOnce1412d8b00 is the matching complete PDB71B/22instructions `insert_resource<TerrainEntities>` thunk.1412d8b05 advances the payload cursor by0x10.1412d8b11/14 MOVUPS copies the complete16B unchanged and1412d8b1e calls142058f60. No swap occurs.

Command apply142058f60 is matching complete PDB1341B/304instructions for exact `country_core::resources::rocky_terrain::TerrainEntities`. Original Resource TypeId immediates are high5dda49fe35d11774 at142058fab and lowd3bda3878f7a9d01 at142058fb5.This identity is not inferred from a similarly named count Resource. Apply142059347/34a copies the same16B to its typed insertion source. The existing-resource branch1420593a3..3ac passes that source; the new-resource branch1420593d0..3d4 passes the same source to memcpy with the registered element size. Nothing reorders the two Entity words.

These identifiers are native evidence for logical ownership, not a claim that current Rust/Bevy runtime TypeId bits or native Entity ABI must be replicated. Current consumers should borrow the one actual typed Resource from this App and use its actual spawned Entities.

## Narrow production addition review

Root owns the legal `core-rocky-entities-20261018` claim. Current production was compared to original frozen147 candidate plus the two already reviewed cast_slice::<_,u8> test annotations: **the only additional difference is the eight-line TerrainEntities Resource declaration/comment**. It holds public floor:Entity and rocky:Entity, derives Resource/Clone/Copy/Debug, and has no Default. Existing loader, JSON/packing/factories/count API and regression inputs are untouched. `production-check.json` binds the checked source SHA.

This is sufficient positive type/field evidence for owner handoff. Original startup stage is Startup8 slot4, as the frozen147 handoff proves. Actual floor/rocky creation, resource insertion, use by rendering and GPU/runtime verification remain the startup/main-chain owner's work; this proof does not manufacture entities or declare those systems integrated.
