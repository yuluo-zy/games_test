# Static binary / PDB baseline

Target: `D:\game\ljxsj_92385\Tiny Glade`. The target was never executed or modified. Machine-readable identity and limitations: `baseline.json`. Reproducible refresh script: `../../scripts/binary_baseline/inspect_baseline.py`.

## Identity

- Fresh SHA-256 of the EXE, RNE, PDB and both Steam API files equals the earlier `D:\game\TinyGlade_逆向初查\initial-analysis.json` identity. Prior BLAKE3 values are therefore retained with an explicit reuse method; BLAKE3 was not independently recomputed in this pass.
- EXE, RNE and PDB share GUID `{09F1A02A-49F2-F8C8-4C4C-44205044422E}` and age `1`. LLVM refresh evidence: `tiny-glade.exe.headers-debug.txt`, `tiny-glade.rne.headers-debug.txt`, `pdb-summary.txt`.
- RNE matches the supplied manifest's `tiny-glade.exe` digest; EXE does not. The two equal-size 58,460,160-byte files differ in exactly six bytes, one `.text` range at file offset `0x172f843`. This establishes a modified code range, but neither its purpose nor author. Prefer the manifest-matching RNE as the static code baseline; record the EXE separately for later user-visible behavior.
- PDB is 274,710,528 bytes, non-stripped, with debug, types, globals and publics. Public records: 101,405. These are symbol records, not a count of original game functions.
- Supplied manifest consistency is not independent verification of official origin.

## Observed entry points for the plan

All names below describe observed symbols, **not established algorithms or runtime sequencing**. The simplified Rust paths are transcribed from mangled public names. Symbol identity/address confidence is high; relevance to a feature is a hypothesis until callers, operands, resources and runtime results are checked.

| Investigation question | Observed symbol / module | Evidence location | Address |
|---|---|---|---|
| Roof regeneration | `system_roof::create_roof::create_roof` | Prior `initial-analysis.json:89`, PDB public record 1187028 | VA `0x1421aaea0` |
| Visual roof shape | `country_core::resources::roofs::roof_visual_shape::<Roof>::get_roof_shape` | Prior `initial-analysis.json:101`, public record 3440900 | section 1 offset 9320736 |
| Height edits | `country_core::systems::wall::shapes::calc_shape_height::calc_shape_height` | Prior `initial-analysis.json:111`, public record 3369864 | section 1 offset 11301088 |
| Freehand input | `country_core::systems::ui_edit_freehand_walls::ui_edit_freehand_walls` | Prior `initial-analysis.json:115`, public record 3342348 | section 1 offset 11307232 |
| Brick support generation | `system_wall_constructor::construct_elevation_supports::util_pillar_construction::construct_brick_columns` | Prior `initial-analysis.json:161`, public record 4050596 | section 1 offset 33432128 |
| Edit transaction end | `country_core::resources::history::History::finish_potential_mutation` | `focused-symbols.json:40`, public record 3506068 | VA `0x140a86980` |
| Input recording | `country_core::resources::history::History::start_input_recording` | `focused-symbols.json:75`, public record 3505544 | VA `0x140a86270` |
| Decorator edit diff | `country_core::resources::history::History::add_decorator_diff` | `focused-symbols.json:82`, public record 3505244 | VA `0x140a854a0` |
| Session metadata read | `country_core::resources::session::SessionMeta::try_load_from_path` | `focused-symbols.json:98`, public record 3509892 | VA `0x140b46480` |
| Session metadata write | `country_core::systems::save::save_game::save_session_meta_to_file` | `focused-symbols.json:154`, public record 3357580 | VA `0x1417522e0` |
| Wall GPU draw lists | `country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_wall_draw_lists_pass2` | `focused-symbols.json:205`, public record 3328540 | VA `0x14098b2a0` |
| Brick drawing | `country_core::systems::render_loop::draw_passes::draw_shaders::draw_bricks::indirect_draw_bricks` | `focused-symbols.json:233`, public record 3314844 | VA `0x1408a53d0` |
| Roof compute pass | `country_core::systems::render_loop::compute_passes::roof_tiles_gravity::roof_tiles_gravity_compute` | `focused-symbols.json:240`, public record 3324884 | VA `0x140a29b10` |
| Shape overlap candidate | `country_core::systems::collision::world::RaycastWorld::iter_shape_overlaps` | `focused-symbols.json:326`, public record 3399192 | VA `0x140254890` |
| Decorator overlaps | `country_core::systems::collision::world::RaycastWorld::decorator_overlaps_with_others` | `focused-symbols.json:340`, public record 3399980 | VA `0x14129be00` |

Compiler module index corroborates actual compilands: `system_roof` module 270 at `pdb-modules.txt:1085`; `system_wall_constructor` module 388 at `:1557`; `system_decorator` module 415 at `:1665`; `country_core` module 474 at `:1901`. PDB module names/paths do not provide those source contents.

Bounded source-file metadata in `roof-module270-files.txt:6` records original `crates/systems/roof/src/delete_roof.rs` with a source hash; `:7` records `bevy_ecs-0.16.0`. Module 271 at `roof-module271-files.txt:12` records `crates/systems/roof/src/lib.rs`. This aids locating related routines and matching dependency versions, without recovering source text.

## Priority and unknowns

1. Follow the original game's drag height/footprint input → history edit boundary → dependent roof/wall rebuild → overlap/decorator adjustment → drawing. Trace actual calls and data changes; do not invent a graph merely from these names.
2. Use targeted PDB symbols to seed a bounded native analysis rather than decompiling the entire 58 MB image. Reuse the earlier roof-entry disassembly, whose 192-byte range did not establish the complete function.
3. Make persistence a separate evidence task: session metadata write is only one candidate and does not establish world save format, compression, full scene serialization or compatibility.
4. Rendering can be investigated alongside packaged SPIR-V and resource schemas. Function labels such as `roof_tiles_gravity_compute` do not prove a physical simulation.

Unresolved: complete game Rust layouts, scheduling/dependency ordering, geometry algorithms, spatial invalidation mechanism, persistence format, runtime behavior and performance. Earlier TPI export has 6,125 records and bounded searches for four game type/module names did not find them; this neither recovers complete layouts nor proves their absence throughout all debug information. Build metadata says Rust 1.86 / LLVM 19.1.7, O3 with debug; optimized/inlined native recovery remains difficult.
