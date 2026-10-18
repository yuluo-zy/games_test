# Configured inactive texture read: current-architecture candidate

Status: work candidate only; no production source applied and no Cargo command run. `rustfmt` parsed the candidate sources. This adds a missing capability to the user's existing graph; it is not an original ABI reconstruction.

## Actual failure and original consumer evidence

Strict real GPU v30 `original_scene_compute_gpu::reprojection_first_frame_uses_false_validity_and_copies_real_depth` failed before submission with `UninitializedRead { resource: 27, pass: 2 }`. That is the newly allocated real `reprojection.prev_depth` temporal texture. No fake clear/import initialization was used.

`manifest.json` binds the installed original shader package and its complete extracted SPIR-V. The sole previous-depth image content instruction is `OpImageGather %241` on set 0/binding 15 in block `%222`. `_Globals` member 2 (`history_valid`) equal to zero bypasses block `%215`; `%220 = OpPhi(false, ...)` then jumps directly to `%221`. Thus the original shader cannot fetch previous-depth texels for `history_valid == 0`. The additional frame-index gate is retained conservatively; this candidate does not simplify it. Native code still unconditionally registers the binding and access code 0x19.

## API and contract

`AccessDeclaration.contents_unused`, default false, states that the configured shader cannot read the resource's contents. `add_inactive_resource_read_range` admits it only for a non-writing access, validates the same graph identity/range/access code, and feeds the current ledger. It does not initialize, write or publish the texture. It cannot disable a future active read or export check.

The ordinary access code, synchronization group, resource index, range and version remain. All ResourceInfo usage inference, native range program, barriers, hazard edges, multi-queue ordering, physical resource allocation and lifetime still use the original access. Only content-initialization requirements and content-producer dependency/liveness change. A dead writer may be culled if its only consumer has no configured contents read; surviving read/write hazards remain ordered.

The current metadata conservatively merges aliases: inactive AND inactive stays inactive; any active same-code/group/range binding makes the access active. This merges only the added capability field. It does not rewrite the original ledger's code/group merge behavior. Legacy read→write upgrades retain their conservative preserving semantics and clear `contents_unused`; this candidate does not relax such an operation.

Full GraphDefinition Eq/Hash includes the field through AccessDeclaration's existing derives; cache-hit declaration comparison copies the complete access. Normalized subgraph keys clone complete accesses and keep the field. Compiler version increments 2→3. The independent plan checker rejects an inactive write or preserving-read combination, checks versions/ranges as before, and skips only the inactive contents provenance lookup. Subgraph input-content validation follows the same capability while boundary identity/version/lifetime remains.

## Host mapping and original consumer

Companion host patch records `inactive_texture_bindings: Vec<u8>` by the actual PassBinding table index. It does not mark a whole resource inactive, because a second shader binding could actively read another view of that same resource. GraphPasses matches native ImageView occurrences in original order, retains their exact view range, and passes metadata only on the corresponding non-writing access. The RG ledger merges duplicated aliases conservatively.

The reprojection consumer adds `inactive_texture_contents(PREV_DEPTH_TEX)` only when `prev.valid == 0`. It retains its exact original uniform byte, all four bindings and the Copy-depth producer. This systems file is owned by a separate user Codex session; the patch is delivery for that owner, never permission to overwrite it directly.

## Files and owners

- `resource_info.rs` family TG-06c573c2211f4ad624 and `pass_builder.rs` TG-b291427134b1c1582b: root `main-chain-frame-20261018` integration claims.
- `access_ledger.rs`, `resource_contracts.rs`, `plan.rs`, `plan_check.rs`, `subgraph.rs`: current refactored architecture modules, no original file-family mapping; no original address/ABI claim is attached to the new metadata.
- `compute_pass_builder.rs`: root-delegated narrow adapter, same existing resource graph/PassResources/session.
- tiny-glade `render_host.rs`, `compute_frame.rs`, test literals: current integration host files.
- country-core systems `reprojection.rs`: other user session owns current business algorithms; parent coordinates handoff.

## Meaningful validation prepared, still unexecuted

New CPU target `rhapsody/tests/inactive_contents.rs` covers ordinary undefined read rejection; inactive non-initializing read acceptance; active read/export after inactive still rejected; both alias-registration orders; rejection of inactive writes; dead producer culling with surviving native read→write hazard; full and normalized cache configuration distinction; independent forged-metadata rejection. Existing typed resource safety APIs remain unchanged.

Then rerun the actual first-frame reprojection GPU test without clearing history. It must execute original Reprojection and Copy-depth passes and read actual copied depth. The test's original resolve readback conversion is already corrected from independent packaged SPIR-V; no GPU PASS is prefilled here.
