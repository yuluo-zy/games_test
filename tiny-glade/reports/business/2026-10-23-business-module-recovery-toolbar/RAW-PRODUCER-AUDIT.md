# Raw signifier producer boundary

Toolbar itself has no `UiAppendSignifier` Raw producer. Its only visual
signifier request is `ui_interaction/replay.rs::denied_input_signifier`, a real
owned `SignifierFactory::from_closure` containing the cross entity constructor.
Toolbar navigation, hiding, mouse arbitration and ordinary button clicks do
not themselves create a Raw request.

The 17-file import/reference search overcounts two files that do not directly
construct Raw records: decorator `ui_systems/mod.rs` is module documentation;
wall `circle/ui_create_circle.rs` documents UiAppendSignifier but constructs a
legacy non-callable factory instead. There are 15 direct Raw producer files:

| Later module | Files under `crates/` | Current producer boundary |
| --- | --- | --- |
| Terrain | `country-core/src/systems/terrain/ui_terrain_focus_mode.rs` | AppMode native tag5 and nonempty stroke state |
| Wall | `country-core/src/systems/wall/eraser/ui_bulldozer.rs` | Bulldozer mode plus ray-pick hit |
| Wall | `country-core/src/systems/wall/snapping/snapping_maintain.rs` | Active snapping state, mode greater than candidate |
| Wall | `country-core/src/systems/wall/snapping/telegraph_walls_are_merged.rs` | Active merged-wall preview timers |
| Wall | `country-core/src/systems/wall/shapes/shape_placement_signifiers.rs` | Shape placement preview geometry |
| Wall | `country-core/src/systems/wall/shapes/rectangle/signifier_edit_dims_line.rs` | Rectangle dimension preview |
| Wall | `country-core/src/systems/wall/shapes/circle/ui_edit_circle.rs` | Circle edit preview |
| Wall | `country-core/src/systems/wall/shapes/rectangle/ui_modes/edit_dims.rs` | Rectangle dimension edit |
| Wall | `country-core/src/systems/wall/shapes/circle/signifiers/edit_arrow.rs` | Circle arrow preview |
| Wall | `country-core/src/systems/wall/shapes/circle/signifiers/edit_line.rs` | Circle line preview |
| Decorator/stairs | `systems/decorator/src/ui_systems/ui_stairs_viz.rs` | Stair visualization/insertion |
| Decorator/stairs | `systems/decorator/src/ui_systems/ui_focus_stairs.rs` | Stair focus operation |
| Decorator | `systems/decorator/src/ui_systems/ui_change_chimney_height.rs` | Chimney height operation |
| Decorator | `systems/decorator/src/ui_systems/handle_splitting_roof_decorators.rs` | Roof decorator splitting |
| Decorator | `systems/decorator/src/ui_systems/handle_splitting_wall_decorators.rs` | Wall decorator splitting |

Three additional producers pass a `SignifierFactory::new` without an owned
callable closure. `wall/shapes/circle/ui_create_circle.rs` and
`wall/shapes/rectangle/ui_create_rectangle.rs` use addresses of local stack
environments; decorator `ui_systems/move_ops.rs` uses a temporary Arc payload
address and marker0. These are not valid original callable factories. Their
record shape must never justify dereferencing these pointers or invoking a
fabricated vtable. All three need real `from_closure` ownership during their
respective module restoration.

Current host mounts terrain focus and wall snapping/merge/bulldozer systems;
therefore unsupported requests can become reachable when those later business
states are exercised. Scope is not a reason to silently discard them or create
empty entities. Their completion condition is an owned original factory with
complete mesh/transform/animation/upload/lifecycle behavior, restored alongside
the owning module, before that module is accepted. The current toolbar work
does not relabel these old candidates as working producers.

The old Raw fields `{tag,kind,position,scale,name}` are a reconstruction, not an
original tagged union. Original create copies a 40-byte factory containing an
optional owned name and Arc callable, hashes `(WidgetId, Option<&str>)`, checks
the frame-start set of existing SignifierId hashes, and invokes the real
factory. FNV over tag/kind/name and spawning `SignifierId` alone cannot preserve
that algorithm, geometry or entity ownership. The prior Raw hash also omitted
position/scale, worsening collisions; changing its hash does not repair schema.

## Captured Arc ownership

Original create `140b2d4f9..514` takes exclusive Arc mutation: weak count must be
1 and strong count must be1, or `140b2d955` panics. It clones each stream
factory's String and callable Arc into the captured WidgetParams' factory Vec.
The Vec belongs to that captured widget owner and therefore persists correctly
through drag frames where a registration disappears. A later change of widget
or completed interaction releases that owner and its factories.

The current public `UiState::active_params()` clones the Arc. Calling it before
`Arc::get_mut` in the consumer introduces an extra strong owner and breaks the
original invariant. Use an internal mutable owner accessor without cloning, or
temporarily take and restore the Arc. Do not quietly substitute `Arc::make_mut`.
The picker currently clears `cache.widgets` immediately after joined picking;
`ui_update` then clears frame registrations. Thus its normal production path
can satisfy uniqueness before signifier_create, while diagnostic/test-held
strong or Weak handles should retain the explicit original failure behavior.

Original create's deduplication set contains only the frame-start Query of
existing entities, excluding deletion-marked ones. It does not insert newly
queued factory hashes into that set. Same-frame duplicate factories may wake
more than once; using `seen.insert(hash)` to suppress them changes behavior.
Original SignifierId is16 bytes containing hash and widget identity; retaining
only the hash loses the owner comparison used by parent-to-volume animation.
