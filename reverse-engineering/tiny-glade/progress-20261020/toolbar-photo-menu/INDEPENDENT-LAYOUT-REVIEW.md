# Independent layout/locked-animation review

Read-only review by toolbar_photo_menu of toolbar_layout's production files.
Verified source hashes match the author's frozen delivery: toolbar_systems
0b17481d65ebea685683da375da28d28eac3fc3325f26c366c700f8bcfb6fd7e;
ui_interaction eabcdb0504f44491b3c8a45dc8f5b78558ddacd7fda489ceb3c08e84084df883.

- Inspected original auto-hide geometry 142343f2f..142343ffc and author native
  table fixtures. Independently executed actual-source-geometry.exe: 1600
  native cases / 6400 exact f32 comparisons plus 5 sound-edge cases passed.
- Inspected original sound branches 1423442cc..1423443a4: old equality rejects
  unordered and new positive/below-one comparisons use JBE to reject NaN.
  The production strict ordered comparisons match these instructions.
- Inspected native click producer and exact five-parameter PDB spelling.
  Independently re-executed locked-click-oracle.py against original B RNE:
  6 events consumed, only live-generation+UiLocked memberships yield the same
  3 deferred insertion commands in original order, repeats preserved.
- Inspected animation 14233ccf3..14233cd75: dt/duration precedes progress add,
  clamp retains NaN; angle=(progress*5-1)*PI, actual sinf/angle has no zero
  guard; state writes and Taffy scale multiplication agree. Completion is
  strict equality to 1 with unordered rejection.
- New real-system tests cover deferred restart after an existing animation
  tick, click-reader advancement, locked-only insertion, completion removal,
  original sinc singularity, owner-local hide progress, all debug extents and
  auto-hide-disabled behavior. These tests were inspected, not run here.

No new production discrepancy found in the changed scopes. The evidence does
not establish whole-toolbar host ordering or a full-game runtime pass; central
Cargo/ECS validation is still required.
