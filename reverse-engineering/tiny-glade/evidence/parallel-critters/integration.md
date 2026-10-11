# P02 integration — preliminary, 2026-10-11

Only critter-owned directories are modified. No shared workspace integration performed.

## Version pins from actual original lockfiles

B system-critters 0.1.0 → country-core 0.2.0, utils 0.1.0; direct public dependencies include Bevy App/ECS/Input/State 0.16.0, glam 0.29.3, fastrand 1.8.0, dolly, itertools 0.14.0, lerp, perchance, parking_lot 0.12.1, RON fork. C system-critters 0.1.0 changes Bevy to 0.17.3, glam to 0.30.10, parking_lot to 0.12.5 and RON to 0.12.0. No C behavior equivalence asserted. A has no Cargo.lock.

## Shared boundaries under investigation

Original source paths exist for ducks_react_to_being_inside_buildings, sheep/sheep_react_to_being_inside_buildings, sheep/sheep_terrain. These are source metadata, not yet caller/producer proof. Need actual country-core terrain, water, building enclosure query interfaces, time, RNG/resource ownership, animation resource IDs and history/replay consumer chain. Do not implement duplicate terrain/water/building logic or supply hand-filled fixtures as actual producer integration.

## Exports and adapters

Pending evidence; no invented public type/default or scheduling edge. Coordinator will integrate shared exports once exact recovered signatures and producers are available. A standalone harness, if built, uses [workspace] isolation and independent target/output.
