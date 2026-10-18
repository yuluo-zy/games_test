# Canonical terrain cursor plane and actual writers (B)

Original TerrainEditingData family TG-e73d16b0f4be6e76bf was free and legally claimed by terrain-curve-20261018; UI draw family TG-3d532bb8325ae1fe35 is already held by the same worker. Queue target_version C remains an inventory classification; this patch recovers current B only, with no C/A behavioral equivalence claim. No production sources/Cargo/shared owner files are modified by this worker.

## Current architecture, retained

The core ECS TerrainEditingData is already a transparent tuple wrapper around tg_terrain::stroke::TerrainEditingData, with Deref/DerefMut. The inner type is the single existing owner of actual strokes/next_index/rasterized. Add cursor_height_override:Option<f32> to that same authoritative inner type; all core Resource users can access it through the current Deref, preserving tuple constructors and avoiding two plane fields or a new Resource. Existing Clone/Debug/Default derive is reused (host initialization None); no native constructor layout/ABI is copied. No serde is added to this transient field.

All current crate definitions/constructions were searched with multiline matching. Only one production explicit inner struct literal exists, snapshot.rs::TerrainSectionJson::into_editing_data; integration-owner.patch adds None there. It is a necessary constructor hunk to its current owner, not a claim on that source family. All other current callers construct the inner through Default and the existing core tuple API. The core resource source is unchanged and retained as evidence of the canonical Deref ownership.

## Original writer proof and failure order

Reused the oschain original2844B ui_draw_terrain@140a9fdc0 pack rather than re-mining it. Whole original PE hash f507a6ce...d03, full raw function SHA3a5c4d4f...4d78d; this worker independently checked all688 contiguous instructions and every raw byte against the original B PE. Full exact SystemParam tuple is retained and binds the editing resource, drag local, onboarding events and audio type. No original dynamic execution/JIT.

- State1 calls original new_stroke@140aa00fe. EAX!=1 at0103/0106 branches to035a and bypasses the writer; current Result<Option> adaptation likewise writes only Ok(Some).
- On success,010f loads raw drag-distance bits from current CacheMouseRaycastHeight;0117 writes Option tag1;011d writes its f32 value into the same editing owner. First SSBO lookup is0133, and height clamp/write is later0511..0534. Candidate Some(draw_state.drag_distance) is immediately inside successful creation, before requiring assets or reading either SSBO. It is neither the clamped stroke height nor the +1.5 desired/audio height; -0 and NaN bits remain raw.
- State2 leaves this plane unchanged. It does not recompute it from each newly sampled cursor or the stroke height.
- State3 get_last_mut_stroke@02d9 returning null branches02e1 directly to06e7, then06eb writes Option tag0. Thus clearing is unconditional at the state3 tail, including missing-last-stroke, after any successful history/hint/unit release outputs. The original SystemParam tuple has NotifyHintSystemTerrain and NotifyHintSystemToolWasJustUsed writers; no FinishInputRecording writer. A folded grow_one alias listing FinishInputRecording is not evidence of a different event identity. Existing actual release events and history path stay intact.

## Regression source and validation

Extended the existing mapped3-frame World test to assert rawSome(-1) even though actual stroke height is0.5; continuation preserves it and valid release clears it. Added actual World/Schedule cases for raw -1/25/-0/NaN copied before a later missing-SSBO failure, rejected ID-limit creation preserving an existing plane, and release with no last stroke still clearing while publishing no fake history/hint outputs. This directly exercises the existing ui_draw_terrain system and canonical editing resource, not an author-written mirror of the state machine.

Three candidates parse via rustfmt without rewriting them. candidate.patch plus integration-owner.patch apply in isolation to recorded bases and reproduce all candidates exactly. Tests are authored but unexecuted here; no Cargo, no JIT, no app messages or source writes. Existing stroke creation, terrain height policy, SSBO providers, current typed history and event schemas are preserved.

## Integration boundary

Root applies candidate.patch (current stroke owner + owned ui_draw only) and gives/applies the one-literal integration-owner.patch to the existing snapshot owner. Source mapping/base hashes are in source-mapping.json. Actual camera/cursor consumers belong to their current owner and should borrow this same core resource field through Deref; this candidate does not create a second plane provider or edit host/input/save/thread/OS sources. This closes the real Some/None producers, not the entire terrain module or renderer lifecycle.
