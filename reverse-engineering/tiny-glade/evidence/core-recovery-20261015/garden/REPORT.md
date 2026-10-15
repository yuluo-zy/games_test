# Garden CPU recovery — 2026-10-15

## Implemented

- `systems/garden`: real Bevy draw/record consumers, shared stroke slot 5, GardenRaster and mapped GardenRasterStagingSsbo; History::Garden uses the existing transaction owner. Local registration orders drawing before recording. Garden replay feedback returns to GardenDrawEvent (the original single replay callback).
- `resources/glade_border`: circle/rectangle point and shape containment. Rectangle border tests a circle through the original step-1 circle sampler; circle border uses strict containment.
- `systems/ui/resources/ui_state`: the required clearing-canvas registry (`RwLock<HashMap<WidgetId, Arc<...>>>`), string identity and active-widget drag query. Interaction is an explicit input to the owner, not a fabricated default.

## Evidence and corrections

- Original B RNE draw_garden `0x140ae4620`; `terrain-water/decomp/decomp-0x140ae4620.txt`; fresh machine code `../roofs-stairs/garden-draw.asm.txt`.
- Garden tag 17; Eraser tag 9 accepts layer 0 or 5. Eraser radius uses **unclamped** inverse_lerp(0.3,2.0) and `t*4+(1-t)` (`0x140c904a0`). Both branches assert finite radius before registering a widget.
- Draw boundary uses radius **(0.4 * brush) * 0.8**, not brush+1.5. The 1.5 constant is the pin request Y offset. Native 0.5 is stroke strength and separately pin-circle brush scaling.
- Drag phase defaults to 2 unless active tag>=2 and widget identity matches. `min(phase.wrapping_sub(2),3)` drives the actual begin/extend/commit transitions. Blocked extensions retain state 2; reentry resumes; phase 2 resets without committing. Inactive AppMode returns without altering state.
- `0x14092df50` hashes **`{name}_canvas`** (format suffix at `0x142aafe48`); `0x1405bdda0` uses std DefaultHasher/SipHasher13 and str terminator. Widget tag arguments are 5 (garden) / 9 (eraser), layer 3. Native return is `(UiState*, WidgetId)`; old decompiler's apparent name-pointer comparison is wrong.
- `0x141695e10` consumes all unread record commands and records slot 5 **once**; the generic existing recorder moves the actual Arc stroke into the canonical History.
- Border `0x140ad2950/0x140ad2970` dispatches to existing utils geometry; no replacement library implementation.
- Claude bounded state-transition analysis succeeded (`claude-draw.result.json`), with local provider/model inherited. Its table was checked against original branches; it is an aid, not primary evidence.

## Verification and limits

One ECS fixture covers Begin → blocked extension → reentry → real CPU pixel/staging mutation → Commit → canonical History, including no rerecord on the next frame. It uses the existing cfg(test) host-memory SSBO adapter; it makes no GPU claim. The integration owner reported this Garden fixture passing on 2026-10-15.

Not complete: full UI input arbitration/other WidgetParams variants and signifier owners; original global scheduling; full 56-byte ShowPinnedDecoClutter union and its renderer; downstream garden scatter/audio consumers. The new GardenPinnedClutterCircle is explicitly only the observed circle/height projection and must not be counted as the full pin system. UiState does not invent an initial interaction or claim native ABI completeness. Garden recording requires the real history input transaction to have been started by its owner.
