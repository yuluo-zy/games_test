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

## 2026-10-30 current production integration (not accepted)

The current Rust producer now constructs real wall/roof/gate/halftimber and
decorator owners, with halftimber and window-light secondary rings. The main
13-target interaction loop and canonical UiNav publication are candidates;
real input/history/save/reload acceptance is still pending. Gate/decorator
pickers remain separately assigned, not inferred from menu construction.

Central independently checked original f507 RNE bytes for the rim icon:
1422937xx calls ClutterVersion::is_initial at140833230; bytes
`8039000f94c0c3` implement byte==0. Original rodata14303b8b8 contains the31-byte
`color_icon/freehand_ty_constant`, and14303b8d7 the30-byte
`color_icon/freehand_ty_terrain`. Rim preview/rollback must update the displayed
texture using those exact, intentionally freehand-named assets. The current
candidate now performs this update. This was static byte inspection; the
original executable was not run, and this is not a full main-loop equivalence
or runtime acceptance claim.

## Gate and decorator picker follow-up

Static f507 bytes at 1422b0e72 pass additional mask 0x3108 to
legacy_raycast_w_params (1408cc770), whose base mask is 0xf3. Tag 13/14
calls lookup_decorator_info at 1422b0f1d and returns on a missing value at
1422b0f25; lookup_storage at 1422b0f3d returns niche 3, rejected at
1422b1240. Tag 4 passes the hit's storage address and decorator id to
140a1c5f0 at 1422b104f; missing info returns at 1422b1057.

The stair menu producer separately reads PlayermadeStairsAssemblyRequests:
1422b0f58 constructs the normalized edge key using 140959e70; 1422b0f88
accesses the map at owner +0x198, and 1422b12a2..12b2 tests whether the last
four-byte record has kind 3. Tag 4 repeats this lookup for graph neighbors
and ORs the results. This is distinct from the show consumer's visual-state
lookup. These findings are static evidence, not runtime acceptance.

## Main preview, defaults and history audit

Corrected 13+9 control flow clears retained color for selected flag and smoke,
then applies a color selected during the same traversal; selected wall style
clears only if no color was selected (142294f00..4f60). Peel clears on hover
or commit, while flat-style hover does not clear it. The wall-style branch
does not suppress the color command's custom flag; brightness suppresses it
only while a brightness value is selected. Flag/smoke do suppress it while
selected. These conditions are corrected in the production candidate.

Brightness default is written by its icon handler before the action-19
try_start_input_recording call (1422941f2 versus 1422947f0); failure clears
commit flags but does not revert that already-written default. Other color,
style, freehand, rim, railing and magic defaults are written after the try
and their native commit/change gates. Window-light and ladder defaults require
both commit and a changed value; railing and magic defaults likewise require a
changed committed value. Building, roof, gate, fence, halftimber and ordinary
decorator defaults use commit; window sentinel/subtype selection is preserved.

Style, peel and freehand commands carry native history flags and their wall
consumer records the edits; the menu must not add a second copy. Rim similarly
delegates history to the roof consumer. Current runtime has a separate confirmed
ordering gap: rim command consumption is in GameSystemSet(40), while deferred
roof history recording is set 0x22 and finish is set 35. A rim record produced
after finish cannot join that same transaction; its registration point remains
unlocated and requires coordinated correction, not a second menu-side record.

The original owner is now identified: PDB signature and decompilation of
system_color::set_roof_customization at 142299dd0 include both flat-style and
rim event readers, RecalculateWallHoles writer, and History. Its rim loop
compares roof+0x3d, emits holes only on change, assigns the new byte, and calls
History::add directly when command byte+0x12 equals 1, preserving the command's
from and to bytes. The original color tuple registers this owner in set 0x1f
after color_menu and before history finish in set 0x23. Production now consumes
both commands in that owner; duplicate set-40 roof adapters were removed.
The real pointer-to-rim, transaction, undo and redo fixture is present but has
not been run by this worker.

## Decorator placement registration follow-up

The f507 plugin body at 140342c10 (PDB range 10365 bytes, SHA256
aa0da87bbbca08860afd0e49979efed71176e157dfcd54b26360bc239a5e6729)
constructs dispatch via 1402d4810 and placement via 140553030 at
140344e32/e3e, aggregates 19 entries via 1400d52f0, and assigns set 0x19.
Constructor 140553030 names move, focus-stairs, place, rotate, chimney-height,
stair-height, inactive and external-command leaves. Direct constructors in
1400d52f0 name derive, clear-blueprints, the 15-generator group, instantiate
colliders, collision flush, validate, cull, maintain-stair-graph,
cull-stair-attached, and backup. The generator group's original name literals
include cottage/gothic windows and doors, dormers, lanterns, chimneys, flags,
arrow slits, trap doors, clutter and playermade stairs. The deferred decorator
history reader is assigned set 0x22 by 140258060. These are static findings;
the new App registration and input fixture are not yet compiled or run.

The slot1 input placement was subsequently corrected by following the config
copy: 140344e32/e3e produce the two configs copied to 14026c790's input;
its PDB generic and config order are convert-autoclutter, dispatch group,
placement group, continue-stairs. 140105360 independently proves slot0
ends with a cache invocation; parent slot2 and slot3 tail each have their own
cache invocations. The candidate registration needs to preserve all three.
The dispatch group also contains debug_spawn_all_clutter ahead of dispatch;
its current host implementation uses a separate debug counter and event owner,
which requires adjudication before mounting it as recovered production logic.

For wall-hole frame advancement, the actual clear_out_wall_holes leaf
1407f98c0 has wrappers 1402943b0 / 140431b10 in vtable 1429ab780;
1402957c0 constructs that system. Plugin bytes at 140344e0f..e12 pass
GameSystemSet 0x0a to that constructor. This is independent of the set25
visual-entity clear and must be registered once in set10.

The set10 wall-hole leaf and slot1's convert → dispatch → placement → continue
order are now applied in the App candidate. The three cache instances and the
single set25 unleash are retained. The original debug-spawn leaf is not mounted:
the current Rust implementation introduces an unconfirmed counter and local
event owner. The current focus-stairs and stairs-visualization leaves likewise
remain unmounted because they fabricate widget geometry or signifier positions;
their real owners and behavior must be restored before claiming the full tuple.
PrevWallHoles is supplied by startup_construction_runtime, and dormer hole SSBOs
are created by init_populate_ssbo_library. No runtime success is inferred.

### Deferred decorator history panic prefix (original f507)

Original 140817880/885 reads and clears pending; 14081789a/89e loads the record Vec pointer and length, and 1408178a2 zeroes its length without replacing pointer or capacity. On unwind, 140817a1c calls 140d24ac0 for the drain fields; its tail_len is zero and it does not free the original Vec allocation. Records are Copy and have no element destructor. The potentially surviving decorator set is cleared only after the History::add loop returns; it remains populated when the missing-recording panic interrupts that loop. History::add 140a84ef0..ef7 detects the absent pending group and 140a8525b..2a0 enters panic. The production regression checks the cleared flag and length, preserved Vec pointer/capacity, and preserved set. This is static original-byte evidence; the regression still needs central execution.

### Wall shape creation without an active history recording (original f507)

Rectangle 140a9448f and circle 14097f134 call the new-wall history owner 140a83300, which calls History::add at 140a83915. Subsequent rectangle History::add calls are 140a94535 and 140a94637; circle calls are 14097f1d2 and 14097f2d7. History::add 140a84ee9..ef7 detects the absent pending group and 140a8525b..2a0 invokes the panic path. The wall authority and roof request precede this first history call; later toolbar and sheep requests do not. The corresponding production fix and fixture remain subject to central build and execution.
