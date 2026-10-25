# B collision query correction contract

Target is current B gameplay, as required by country-core/recovery/README.md. The A/B/C file group is held by building-audit-20261018; C remains a distinct unverified baseline. All outputs are candidates in this claim directory; production source and Cargo were not changed.

## Identity and evidence

- Read-only live Ghidra: tiny-glade.exe at `/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.exe`; path/base and revision recorded in `collision-overlap-fresh-direct.json`. Local SHA is cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323.
- Original B RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. `static-byte-identity.json` confirms 14 bounded function ranges are identical to analyzed B. This is static instruction identity, not native runtime validation.
- Original retained evidence: `evidence/building-collision-world/B-query-owners/1408cdc60.asm.txt`, `1408ce180.asm.txt`, `evidence/country-core-deep/collision-eco/decomp/decomp-0x142612d10.txt`, `decomp-0x14155d340.txt`, `decomp-0x14129b780.txt`.
- Fresh originals: `collision-overlap-fresh-direct.json`, `collision-rectangle-disassembly.json`, `collision-overlap-closures-fresh.json`, `collision-overlap-callback-disassembly.json`, `collision-queries-fresh.json`, `collision-terrain-disassembly.json`.
- Reused dependency implementation: locked PounceLight/rapier commit 7b5ece73c45a4d39bb983c48c283e773d7241050, local `src/pipeline/query_pipeline/mod.rs:683..716`. Its intersections callback returns true to continue. The private game policy is not replaced with Rapier internals.

## Observed facts and corrections

1. B Rapier::raycast 0x1408cdc60 updates XMM6 from hit TOI at 0x1408cde07 and passes XMM6 to each subsequent cast at 0x1408cdda4. Raycast_simple 0x1408ce180 similarly assigns hit TOI to its next query bound. Current reconstruction always used f32::MAX for every slot and could choose a farther collider from a later slot. Candidate retains the nearest hit; dependency equality policy preserves later equal hits.
2. B raycast_w_terrain 0x1408cd430, at 0x1408cd508..0x1408cd51f, passes the selected collider TOI into terrain intersection. Current reconstruction passed f32::MAX and let farther terrain cover a nearer building. Candidate supplies selected TOI when a mesh hit exists.
3. B legacy_raycast_w_params 0x1408cc770 writes integer 2 to result+0x30 on miss. Current reconstruction cast 2 to f32 and wrote 0x40000000. Candidate writes f32::from_bits(2), matching the already-corrected unparameterized sibling and real callers' integer comparison.
4. B Rectangle3d adapter 0x14155d340 computes half extents = extent*0.5; position = (footprint.center.x, (bottom+top)*0.5, footprint.center.y); rotation = Quat::from_rotation_arc(Vec3::X, (axis.x,0,axis.y)). Full raw instruction sequence plus helpers 0x140c98660/0x140c9cde0/0x140c9cdc0 establish the operands. Current helper returned identity and consumer queried at origin. Candidate preserves the original center and orientation.
5. B handle/iso adapters 0x1408d98c0 / 0x140b26d20 produce both pose and shape; 0x14129b780 / 0x140254890 pass pose into dependency overlap queries. Current wrappers discarded pose. Candidate carries it to the existing shape query loop; the existing origin-shape convenience entry remains available.
6. The current reconstructed public callback explicitly returns true to stop. Current loop passed it unchanged to Rapier where true means continue; false also caused Rapier to stop after the first hit in each slot. Candidate inverts the dependency callback and preserves outer termination. This correction is supported by current API contract + locked dependency source, not a newly claimed recovery of arbitrary original callbacks. Original player-made stairs closure 0x1403c1060 always returns AL=1 to continue after its game side effect, consistent with the dependency contract.
7. B OffsetColliderTransform adapter 0x1408d9960 copies the complete pose. Candidate decorator helper now uses that pose; full decorator source filter/rank/return policy remains unresolved. Unit quaternion components are wrapped without renormalization, matching original pose-copy semantics and existing utils::IsoTransform's unit-rotation precondition.

## Producer → consumer and lifecycle boundaries

The chain is authoritative Rectangle3d / collider pose / IsoTransform → transformed overlap query → current callback, or world subworld colliders → nearest ray query → core focus/picking/hint callers. Creating/editing/removing colliders and flushing query pipelines remain the existing lifecycle. Candidate does not claim to fix their producer or registry semantics. Existing stair assembly original consumers are 0x14129b780 and 0x140254890; current workspace has no external calls to the reconstructed overlap helpers, so original full building integration is still open.

## Verification status and concrete remaining gaps

`world.rs`, `collision-world.patch`, `source-manifest.json` are the deliverables. Four narrow regression tests are included: nearest selected-subworld ray, miss bits, remote rotated rectangle, remote collider overlap + callback continue/stop. Per root ownership, no Cargo build/test ran here; root performs shared verification. The native instruction oracle was not executed for these geometry changes. Status is implemented_unverified pending independent review and integration validation.

- Current RaycastGeoSource drops observed 24-byte payload and ColliderRegistryEntry uses kind/meta projections; derivation from slot is not enough for exact callbacks.
- Registry lookup currently uses raw collider index/generation instead of collider.user_data → global registry; upserts across separate subworlds may collide. Full allocator/source/mesh owner repair requires broader contract; candidate does not hide it.
- Original decorator_overlaps_with_others returns a resolved source pointer/option, uses fixed subworld and custom predicate/rank; current bool+mask API remains a partial approximation. Only pose loss is fixed here.
- Original spherecast accepts ShapeCastOptions (max TOI and flags). Current radius/mask signature substitutes defaults and the wrapper compares terrain TOI to radius; signature/options/limit need recovery before claiming correctness. `collision-sphere-disassembly.json` records max-TOI narrowing (XMM7 hit update @0x1408cd23f).
- Raycast detailed normal/UV projection still bypasses original `rapier_hit_to_raycast_world_result` and real source metadata; missing registry currently falls back to fabricated raw handle bytes.
- resources/stairs/invalidate.rs still registers collider removal×3 / RT removal as comments; systems/wall/decorator_wall_events.rs pending handler is empty; systems/wall event payloads and structure_outline retain `_gap`. These are actual separate unresolved producers/consumers and were not rewritten under the collision claim.
- No full schedule, save/replay, GPU or complete systems/resources acceptance is claimed.
