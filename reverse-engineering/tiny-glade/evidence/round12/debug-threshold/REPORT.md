# Complete debug threshold producer and real sphere submission candidate

Original identity is untouched B RNE SHA
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
`production-review.patch` is a five-file narrow candidate. No production edits,
Cargo runs, fake meshes, default-camera values, empty callbacks or T13 work.

## Original complete policies restored

- `debug_viz_threshold_curve140a8b0c0/1129B`, SHA
  `8df83c2b8a8a8635145e3ae2f0ff859ee9bfb44258959ee9d6813332f0a91212`:
  requires DebugGui bytef3/show_threshold_curve; visits every public wall and
  performs its real valid-source-curve assertion, including empty prepared
  arrays. Zips prepared positions(+90/+98), prepared threshold values(+c8/+d0),
  arch heights(+e0/+e8), independent third heights(+f8/+100), truncating at the
  shortest of all four arrays. Emits four spheres per point in original order:
  white original position/radius.1/width1; red threshold-relative/radius.05/
  width2; red independent-third-relative/radius.15/width.5; blue absolute arch
  height expressed by the original subtract-then-add/radius.15/width.5.
  Original multiply-by-zero operations are retained: they affect NaN/Inf and
  signed-zero semantics. Current owning prepared/cache arrays are reused.
- `DebugSphere::show140b02520/676B`: all six colors resolve their exact original
  mesh name; identity is the actual original Debug string, including original
  field labels `position`, `line_width`, `filled_in` and Custom([r,g,b]). The
  current public typed fields center/alpha/solid are retained for callers.
  A real owned closure factory enters the canonical UiSignifierStream.
- Sphere factory `140b027d0/1585B` plus actual
  `SignifierEntityBuilder::build14089dd50/3840B` named-mesh branch: required mesh
  lookup, required vertex_color_shader lookup, real DrawableMeshBundle and
  SignifierAnimation spawn, then UiPass and DontCastShadow, then DrawTopology
  tag1/native line_width only when not filled. Returns that real entity to the
  existing signifier consumer for its actual identity/lifetime assignment.

The existing general SignifierEntityBuilder::build is a known empty
spawn+animation shell, so this candidate does not call it. The sphere factory
directly implements its complete original named-mesh branch using the current
canonical owners/components. Other builder variants are left unchanged; this
does not claim the general builder recovered. Native missing-asset paths fail;
the Rust required lookups also fail rather than making invisible entities.

## Actual execution evidence

- `native_threshold_policy.py` directly executes the whole original1129B with
  original hashbrown iteration/probing/validation and float instructions. Only
  the standard hash result and final Sphere show destination are recording
  boundaries. `native-threshold-fixtures.json` has72 cases, including zero and
  mismatched array lengths, disabled/enabled debug flag, signed zeros, negative
  heights, NaN and ±Inf.
- `leaf_validation.rs` includes the exact proposed production policy and reuses
  actual locked glam0.29.3. Standalone rustc execution **passed72 original
  cases/144 emitted spheres, every semantic bit equal**. It does not compile or
  test the Bevy adapter. Output: `leaf-validation-execution.txt`.
- `native_sphere_show.py` executes the complete original676B and its original
  Debug/std formatting machinery. Generic allocator/CRT and final append-stream
  destination are bridged. All six original strings and captured environment
  fields are in `native-sphere-show-fixtures.json`.
- `name_validation.rs` extracts the exact proposed production identity method;
  standalone Rust execution **passed all6 original names**, including Custom.
  Output: `name-validation-execution.txt`.
- `native_sphere_closure.py` executes the entire original1585B closure with the
  builder destination recorded. `native_sphere_full_build.py` then executes the
  original closure **and the real original3840B builder**, bridging only generic
  memory/profiler, named asset results and Bevy reservation. Native output is
  152-byte drawable+animation command,32-byte UiPass,32-byte DontCastShadow,
  optional40-byte topology, returning one real entity. Records are in
  `native-sphere-full-build-fixtures.json`. PDB executor identities independently
  confirm `1412db690`, `1412d96a0`, `1412d8160`, `1412dc8f0`.
- Required valid-source-curve assertion and native missing mesh/shader branches
  were inspected in complete original assembly. They were not exercised by a
  native panic/unwind harness.
- `git apply --check` and rustfmt syntax parsing passed at candidate generation.
  Full Bevy compilation/ECS execution remains the central owner's validation.

## Actual assembly boundaries still requiring independent closure

Original Sphere starts with scale zero, easing tag1, flags[0,1], value=radius,
duration0, t0, eased0. These are preserved. Current signifier_animation_t fails
to set t/eased for duration0; forcing scale1 or an animation bypass flag would
change the algorithm and is not done. The current actual tiny-glade host has no
signifier create/animation/mark/delete systems registered and no debug_sphere_*
asset producer. Thus this restores the requested complete producer and sphere
submission implementation; it does **not** certify the entire visible debug UI
or GPU lifecycle. These consumer/startup-owner gaps were reported to central.

Warm Windows DB overlay adds only the bounded original main1129B, Sphere676B
and closure1585B body evidence; no guessed ABI signature was installed. The
shared warm worker was subsequently lent for structure_outline evidence; its
separate overlay manifest is maintained by that owner. No original baseline DB
synchronization is claimed.
