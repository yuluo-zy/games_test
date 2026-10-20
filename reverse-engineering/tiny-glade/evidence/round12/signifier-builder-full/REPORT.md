# Complete SignifierEntityBuilder candidate

Status: seven source candidates only; no production writes and no Cargo run by this agent. `candidate-checkpoint.json` contains original-byte base SHA256 and proposed-byte SHA256 for each file. `production-review.patch` passes git apply --check; every proposed file passes rustfmt parsing.

## Original evidence and all branches

Original B RNE SHA256: `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

PDB module486 establishes `new@14089d9b0=274B`, `build@14089dd50=3840B`, and nine emitted setters. `pdb-module486.txt` and `original-builder.asm` retain these extents. PDB void() records are placeholders; actual call ABI is sret RCX/self RDX/AssetMeshLibrary R8/AssetShaderLibrary R9/Commands stack. The native return is EntityCommands; the current owned API returns its Entity and accepts Commands for equivalent subsequent inserts.

`original-constants.json` records exact native strings and the five jump targets. SignifierMesh uses Rust container niches rather than a plain integer enum. The complete switch is:

| Native discriminator | Input | Behavior |
|---|---|---|
| 8000000000000000 | owned name | existing mesh handle lookup; missing name panics |
| 8000000000000001 | empty | register real empty Mesh with label `signifier empty` |
| 8000000000000002 | owned Vec<Vec3> | existing mesh_from_pts, label `signifier custom` |
| default valid Vec capacity | points + colors | mesh_from_pts + actual Vertex_Color overwrite, label `signifier custom w colors` |
| 8000000000000004 | boxed point factory | invoke factory, mesh_from_pts, label `signifier custom fn`, drop factory |

All four nonnamed branches insert the existing TransientMesh receipt, preserving the shared deferred deletion registry. No substitute empty geometry is used for a requested points or named mesh.

The old raw builder incorrectly called the first16B tag/color fields; native rotation setter copies a full quaternion there. New builds retain actual quaternion/position, default shader `vertex_color_shader`, default EasingFunction tag1/CubicInOut and native animation flags/value. Native new sets flags16=0x100, value1, duration0.2, position0 and identity rotation. An enabled default animation starts Transform.scale at zero and stores explicit/default scale in animation.value; disabled animation starts at explicit/default scale directly.

The candidate resolves shader handles with the actual AssetShaderLibrary and preserves missing-shader failure. DrawableMeshBundle and SignifierAnimation are inserted on the reserved entity; WireframePass takes priority over UiPass, DontCastShadow is always inserted, and optional MinimumLifespan and ParentToVolumeOrigin are actual components. Native set_ui_pass stores pass XOR one, so input zero disables UiPass. SignifierId is assigned by signifier_create; the builder must not fabricate an id0.

## Numeric animation and deletion

Independent owner history_wire_fix extracted the complete original functions and typed Bevy wrappers, then executed 1008 original SSE boundary vectors. See sibling `history/SIGNIFIER-ANIMATION-INDEPENDENT-REVIEW.md` and `signifier-animation-independent-original.json`. The candidate embeds those original outputs as standalone Rust regression data; no Rust test pass is claimed yet.

Animation's exact query is mutable Animation, optional deletion mark and optional mutable MinimumLifespan. For nonzero or NaN duration, original evaluates `(1/duration)*signed_delta+t`, preserving operation order. Deletion flips delta's sign; MAXSS/MINSS clamp preserves unordered NaN and signed zero. Duration ±0 chooses endpoint one when unmarked and zero when marked, then still applies the actual existing utils EasingFunction. MinimumLifespan age always adds positive frame delta. The incorrect extra tick_acc field is removed, restoring the real28B component.

Mark lifespan comparison is `!(min > age)`, allowing unordered input. Despawn's real query includes Entity, deletion mark and optional Animation; deletion happens with no animation or ordered t==±0, while NaN t remains alive. Multi-frame regression expectations now wait for reverse animation instead of enforcing immediate destruction.

## Original builder executions

`original_builder_probe.py` executes the entire original3840B build function for160 cases: five mesh variants times32 combinations of animation disable, wireframe/UI, minimum lifespan and parent flags. It verifies original scale/value, transform bytes, pass callback identity, deferred queue length and all mesh branch traces; full native deferred payloads are retained in `original-builder-fixtures.json`.

This oracle isolates existing mesh/shader libraries, mesh_from_pts, std allocation/TLS, Bevy Entity reservation and transient receipt insertion at explicit call boundaries. It proves builder control flow and deferred payload contracts; it is not a full App/GPU execution or a replacement implementation of these dependencies. The candidate reuses those real current owners and algorithms rather than adopting oracle stubs.

## Cross-crate callers and verification boundaries

Water eraser and glade-border factories already receive the real mesh/shader libraries. Their previous implementations discarded both libraries and manually fabricated raw qwords. Both now call the complete typed builder, preserving original name/points, scale/position, shader, pass choice, topology and dashed-line length. The glade-border test uses a local test-only CPU shader-artifact backend because dependency cfg(test) TestDevice is not available across crates.

The seven files are factory.rs, animation.rs, mark_for_delete.rs (only numeric lifespan/despawn), signifiers/mod.rs (SignifierMesh export), existing multi-frame tests, water draw_water.rs and glade-border display_glade_border.rs. New core tests exercise actual World insertion, real asset handles, every owned mesh branch/receipt, pass precedence and duration-zero visibility; they await central Cargo validation.

Roof focused factory's agreed call is from_mesh_name("brush_arrow_flat_single_up"), duration0, captured quaternion, captured scale*1.5 and captured position. That separate owner integrates its original473B factory; no focused-only special case is added to this builder.

The independent create/mark owner is recovering authoritative UiState factories and16B SignifierId(hash,WidgetId). This candidate deliberately does not replace shared UiState or signifier_create. Existing animation parent-to-widget translation switch also remains an explicit separate gap; its scale/numeric lifecycle is restored here. Do not describe the entire signifier UI/GPU chain as complete before those owner integrations and actual App scheduling/render validation.
