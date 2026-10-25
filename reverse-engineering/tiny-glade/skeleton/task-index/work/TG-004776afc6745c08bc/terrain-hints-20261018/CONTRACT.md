# B terrain hint consumers, incremental recovery

Owner terrain-hints-20261018 holds the three original hint_terrain/hint_terrain_focus/
hint_terrain_steepness groups. Parent authorized these upper-consumer fixes to the
existing hand-written modules. Root alone integrates production and compiles. No
consumer architecture or entire module was recreated: preserve GameProfile,
condition resources and DisplayHintRequest; replace the constant event seam and
wrong Query<Entity> gate with the actual existing typed event and terrain hover.

Target B RNE f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03,
matching PDB2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911.
C remains a version boundary. Original full system tuples retained in
original-hint-param-identities.json identify the last parameter as
Res<HoveredTerrainStrokeId>, not an arbitrary scene entity query.

## Schema and precise behavior

Coordinate with the exclusive resources/onboarding owner terrain-curve-20261018:
NotifyHintSystemTerrain { tag:u8 } and NotifyHintSystemTerrainSteepness { tag:u8 }.
The original consumers read only the first payload byte. The suspected second
event byte is a state-resource byte, not event data. No boolean field is added.
Default0 is a host convenience if retained by that owner, not a native Default
claim. Do not derive extra meaning from EventInstance16-byte stride/padding.

- Layering0x142283cc0: tag0 alone increments count (native u64 wrapping INC).
  tag1 emits completion6 once if the condition's hint_done is false, then sets
  that state true. Other tags do not affect state. Display requires Terrain,
  profile.terrain_layering_completed false, count>=2, hovered.tag nonzero and
  state.hint_done false. The old every-event counter and seam0 prevented the
  real completion branch and changed the display threshold.
- Focus0x142283980: each tag2 sets hint_done and emits completion5, including
  repeated tag2 notifications; other tags do not affect it. Display requires
  Terrain, incomplete profile/state and hovered.tag nonzero.
- Steepness0x142283640: tag0 sets the first existing condition byte (hint_done,
  whose old API name is retained); every nonzero tag sets event_byte1 and emits
  completion4. Display requires Terrain, incomplete profile, first byte1,
  second byte0 and hovered.tag nonzero. The second byte is this state flag.

Native ui_draw release is a tag0 Terrain notification. Original ui_edit hover
has tag1 on valid hoveredID and cursor.tag==1 (0x140aef3b4..0x140aef401), not its
release port. ui_edit release's tag1 is CursorCapture; its steepness notify is
separate tag0 on release/tag1 after horizontal accumulation>0.3. Original
ui_edit_focus's terrain-selection branch writes Terrain tag2 at0x140977d60.
Earlier interpretations of the ui_edit release tag1 identity are withdrawn.

## Original validation and candidate regression

native_hint_terrain.py executes the three full original entry functions on42
tag-sequence/state cases (7 sequences ×2 initial states ×3 consumers), preserving
all event-reader/state/completion writes. Mode0 excludes the display backend.
The native outputs cover exact count/flags, ordered completion IDs and consumed
reader counts. Preallocated events and valid Res wrappers are supplied; only
allocator/CRT helpers are bridged. No Rust execution or game process is claimed.

native_hint_hover_gate.py adds9 original gate cases: hovered.tag0 versus1/2 for
each function, with all other display conditions ready. It stops immediately
after all original guards, before a display-builder/backend call. Only nonzero
hover passes. This proves the input identity and gate without claiming text/
binding helpers. Candidate regression uses the42 retained native state/output
observations plus a real ECS World with an unrelated entity: hover0 produces
no display, hover1 produces the existing three typed DisplayHintRequests.

Unicorn requires approved require_escalated JIT on this macOS host. Original
bytes are independently retained in each owned work directory. No compilation
was performed; root validates after applying all3 candidate.patch files and
the schema owner's event fields.

## Integration boundary

Current system-onboarding::configure initializes resources but ignores schedule;
no production caller or other registration of these three hint functions was
found in the current crate tree. registration-only.patch is a minimal shared
lib.rs integration hunk for root, adding actual typed event/hover registration
the3 consumers followed by the existing tutorial_completion using current Schedule. It does not claim original global
scheduling or a complete app plugin installation. Host must supply GameProfile
and AppMode; the chain runs after TerrainCpuStage::Changed. The registration
hunk reuses existing hints_startup once at configure time when profile is present
and none of the3 terrain states was supplied. Existing states survive later
configuration. tutorial_completion writes real GameProfile flags and the registered
WriteSettingsFileCmd for each completion. The renderer/display orchestrator is
not installed by this bounded CPU hunk. No unrelated onboarding systems are
silently claimed recovered by this hunk.

The original tag2 ui_edit_focus producer is independently identified, but its
entire selection-system body is absent from the current module (only existing
GeoSource conversion helpers are present). Its separate claim/work report
records that concrete producer gap. An inferred AppModeChanged proxy or orphan
helper would not close it and is not added. Original glyph/binding helper
behavior, queue-growth/error paths, full UI selection and full app scheduling
remain outside this bounded acceptance.
