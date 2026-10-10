# Original Tiny Glade PDB structure pass

This pass studies only the supplied original-game artifacts. No prototype was inspected or mapped; no target was executed or modified. No Ghidra session was opened by this worker.

## Exact compiled boundaries

`procedures.json` records 86 original-game procedure candidates with PDB record lines, sizes, addresses and exclusive ends. Each boundary below is independently corroborated by `S_GPROC32` and the section-contribution record in `contributions.json`.

| Original routine | Entry VA | Bytes | Exclusive end | PDB evidence |
|---|---|---:|---|---|
| `system_roof::create_roof::create_roof` | `0x1421aaea0` | 2075 | `0x1421ab6bb` | `roof-create-module3467.txt:57` |
| `country_core::resources::history::History::finish_potential_mutation` | `0x140a86980` | 420 | `0x140a86b24` | `history-module555.txt`, full machine index |
| `system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns` | `0x141fe3240` | 848 | `0x141fe3590` | `wall-constructor-module3041.txt:1746` |
| `system_decorator::decorator_visual::door_stairs_assemble_bricks::construct_door_stairs<...>` | `0x14200c7f0` | 4668 | `0x14200da2c` | `decorator-stairs-module3088.txt:59` |
| `system_roof::visual::generate_roof_stone_floor_and_roof_bottom::generate_roof_stone_floor_and_roof_bottom` | `0x1421a6d40` | 840 | `0x1421a7088` | `roof-nearby-module3463.txt:72` |
| `system_roof::update_roof_spatial::update_roof_spatial` | `0x1421a7710` | 1459 | `0x1421a7cc3` | `roof-nearby-module3465.txt:68` |
| `country_core::resources::window::WindowSize::aspect_ratio` | `0x140949c20` | 20 | `0x140949c34` | `window-module512.txt:3356` |

Full function disassembly is retained for roof creation, history mutation completion and window aspect ratio. Sizes describe compiled code ranges; they do not restore original Rust function arguments or include all inlined routines as separate functions.

## Actual static call edges

`roof-call-candidates.json` maps all 26 `callq` sites in the exact roof-create range; `history-call-candidates.json` maps all six in the exact history-finish range. Matching uses the original PDB's 101,405 public records and exact section-relative addresses. Only eight aliases are retained per destination, prioritizing game names, and original alias counts are preserved.

Roof creation has direct calls to the addresses labeled `PublicWalls::get`, `PublicWallState::max_y`, `Roof::new`, `Roof::ty`, `Roof::is_gable`, `ShapeParameters::center` and `Commands::spawn`, as well as allocation/panic/drop helpers. Address `0x1408c5670` carries both `WallStyle::gable_roof_style` and `WallStyle::is_halftimbered`; that call has ambiguous symbol semantics. Static sites include branches and unwind/cleanup paths. They are not a runtime sequence or a proof of a geometry algorithm.

History completion's direct calls include Arc drop helpers, a thread-local helper, panic helpers and a function labeled `drop_in_place<HistoryMutationToken>`. It does not establish the entire undo/redo architecture by itself.

## Recoverable data and missing layouts

`type-layout-coverage.json` records the search boundary and explicit unknowns. In the full previously exported 6,125-record TPI stream, no `country_core`, `system_roof`, `system_decorator` or `system_wall_constructor` type name is present. The lone substring `WindowSize` is unrelated C member `maxWindowSize` (`pdb-types.txt:7721`), not the game type.

Sampled original modules expose meaningful type names in compiled generic signatures, including `EditType`, `WallHistoryEdit`, `RoofEdit`, `DecoratorHistoryEdit`, `TerrainEdit`, `HistoryMutationToken`, `Roof`, `PublicWalls`, `PublicWallState` and `WindowSize`. Their field names, offsets, enum discriminants and complete sizes are **not recovered** from these names. The five initial sampled modules have zero `S_LOCAL` and zero `S_GDATA32/S_LDATA32` records. Procedure type `0x1001` is the generic CodeView `void()` placeholder, not a trustworthy recovered Rust ABI signature.

One function gives direct bounded layout evidence: the complete 20-byte `WindowSize::aspect_ratio` body reads four bytes at `[RCX+0]`, zero-extends the result, converts it to float32, reads four bytes at `[RCX+4]` and performs the same conversion, divides the first float by the second, then returns. Thus `float32(u32_at_offset_0) / float32(u32_at_offset_4)` is mechanically established, with no denominator check in this body. Field names and total structure size remain unknown; two observed four-byte reads only prove that readable span.

## Original source-path metadata

PDB retains paths and source hashes, not source contents. Useful observed paths include:

- `crates/systems/roof/src/create_roof.rs` and `country-core/src/systems/wall/valid_enclosure/mod.rs` (`roof-create-module3467.txt:13-14`).
- `crates/systems/roof/src/visual/generate_roof_stone_floor_and_roof_bottom.rs` (`roof-nearby-module3463.txt:14`).
- `crates/systems/roof/src/update_roof_spatial.rs` and `country-core/src/resources/roofs/roof_shape.rs` (`roof-nearby-module3465.txt:13,47`).
- `crates/systems/wall-constructor/src/construct_elevation_supports/util_pillar_construction.rs` (`wall-constructor-module3041.txt:13`).
- `crates/systems/decorator/src/decorator_visual/door_stairs_assemble_bricks.rs` (`decorator-stairs-module3088.txt:13`).
- `country-core/src/resources/history/wall_compound_edit.rs` (`history-module555.txt:44`).
- `country-core/src/resources/window.rs` (`window-module512.txt:63`).

Next native analysis can use these exact boundaries and call destinations. Data layouts beyond the bounded aspect-ratio reads require operand/data-flow recovery, caller corroboration and evidence-specific validation; they should remain unknown until then.
