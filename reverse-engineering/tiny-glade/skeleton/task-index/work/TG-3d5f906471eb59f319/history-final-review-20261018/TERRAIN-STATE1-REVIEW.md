# Independent B terrain state1 review, 2026-10-18

Reviewer: history_audit. Reviewed the frozen fourth-batch manifest at
TG-3d532bb8325ae1fe35/terrain-curve-20261018/initial-height/frozen-inputs.json
(all 15 fingerprints matched before execution). No production files were edited,
no Rust compilation or GPU execution was performed by this reviewer.

## Accepted bounded facts

1. Read original instructions before reviewing the candidate. The state1 branch
   0x140aa00e9..0x140aa076d and mapped constructor 0x14161cd80..0x14161d0e0
   match the patched B analysis input byte for byte. Oracle is B release RNE
   f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.
2. Independently replayed all 36 original initial-height fixtures: height bits,
   editing/UI-active writes, event counts and the native queue payload tag all
   matched exactly. A subsequent readback also records the complete five native
   words tag/XYZ/audio-height for all36 cases, exposing the candidate error below.
   Six additional signed-index -2 cases with rock false/true
   and drag -0/negative-infinity/positive-infinity also ran the original branch.
   They preserve active writes and the one tag5 notification. These are native
   instruction checks under the existing resolved new_stroke/world/allocator/CRT
   bridge, not an execution of the Rust helper.
3. Original optional mouse lookup, mapped first-element index@16 with -1 miss,
   bit-grid rock sampling, desired-height addition and clamp are consistent with
   the candidate. Existing index is every i32 except -1; absence/-1 selects
   minimum2.5, another index selects minimum0. Only existing index+not-rock adds
   1.5. NaN selects the finite minimum; the upper bound is20.
4. The typed provider reuses AssetSsboLibrary/MappedStorageBuffer and represents
   the observed one32-byte, alignment16 element. Native initializes leading16
   bytes to0 and index to-1. Its remaining12 bytes are padding, so the candidate's
   explicit zeroed reserved words are a sound upload representation without
   claiming their business meaning. Constructor usage0x22/memory2 matches current
   shared storage policy. The same typed read helper feeds hover and draw.
5. Existing register_terrain_events initializes TerrainEditUiState, so the new
   ResMut parameter has a registered owner. The current test-only helper supplies
   both buffers using TestDevice. The inspected production startup path constructs
   an empty AssetSsboLibrary; no production add_new for either typed mouse buffer
   or TerrainRockMaskSsbo was found. Actual GPU dispatch/submission and coherent
   mapped readback remain outside acceptance.

## Correction: event identity is not accepted as stairs

The old source and fourth-batch candidate call the native40-byte queue a
StairsPlacementBucket. This interpretation is wrong: a folded grow_one symbol
does not identify its element type. The full B FunctionSystem signature retained
in ../history-review-20261018/pending-terrain-type-evidence.json lists ui_draw's
last parameter as EventWriter<systems::audio::terrain_audio::PlayTerrainSound>.
The stack parameter at frame+0x160 is that last parameter. Native state1
0x140aa0738..0x140aa0769 writes a40-byte EventInstance with tag5, XYZ and height.
The 42 native cases prove these bytes/counts, not a stairs event or its consumer.

We explicitly withdraw any prior interpretation/acceptance of this queue as
stairs in the earlier independent review correspondence. The old fixture keys
placement_count and placement_tag are retained as historical filenames/labels;
their raw payload evidence remains valid. This correction requires the legitimate
terrain owner to route the event through the existing typed audio consumer.
Additional consumer fields must be recovered before filling them with guesses.

The candidate also places the clamped stroke height in the tag5 value field.
This is incorrect: original0x140aa0511..0x140aa0534 computes the clamped height in
xmm0 for the stroke, while0x140aa075c writes xmm7 to the event. xmm7 is the desired
height before clamp (drag, or drag+1.5 for existing+not-rock). Independent event
readback proves the difference: fixture0 has stroke2.5/audio-1; fixture28 has
stroke20/audio26.5; fixture29 has stroke0/audioNaN. The original36 author probes
only returned the tag and stroke height, so their success never validated this
field. This is a concrete blocking payload correction in addition to port identity.
The author and root received these exact original results before this report froze.

The same full signature also identifies the two state3 ports at frame+0x138 and
frame+0x140 as NotifyHintSystemTerrain and NotifyHintSystemToolWasJustUsed:
0x140aa064f..0x140aa06a6 writes16-byte EventInstance/tag0 to the former;
0x140aa06aa..0x140aa06e3 writes the unit8-byte event to the latter. The old
StairsPlacementBucket/tag0 and RecalculateHeightMapCmd binding is not accepted.
Owned points/height, pending-empty assertion and active-clear original facts are
unaffected. The author and integration owner have been informed before delivery.

## Upstream input boundary

The original ui_draw fourth parameter is Res<world_raster_defs::WaterRaster>;
0x140a9fe32..0x140a9fe9e selects its published raster and bilinearly samples it.
The previous/current host code uses TerrainHeightsData<TerrainHeightSsbo> at this
gate, while hover still has an explicit TerrainWaterSample adapter. The resolved
state1 fixture starts after that gate, so neither input substitution is proved
equivalent here. This is a concrete remaining input-owner gap, communicated to
the terrain owner for incremental reuse of current WaterRaster::as_ref().

## Disposition

Pass for the frozen stroke-height numerical branch, active writes and typed
mouse storage/provider adaptation. The frozen candidate's tag5 value and event
port do not pass review. Whole start/continuation/release consumer-chain
acceptance is withheld pending those corrections and the water-raster input.
Renderer initialization/readback lifecycle remains unverified.
This report does not supersede or erase the frozen fourth-batch artifact.

See review_terrain_initial.py and terrain-state1-independent-review.json for the
executed independent probe and exact fingerprints. macOS sandbox JIT is blocked;
the read-only oracle completed under approved require_escalated execution.
