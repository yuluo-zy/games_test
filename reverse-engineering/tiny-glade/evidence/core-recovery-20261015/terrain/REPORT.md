# Terrain CPU consumers — 2026-10-15

Implemented `systems/terrain/{draw_terrain_stroke,edit_terrain_stroke,delayed_terrain_changed_msg}` with real Bevy resources/events and local ordered registration. `resources/terrain_editing` wraps the existing tg-terrain owner; edit arithmetic reuses `tg_terrain::edit::apply_resolved`.

Evidence: `evidence/terrain-editing/B-core/14094e000.c`, original `B-original-14094e000.asm.txt`, `B-core/1408592d0.c`, `B-dossier/dossier.json`; current `country-core-deep/terrain-water/decomp/decomp-0x140a03ba0.txt` and `decomp-0x140aabdd0.txt`. Existing SSBO size/memory evidence and recovered `read_terrain_height.cs.hlsl` confirm 0x20000 packed-u32 height elements.

Actual policies:

- Point replacement commands run before parameter edits. Old/new first-point indexing precedes mutation. Point audio magnitude is `delta * sqrt(z*z + (y*y + x*x)) / f32::from_bits(0x3a83126f)` (RNE 0x14094e6aa..6df).
- Missing stroke IDs **log and skip**, contrary to the old chapter's panic claim (0x14094e550..61a / 0x14094e9f0..eabc). Empty first-point indexing still panics.
- Every successful replacement/edit emits RecalculateHeightMapCmd and increments the shared ScheduledTerrainReadback counter. EvTerrainStrokeChanged carries a StrokeId and coalesces all unread changes into one recalc/increment/tag-0 audio event.
- RecalculateHeightMapCmd is proven ZST (event queues contain only the eight-byte Bevy ID); the old Gap payload was removed.
- Delayed readback uses one existing ScheduledTerrainReadback resource, two real mapped SSBO identities, two CPU TerrainHeightsData owners and a Local delay vector. Existing delays decrement first; the first expiration copies authority, emits OnTerrainChange regardless of pixel differences, then copies visual. Coincident expiration is coalesced. The issued-copy flag is taken afterward and pushes delay 2; zero entries are retained out.
- PlayTerrainSound tags 0..4 have typed observed payloads. Audio playback remains the audio owner's work.

Claude's bounded delay analysis (`claude-delay.result.json`) agreed with the directly checked control flow. No GPU fence, simulated GPU calculation, or default interaction was invented.

One ECS fixture checks ordered edits/notifications, missing-ID behavior, real CPU SSBO bytes and the two-frame delayed authority/visual copy. It uses the existing cfg(test) host mapping adapter. Central integration owner confirmed this fixture passed (1/1).

Remaining: terrain UI/new-stroke input producers, original plugin resource initialization and global scheduler, terrain history/replay lifecycle, audio playback, original GPU execution, and the complete downstream wall/collision/scatter subscription chain. These consumers do not make the full terrain module complete.
