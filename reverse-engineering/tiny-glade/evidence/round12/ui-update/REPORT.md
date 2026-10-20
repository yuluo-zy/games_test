# Original ui_update and lossless UI event candidate

Frozen evidence candidate; no production file or Cargo invocation. Original B SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. Exact original ui_update140b2de60 size7577 SHA256 78070209e56873b445bcecd65c7b1ca67fe64a8e82065a3beff6b3b264c5b5c2; get_ui_volume140b2dc60 size503 SHA256 b0bd8122e4c3ccc3a8f30e201794c1b784f8ffdea8efd624bb7b6ab8e9adf955. Reused exact machine bytes/Capstone from ../ui-picker. Cold unwind handlers are preserved in the native function boundary but are not extra game branches.

## Original contract

PDB generic of chain helper14175a380 fixes all8 params, in order:
1. ResMut UiState
2. Res ActionSetStates
3. EventReader WindowFocusEvent
4. EventReader EscapeMenuJustOpened
5. EventReader ShutdownGameCmd
6. EventWriter OverwriteBrushPreview
7. EventWriter OsCursorVisibilityCmd
8. EventWriter PlayUiSound

The original set6 tuple actually contains9 items: hover_intersection, two debug consumers, ui_update, and five signifier lifecycle consumers. Hover->update is a necessary slice, not a claim that the full set6 tuple is complete. Hover original fifth resource is MouseDepthValue and seventh Local<HoverIntersectionCache>; no guessed depth was used.

ui_update consumes all ShutdownGameCmd, all EscapeMenuJustOpened, then all WindowFocusEvent retaining only the last focus byte. It computes hover from the first sorted 56B result, except captured drag and the genuine native hover-false branch. Registry get_ui_volume returns/clones Arc<WidgetParams> and panics on absence. It does not compute world geometry. Captured interaction keeps its own strong Arc and 56B initial intersection; continuing frames prefer current registry params, falling back to the captured Arc when the widget temporarily disappears. A continuing drag does not overwrite its captured intersection with a new hover hit.

Digital slot0 is read at native owner+0 phase/+4 repeat timer. No physical mouse-button scan code is embedded here. Earlier chapter claims that 0x5c/0x2f are mouse key codes are incorrect: those literals occur in the tracing/file-path UTF-8 parser before the game state machine.

Phases are None2, Started3, Continuing4, Finished1, Cancelled0. A drag release OR Escape/Shutdown/focus-loss ends the drag; finish1 vs cancel0 is decided by current hit identity matching the captured widget. Thus force events do not unconditionally mean cancel0. Following frame an ended drag returns to hover/None; same-target hover reuses its captured Arc. Exact state machine and cursor/sound order are implemented in ui_update.rs.

Every normal frame emits a brush overwrite and cursor suppression/unsuppression origin1, then phase-specific sound, then clears the registry. Brush extra16B is an enum: tag0 None, tag1 kind with current cursor fallback, tag2 kind+offset. For tag2 the Sphere center, indexed SphereCloud center, or PlaneDisc point is added to offset and flag=true; other volume tags leave flag=false. SphereCloud out-of-range index preserves original panic. Native custom-cursor suppression tests intersection flag bit8 and slot0 pressed. All4 UiSounds24B slots are forwarded, including kind0 and the nested word at payload+4; hover0/start1/cancel2/finish3.

## Executed native checks

ui-update-oracle.py runs the unchanged original7577B function under existing Python313 Unicorn. Hooks bridge disabled tracing, standard Arc/map lookup/hash/clear/parking_lot hooks, and preallocated Bevy event storage. No game state branch is intercepted. All helper bytes remain the verified B original.

- native-state-fixtures.json:600 completed cases. Original old-state tags0/1/2 and valid phases0/1/2/3/4, missing/same/different hit, all5 digital phases, three event families and focus true/false. compare-model.py is an independent semantic model of the Rust candidate and compares tag/phase/widget/Arc identity, registry lookup sequence, and emitted sound variant order;0 mismatches. Uninitialized inactive enum payload is excluded from equality, because it is not a game value.
- native-preview-ownership-fixtures.json:150 completed additional cases. All8 original volume tags ×3 brush variants ×2 cursor flags ×hover/start/continuing contexts; verified original brush flag/kind/position and cursor origin/variant. Also registry replacement Arc, missing registry fallback, mixed focus event order, positive repeating timer and NaN timer. NaN input is serialized as its timer_bits.
- Real Rust candidates have only rustfmt parse verification. real-input-chain.tests.rs exercises actual registry→camera/mouse picker→ui_update→canonical history start/finish using genuine MainActionSet lifecycle values. It never calls replace_interaction and never seeds clicked/used state. These tests cover real selected screen geometry, pointer movement outside, captured Arc while registry absent, Escape/Shutdown/last-focus rules and sound+4 preservation. They are staged/unrun and do not claim OS event loop, full wall algorithm, audio playback or GPU acceptance.

## Required narrow migrations

recording-active-arc.patch fixes the historical consumer: native history reads captured Arc at UiState+20, not the just-cleared frame registry. Without this patch real UI6→history14 would never start. The canonical active owner has priority; registry fallback only preserves the preexisting explicit low-level UiState construction API. Original finish reads the same permission bit. This patch creates no second owner.

play-ui-sound.patch + sound-type-fragment.rs correct the single canonical PlayUiSound event to24B (kind u32 + remaining20B). Native EventId at EventInstance+24 is Bevy bookkeeping and is stripped. Both nested+4 word and remaining native payload survive roundtrip. The patch updates six existing source files by narrow hunks, with baseSHA and exact writer/reference counts in sound-writer-migration.json. No second event type is introduced, and no native FMOD/AudioCore implementation is replaced. No current source contains EventReader<PlayUiSound>; the real game-audio dispatch consumer is a separate remaining playback integration gap, not covered by this producer restoration.

Apply only narrow hunks against current source, keeping the roof-ui owner's independently extended WidgetParams and UiState fields. ui_update.rs requires that owner's registry_snapshot/hover_intersections/active_params/replace_interaction_full/clear_widgets and geometry APIs. Module registration is a central-owner change: add pub mod ui_update and register its actual system after hover in set6; register all3 input events, actual brush and cursor outputs, and corrected canonical PlayUiSound. Keep events owned by their existing producers and register once. Then wire recording14/34/35 from ../app-edit-recording. Full signifier set6 consumers and full wall splitter/merge remain explicit gaps.
