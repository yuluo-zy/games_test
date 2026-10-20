# T16 original producer/lifecycle recovery checkpoint

B original RNE identity, exact function ranges and original jump-table bytes are
in `native-identity-and-tables.json`. Whole original executable SHA256 is
f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.

## Original evidence corrections

* Original ColorRadialMenu::create is 5531 bytes; the old1947-byte decompilation
  truncated two12-entry tables in rdata. `create.asm` covers the complete PDB
  range. `../render/radial-menu-jump-tables.json` gives both tables.
* Original show_color_options is74991 bytes, ending1422accdf. The retained
 267500-byte C export corrupts table boundaries. Its apparent cases0..38 are
  **not39 real cases**. Table14303ca70 has13 entries, immediately followed by
  the distinct selection table14303caa4 (13 entries), then hide's14303cad8
  table (13 entries). C cases0x1a onward incorrectly reach the hide function
 1422ae* outside show's original range. Do not implement those false cases.
* Main color_menu is26486 original bytes;136201 is decompiler text size. Its
 22-entry table14303ba18 is retained separately. Its tail is also misdecompiled
  (overlap at142296666/halt_baddata) and requires original-instruction repair.
* hide_color_menu is5058 original bytes, not the old2408-size annotation.
  HideColorMenuCmd is a boolean. True cancels previews by restoring saved
  values; false keeps preview values and closes. The latest command wins;
  ui_cancel18 is checked only when there are no pending commands.
* Autoclose checks the latest ModeJustChanged tool against12; any Escape open
  cancels; a cursor outside `icon_size*2+radius*1.2` cancels; the camera moving
  over10 units from its opening eye position cancels. The screen position and
  stored opening camera position are distinct fields.
* Show message is24 bytes: target16 and screen center8. Target contains kind0,
  a real subtype byte1, decorator id4 and wall id8. Current frozen message
  retains24-byte size/id/screen position but the subtype byte still needs an
  explicit field at the next coordinated checkpoint. It must not be dropped.

## Implemented production checkpoint

* Core icon identities now carry the original payloads: ColorIcon u32,
  WallStyleIcon u8, MagicColorIcon u32, PlasterPeelIcon f32, StairRailingIcon u8,
  brightness selector and12-byte FlatRoofStyleIcon, plus all actual markers.
* Radial owner stores icon entities, initial36-byte snapshot, target, screen
  center, radius, size, angle range, selection and opening camera position.
* `ColorRadialMenu::spawn` transcribes both native switches, all component
  inserts, initial selection, exact angle/radius/spacing math, separators,
  image layers, initial negative appearance delays, and ownership of both
  all-icons and interactive-only entity lists.
* Native selection index has default0 even when no initial value matches.
  Frozen typed owner currently represents that as Option; the main consumer
  must map None to0, or change the field to usize at the next checkpoint.
* hide sends all actual restoration channels and initial values (including
  wall style, flat rim, plaster, flat style, brightness, flags, chimney,
  window lights, magic and connected-stair color branches). Original does
  not have a railing cancellation writer here; none was invented.
* Menu entities despawn; icons only receive MarkedForDeletion and keep their
  alpha/extent for the already restored exit animation/deletion systems.
* Flag/Chimney event identities now alias the actual core events so producer,
  replay, consumer and history share one event type.
* Three canonical wall events were typed and handed to wall_gpu_chain for
  independent original consumer restoration. All from/to/history/audio
  fields are preserved, including two float values for plaster.

## Validation boundary and remaining chain

No Cargo command was run by this worker. rustfmt succeeded. Root owns independent
compilation and execution. Three bounded regressions were added: all native
icon variants/payloads/ownership;13 cancellation targets and lifetime; latest
false command overriding keyboard cancel. Passing those would validate this
bounded checkpoint, not the complete T16 player interaction.

The old show producer still calls its explicitly incomplete data-only create
surface, with old fake radius/selection gates. It is not connected to native
spawn yet. Main color_menu remains unrecovered. Cursor/wall/roof/decorator/gate
UI triggers also still contain stubs. No T16 completion claim is justified.
The next implementation must recover true13-way show inputs/options and its
five nested native spawn call sites, then the22-way main interaction and full
history/default customizations/write-settings side effects. `show-create-callers.asm`
retains all five exact argument projections. Avoid accepting the corrupt C
export's neighboring-function branches as game logic. T13 remains excluded.

## Show layout proven after the frozen source checkpoint

Original14229e9e0 reads GameConfig.ui.icon_size_mult. Main icon size is48*scale;
spacing is12*scale, halved only for target0. Radius is native MAXSS60 of
`icons.len()*(icon_size+spacing)/f32::from_bits(0x4071463b)`; this count includes
separators. Appearance divisor is1 for non-wall targets and1.5 for wall target0.
Original screen center is Show message+0x10/+0x14. Original eye_position is
copied to the stored opening camera Vec3; it is never the screen center.
Secondary rings use icon_size*0.8 and radius increment icon_size*1.1. Ring
angle constants have distinct original bit patterns0x3e7ae145 (flat roof ring),
0x3ec51eb7 (other secondary rings), not rounded decimal substitutes.
All constant bytes are in show-layout-constants.json. The five create call
sites pass the live GameConfig values and original owner snapshots; generic
radius1 and default selection cannot be accepted as a restored producer.

## Shared native UI follow-up ownership

The main icon widgets use native WidgetVolumeShape tag3 screen AABBs. Frozen
core currently has no full native volume picker; UiState::add only registers
parameters. roof_support_producer owns the coordinated candidate shared
WidgetGeometry::{TriangleMesh,ScreenAabb} API and real input/picking producer.
Color must consume that canonical picker and geometry, not introduce another
UI interaction resource or use replace_interaction as gameplay acceptance.
The native widget literal list is retained in main-widget-literals.json.


## Continued checkpoint: exact primary + snapshot policy, production still frozen

The main22 interpretation is superseded: there are two adjacent relative tables, 13 entries at14303ba18 and9 at14303ba4c. The latter is reached through `target-3 <=8` at142295e8d..eac. Adding the latter offsets to the former base produces instruction-interior fake targets; these are rejected in native-identity-and-tables.json. Main corrected overlay/export are main-corrected-overlay-revision.json and color_menu-analysis.c (117012 chars). Show two13-entry tables decompile to224536 chars; create19292, hide26983. All ranges match the retained original RNE byte hashes.

Actual original x64 execution now backs the option and selection candidates. Allocator/reallocator/free/memory-copy shims only replace generic runtime dependencies. The six resource-independent primary targets execute without game-resource hooks. Special/wall/flat inputs use explicitly recorded data dependency fixtures; those validate original construction policy, not production ECS lookup. `primary_options_original_comparison.rs` compiled the proposed Rust implementation independently and matched208 original fixtures. `initial_selection_original_comparison.rs` matched15 original36B snapshot fixtures. Exact names and meaningful per-kind payloads/layers are compared; native uninitialized padding is excluded, never interpreted as extra state.

The complete primary candidate preserves wall demo locks, shape<2 versus freehand branches, plaster peel eligibility, actual flat-rim ClutterVersion, original `plaster_colors` asset name, flat-roof floor/color inheritance, nine-entry demo roof palette, window sentinel999 and subtype gate, flag attachment exclusions, lantern brightness controls, chimney kind20 special form, four stair railing selectors, and all shared wooden palette cases. Snapshot candidate preserves default brightness1.0, wall freehand byte31, missing-extra zero fallbacks only where present in native code, full12B flat appearance and original missing flag/chimney defaults. No default entity or fake camera/screen producer is introduced.

GCC correctness issue escalated and assigned to save_migration_full by central: original windows is Option<ColorId>, plus original full JSON exposes stairs object, numeric flag color, plural fences and named enums. Original full serializer and field visitor were directly executed; all GCC evidence is available to that owner. The earlier narrow windows patch is superseded by that owner's full schema candidate.

Open integration: real source lookups and failure policy; secondary rings for halftimber/window light/stair magic/ladder; native navigation graph publication; full eleven widget handlers and thirteen+9 main target side effects; actual input->History->SSBO/GPU validation. These remain unclosed; passing pure producer fixtures is not reported as T16 completion.
