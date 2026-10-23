# Independent toolbar evidence, 2026-10-23

`native_nav_trees.py` reuses the existing `phase2-toolbar-focus/native_focus_world.py`
native Bevy query harness against the local shipped EXE. Run:

```sh
env UV_CACHE_DIR=/tmp/toolbar-evidence-uv uv run --offline --with unicorn --with pefile --with capstone python3 tiny-glade/reports/business-module-recovery-2026-10-23-toolbar/native_nav_trees.py
```

The recorded run executed 20 nonempty cases, all matching their explicit expected
node. Navigation `14233eed0` (6017 bytes), subtree `141ea8c60` (1128 bytes),
query get/iteration, native maps/sets, anchor traversal, distance comparisons and
focus writes execute the unchanged machine code. Bridges cover Taffy `children`
and `style` dependency getters, standard allocation/memory and Windows TLS.
This verifies native business control flow over supplied trees, not native Taffy
storage, actual OS input, production Rust, or rendering. The Rust fixture consumer
must independently construct real Taffy trees and compare to `native_selected`.

Cases cover sibling root priority, nearest within one root, equal distance ties,
hidden candidate eligibility, reversed left siblings, no forward candidate,
entering and returning all four anchor directions, wrong return direction,
sibling priority over a closer anchor, nested ascent, flex orientation gating,
and anchor-cycle termination via the original visited set.

The local full EXE hash is `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`,
different from the old Windows report's full EXE hash. Both relevant function
bodies exactly match the old original-body hashes; the script asserts these
before execution and records them in `native_nav_trees.json`.

Lifecycle audit found no original direct-Reload reset producer. Original
`add_ui140945c60` registers its Update/set6 chain without a Game condition;
resources are inserted at application setup. The `ui_update` parameter list
contains Escape, Shutdown and WindowFocus events, but no Reload event. Its
captured Arc intentionally survives absent widget registration while input is
held. PDB FunctionSystem symbol scanning found only hover, ui_update,
signifier_create and MEI pre_update as mutable UiState system owners.
Thus the new host OnExit cleanup is an explicit host lifecycle repair required
for direct Reload; it must not be presented as an original reset algorithm.

Static navigation review matched the current implementation's query filters,
direction slots, parent orientation, sibling slicing, anchor directions,
strict floating comparisons, per-root best reset, generation niche and visited
set. The tag2 original same-focus write/changed-tick difference was reported and
the implementation owner corrected it.

At audit time the host lacked the five original set6 signifier lifecycle
consumers. Replay's deny factory registration therefore did not prove a visible
cross or stream reclamation. This was reported to central as a current module
blocker, not deferred joint acceptance work.
