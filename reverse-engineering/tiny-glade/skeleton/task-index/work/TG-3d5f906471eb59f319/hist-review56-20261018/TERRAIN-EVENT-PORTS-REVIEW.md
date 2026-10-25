# B terrain batch 6b independent review

Reviewed the frozen 25-file event-ports package in TG-3d532bb8325ae1fe35/terrain-curve-20261018/event-ports. Each SHA-256 matches. Original function parameter identities and stack origins were read before the candidate implementations; this review does not infer types from code-folded grow_one symbols.

## Original observations

Independent read-only Unicorn replay reproduced all seven original resolved branch observations: begin emits CursorCapture(0), caches prior height/falloff/point and sets DecoratorArchivist.culling.allow_any; release emits CursorCapture(1) and Steepness(0), preserving the typed pending terrain edit; hovered terrain emits Terrain(1); highlight writes stroke17 only without unread MainCameraMoved; continuation writes the unit Arrow event, and writes Steepness(1) after the original accumulated horizontal threshold. Queues are preallocated. Native growth, GPU readback and global scheduler are outside these cases. The 447 instruction bytes in the hover evidence pack match B RNE exactly.

## Candidate assessment

The candidate uses the actual existing resources and typed event ports: WaterRaster gates water height, CursorTerrainRaycast supplies the raycast point, the existing archivist owns culling.allow_any, the original desired height is retained for PlayTerrainSound::CreateChunk, and committed draw emits Terrain(0) plus ToolWasJustUsed. ui_edit uses the real CursorCapture/EditTerrainStroke/Highlight/Terrain/Steepness/Arrow ports. It removes the previous WindowFocus/stairs/recalculate/UI-active misidentifications rather than retaining duplicate effects. Both hint events preserve the actual first-byte tag; no speculative payload bool is added.

The existing EditTerrainStroke consumer and terrain CPU edit order are reused. Onboarding consumers were independently recovered and supplied separately, including completion into GameProfile and WriteSettingsFileCmd. The Arrow event owner in this package is only its unit schema: the actual arrow_gizmo_animation consumer is a remaining separate integration requirement, not accepted here. AddTerrainHighlightCmd requires its existing type to derive Event; root is supplying that registration-level correction. Production startup GPU buffer allocation, actual frame provider lifecycle and whole-game scheduling remain unverified.

## Withdrawn earlier interpretations

The former stairs bucket interpretation is rejected: the complete draw SystemParam tuple identifies PlayTerrainSound. The previous UI-active interpretation of write +0x1e0 is rejected: argument 8 is DecoratorArchivist. The ui_edit release write1 is CursorCapture, not Terrain hint1. Source evidence is retained in STATE1-IDENTITY-CORRECTIONS.md; this review does not re-accept those earlier claims.

Verdict: the bounded original branch behavior and corrected port wiring pass this independent review. No production files were changed and no Rust compilation was run by this reviewer. See terrain-event-ports-independent-review.json for the exact native observations and frozen input manifest hash.
