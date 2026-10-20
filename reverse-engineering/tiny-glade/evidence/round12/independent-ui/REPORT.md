# Independent UI acceptance, original B

Scope: read-only audit of picker, UI state ownership, history permission and
ui_update. Production and author candidate files were not modified. No Cargo,
rustc, LLVM code generation or GPU work was started. T13 is excluded.

## Observed

All original execution reads the unmodified installation `tiny-glade.rne`,
SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
These are separately implemented machine-state harnesses in this directory;
they do not run the author's Python transition model or candidate Rust.

| Original code executed | Cases | Result and exact boundary |
| --- | ---: | --- |
| is_inside_bbx, 14123ffc0/44B | 85 | All four boundaries excluded; NaN false, signed zero and infinite/reversed/zero-size bounds match strict comparisons. No function calls or geometry substitutions. |
| history_start inline, 141734605..63f and 1414e182e..868 | 960 | Mode != 13, UI tag >= 2, phase == 3, captured Arc bit0 set. Stops before History method call. |
| get_ui_volume, 140b2dc60/503B | 5 | Actual original bucket key comparison and Arc strong count increment execute. Matching key returns cloned Arc; absent/mismatched key reaches unwrap panic. Panic unwind is not executed. |
| ui_update, 140b2de60/7577B | 77 | Selected old phases 0..4, held/released, same/different/absent hit, Escape, Shutdown and ordered focus events. Actual original get_ui_volume executes. No game state branch or lookup helper is substituted. |
| picker host, 140ac3f50/3455B | 39 | Independent machine state reproduces all frozen original fixture outputs across all eight volume tags, nearest/tied members and boundary/non-unit-ray cases. |
| retain and stable-sort region, 140838bfd..eec | 21 | Independent machine state reproduces all original fixtures: strict depth equality, bypass, negative permissions, ties/signed zero, canvas layers and nonidentity view. |

`verify_native.py` and `native-verification.json` retain exact span hashes and
fresh disassembly. get_ui_volume bridges only the standard keyed hash output
for a prepared collision bucket and parking_lot debug hooks. The miss proof
stops at panic entry; lock release through panic unwind is outside its scope.
Picker/arbitration bridge only standard allocation, Vec growth, frees and CRT
memory. Their fixture inputs/expected outputs come from the frozen author
corpus; the separately written harness establishes reproducibility, rather
than introducing a second geometry implementation as an oracle.

Inactive normal payload and six trailing padding bytes are excluded when
comparing native picker records; canvas owner pointers are normalized away.
All meaningful normal tags/values, ids, member indices, distances, positions,
occlusion and flag bytes remain exact. This normalization does not alter Rust
tests or original fixture files.

The independent selected ui_update matrix confirms:

* A forced end over the captured widget produces Finished phase1, including
  Escape/Shutdown/focus loss. Missing/different current hit produces Cancelled
  phase0. A force event does not imply unconditional cancellation.
* Last focus payload matters: [1,0] forces end; [0,1] keeps Held at phase4.
* Sound business values are six u32 words (24B), preserving the +4 word55 and
  all further words. The following 8B EventId is event-container bookkeeping.
* Each normal return clears the frame registry. The active Arc is separate.
* Both history-start copies load UiState+0x20 and test Arc+0xb9 bit0, rather
  than looking up the just-cleared registry. The no_history_recording original
  1409a8f18 clears params+0xa9 bit0; clearing canvas14092e0e2 writes0x0100 at
  params+a8. Thus flags1 permits recording. Flags0 disables it.

The narrow `ui-picker/is_inside_bbx.patch` is consistent with this independent
execution. Generic `Aabb2::contains` must retain its separate contract. The
`ui-update/recording-active-arc.patch` is required for real set6->set14 input:
registry-only permission loses the real press after clear_widgets.

## Inferred source assessment

The inspected candidate preserves all eight geometry variants, stable
distance/layer sorting, original depth margin1.0 and negative-distance bit2.
Its ui_update phase transitions and retained Arc design agree with the native
cases above. This is source review plus original behavior evidence, not a
passing Rust comparison claim. The previous author direct Rust fixture failure
at screen edge case21 remains a genuine failure until the narrow private-helper
fix is integrated and the actual candidate is rerun.

The author 750-case native matrix is useful but hooks the entire original
get_ui_volume helper. Our separate five-helper and77-state executions remove
that limitation for the selected cases. Neither matrix executes an operating
system event loop or audio playback.

## Runnable Rust acceptance candidate and unknowns

`permission-chain.tests.rs` is a separately authored integration fragment for
the bottom of the integrated ui_update module. It is rustfmt-parsed only and
has **not** been compiled or run. It registers an actual screen widget, uses
camera/cursor and digital action state through the real picker, then ui_update
and canonical history_start. It tests both allowed/forbidden recording after
registry clear and exact min/max edges. It never seeds UiInteraction, clicked,
used or calls replace_interaction. Only the central Cargo owner may run it.

The author's real-input-chain tests also use genuine picker input, but their
History is empty and no wall/roof authority or aux diff is mutated. Therefore
they establish a bounded UI/history lifecycle slice only. Remaining acceptance
must include nonempty original roof/wall authority edits, collision archival,
captured history aux, commit/cancel and real undo/redo through consumers. This
independent audit does not claim those chains, the complete set6 nine-system
registration, signifier creation/destruction, audio dispatch, OS input, full
App runtime or GPU completion. Shared App add_event maintenance and numbered
scheduling are audited separately by independent_flow_wiring.

Run with existing Python313 from repository root:

```
python reverse-engineering/tiny-glade/evidence/round12/independent-ui/verify_native.py
python reverse-engineering/tiny-glade/evidence/round12/independent-ui/verify_ui_matrix.py
python reverse-engineering/tiny-glade/evidence/round12/independent-ui/verify_arbitration.py
```

The latter imports verify_picker and executes the39 picker corpus as well as
the21 arbitration corpus. Imports also rerun the85+960+5 native helper checks;
these repeated runs must not be counted as new coverage.
