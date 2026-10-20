# Decorator lifecycle: scoped source recovery and remaining native owner gates

Latest synchronized source base: main a6ef0b5. This packet does not assert that
toolbar, decorator, stairs, the full OnExit schedule, or the game has completed.

## Source delivered

`tiny-glade/crates/systems/decorator/src/lifecycle.rs` implements
`reset_scoped_existing_decorator_owners_on_glade_unload`. Its 19 represented
owners are mandatory Bevy resources. It is intentionally not mounted as the
complete original 20-owner unload entry. The missing StairColliderRegistry
operation belongs after StairsState and before DoorTracker. Current interaction,
audio, stone-column, stair-visual, and structural-graph schemas still have gaps.

The implementation follows the individually proved native operation order:
Storage, current/previous Blueprint maps, live Archivist, DeferredHistory,
VisualState, ID generator, placement interaction, AudioEvents, WindowLights,
Physics, StoneColumns, DormerTiles, ChimneyTracker, StairsState, [missing native
registry], DoorTracker, StairsVisualState, StructuralGraph.clear,
Affordance.clear, deferred remove_resource<StairsTopDownMesh>.

These are separate original owner replacements, not an inferred blanket reset.
Each real Default is constructed before the old aggregate is dropped. Current
and previous blueprint Arcs drop independently. Trackers drop all pending
variants without invoking removal/audio callbacks. Visual ID table destruction
does not despawn ECS entities. Affordance.clear preserves its unrelated type byte
and writes state 6/payload 0; derived Default would write the wrong state. Graph
clear retains its current containers. The top-down mesh remains until the command
queue is applied. Its logical ManagedHandle still does not prove GPU retirement.

No new staging buffers, allocation owners, graphics defaults, global event
clearing, world-physics clearing, renderer resets, or substitute collider registry
were introduced. In particular CachedDecoratorTransforms is not one of this
original consumer's parameters; its independently researched cache lifecycle is
owned by the visual agent. WindowLights must not also be reset by an independent
host fragment once a coherent full native unload entry is eventually mounted.

## Native verification

`extract_native_evidence.py` verifies the shipped PE's complete SHA256
f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03,
then extracts bounded raw bytes and Capstone disassembly. The machine-readable
manifest records ranges, hashes, decoded lengths, and direct call sites. It
does not execute the target, and unwind-table physical ranges are not assumed to
identify source-level function signatures.

The retained full original contract used for native type identities and order is
`skeleton/task-index/work/TG-bd3d82a9ddc435cfb2/deco-trackers-20261020/decorator-unload-contract-v1/CONTRACT_B_V1.md`
and its RESOURCE_OPERATION_AND_OWNER_MATRIX_B_V1.json. Its old source hashes and
absolute paths are historical evidence, not current source ownership.

The native unload is **9451 bytes**, SHA256
9745c5b55db1980334717839e4a5db03ad02e27b5a7bf4d974d2e5104f83e82c.
The old host comment's 5395 bytes does not cover the complete original. The
3507-byte exception-table range for invalidate_dirty_stair_artifacts likewise
must not be replaced by the old comment's shorter 3455-byte extent.

## Registry findings and exact unresolved boundary

The synchronized HEAD had no canonical StairColliderRegistry resource. The
StairsCollider zero-sized marker is a different type and cannot carry this
responsibility. This packet now restores the actual four-map canonical owner,
the structural geometry producer and handoff leaf, and the three-map removal
leaf. It does not install an always-empty startup resource or mount full unload.
StructuralGraph is an adjacency/index owner, while RaycastWorld owns the actual
global collider/mesh registry. Neither supplies the three per-edge keyed maps
which the native invalidation consumer passes to remove_collider. The existing
generic upsert_collider/remove_collider APIs already provide actual collider
creation/disposal and should be reused after the registry's producer schema is
recovered; a parallel physics implementation is unwarranted.

Native 1403481b6..14034831e constructs four empty keyed owners and installs 0xc0
bytes only after dropping the previous aggregate. Drop 14060a9e0..14060aadc:

| Native map base | Drop allocation calculation | Proven consequence |
| --- | --- | --- |
| +0x00 | bucket stride 0x1c | table ownership, no per-value destructor in this drop |
| +0x30 | bucket stride 0x1c | separate table, not an alias |
| +0x60 | bucket stride 0x1c | separate table, not an alias |
| +0x90 | bucket stride 0x20 | separate table, no per-value destructor in this drop |

In native invalidate_dirty_stair_artifacts, 140b376a1..140b376b1 constructs
base/base+0x30/base+0x60 aliases. The three remove_collider<StairGraphEdge> calls
are 140b37787, 140b377d7, and 140b37827. Each returns an optional old mesh whose
Arc is released on the matching successful path. Existing core invalidation
currently drains stairs and updates visual state but omits these three real
collider removals and the associated RT/assembly cleanup. That is a genuine
consumer gap, not a resource name or registration typo.

Do **not** infer a `HashMap<StairGraphEdge, Vec<Vec3>>` field from the old pillar
source comment. The checked physical containing wrapper is
1401788b0..140179abf (4623 bytes); the historical 140178a70 address is inside a
UTF8/file-name profiling scan, not a function entry. Its 32-byte-stride Vec map
is constructed on the stack at 140178c12..140178c27 (+0x1a0/+0x1b0/+0x1c0),
queried around 140179398, and drained near 140179678. This alone does not prove
that those vectors belong to the StairColliderRegistry resource. In addition,
the registry Drop's last table has no per-entry Vec drop loop. Restoring that
schema requires the actual argument/producer bindings, not the old comment.
The fourth map's identity was subsequently closed by an independent producer
binding: native create_structural_colliders_and_initial_graph takes argument 2
through 1407d23e1..1407d23ec to owner+0x90 and passes that map to 141299ca0 at
1407d2b79 and 1407d2ede. Retained B collision/world.rs:6507/6514 names that
monomorph `RaycastWorld::upsert_collider<(StairGraphEdge,u32),IsoTransform>`;
lines 18926 onward explicitly preserve its `(key,ColliderMeshInstance)` map
relations. The caller constructs a 12-byte `(edge,index)` key at
1407d2af8/1407d2aff before also passing it to StructuralGraph::upsert_span.
Thus the fourth map is a structural segment registry, not a Vec payload.
Current StructuralGraph's edge-only span key drops the sample index and remains
a separate schema gap. First-three lane semantic names remain unresolved; the
new canonical owner preserves their three exact identities as array slots.

Native 1407dec90/955B is now restored completely: two endpoint/direction/width
triples produce eight float vertices, then the real convex constructor, with
the native tiny-sphere fallback only when convex construction fails. Independent
Unicorn execution ran this original leaf and its original swizzler, hooking only
the convex/sphere construction boundaries. The retained fixture contains their
actual float-word arguments for normal and degenerate geometry and both hooked
constructor outcomes. The fallback's unusual endpoint sum plus 0.5 (with each y
shifted by -0.5 first) is preserved exactly, rather than replaced by a midpoint.
The tiny sphere's radius is original bits 0x3727c5ac.

The original leaf passes a contiguous Vec3 array to native new_convex. The
existing canonical constructor previously cast UVec3 word values numerically
to f32, corrupting negative/fractional coordinates. Its API now accepts actual
Vec3 points; it hashes their original u32 bit view and sends unchanged f32
coordinates through real Parry convex construction. No old caller used this
constructor. The new structural handoff leaf uses that actual mesh with existing
upsert_collider and the full `(edge,index)` key, requiring a real source input.

These leaves now produce nonempty real colliders in the canonical world and
dispose them through its existing generation/mesh/dirty-publication mechanism.
The full request traversal remains unconnected: current request models omit
native +0x98 optional sampled data, +0x118 metadata and +0x148 gate, while the
original producer also depends on complete span-aware StructuralGraph. The
current full invalidation consumer also still lacks RT/assembly/structural
retention branches. No missing field is defaulted or silently skipped here.

Other retained schema gaps are not concealed by resetting current subsets:
native audio owns four map owners, current audio only two maps and frame words;
native StoneColumnCollisionStorage additionally owns a collider registry;
StairsEdgeVisual leaves most of its native payload opaque; StructuralGraph
still documents its missing +0x120/+0x180 tables. These need owner-specific
producer/consumer recovery before declaring complete unload.

## Verification boundaries

The external target `decorator_unload_lifecycle` contains four production-helper
tests: populated reset (including live cull policy, blueprint Arc lifetime,
deferred mesh removal, no audio emission and no entity destruction), both strict
trackers with Current/Added/Changed/Removed across sessions, actual RoofTileSsbo
allocation Drop to the same sparse waste queue followed by actual waste drain
and capacity reuse without duplicate retirement, and missing mandatory owner
failure before mutation or final command.

Only source formatting was performed by this agent. Root exclusively owns Cargo
and records its actual outcomes; no target execution or GPU acceptance is claimed.
The tests do not duplicate the general host despawn policy and do not prove the
unmounted original OnExit chain. Host-side acceptance still requires real
producer-created InstancedWall base/chimney/door allocation destruction after
general deferred entity cleanup, then same-buffer reclamation, and a second
scene with no prior-session visuals, collisions, audio or RT geometry.

The prior save target fixture adjustments only initialize the newly mandatory
real StairsState in the two test worlds, leaving save production logic untouched.
Root subsequently reported all 11 save integration cases passed after sync.

Additional completed frame-consumer repair: global
DecoratorStorage::clear_modification_tracking (140a1e720/791B, byte SHA256
71c94166b6e26671ff14b1a0bf6d5965a5e48b3e3e22efdda1991c559b668a09)
now asserts mutable first, clears only modified/removed sets per actual bucket,
then clears the removed-storage registry. It preserves records, d2s and bucket
capacity. The similarly named bucket destructive adapter is unchanged.
`decorator_tracking_lifecycle` has two tests for four populated bucket owners
(including full64-bit distinct walls), all d2s entries across two frames and
pre-mutation immutable failure. This fixes actual record deletion by global
metadata cleanup. The original caller is the validate_blueprints entry,
after preview/collision generation and before fullvisual, not frame end.

`stairs_structural_collider` has four further tests: original execution's exact
eight vertices and fallback words, actual Parry negative/fractional convex plus
degenerate fallback, actual mesh->two segment keys->independent disposal, and
all three native dirty-edge lanes removed through canonical physics while an
unrelated edge remains live. Fixtures reside in this crate's tests/fixtures,
not an external or deleted evidence path. These are leaf/owner tests, not a full
assembly request producer, OnExit schedule or GPU validation verdict.

## Shared shader and CPU fixture integration

The actual App owns SharedAssetShaderLibrary, shared with its ApplicationShaderLibrary
alias. Fullvisual now consumes that same Arc owner and borrows its contained real
AssetShaderLibrary for builders; it no longer requires an uninstalled raw library
resource. Both production signatures and their explicit CPU fixtures were updated.
No additional raw library/default host asset owner was installed.

The four terrain/cache/Nani integration cases retain all original assertions,
including actual Nani instance row maintenance and the 512-case original UV
fallback fixture. Their inter-frame drain now calls the real global metadata
method rather than manually clearing selected sets. The CPU schedule uses
chain_ignore_deferred so fullvisual commands are not flushed before copy. This
does not mount or assert the full original validator or complete outer19 schedule.

## Next validation chunk, explicitly not production

Original validate_blueprints is 140827130/539B; its business callback is
1412b84f0/8161B, not disposable generic iterator code. The retained complete
contract is under TG-2cc8aec081d0811cbe/clutter-startup-20261020/validate-investigation.
The exact Update/set25 order is collider-only, flush, validate (outer member9),
then later fullvisual/assemblies/copy (member14, chain_ignore_deferred). No
separate invented reset system or half-validator was installed in this batch.

Hard dependencies remain: real affected locator ID (currently only address),
full location result tags0..10 plus Valid11 and actual wall/roof algorithms,
actual collision/interaction/outline table payloads and loader, exact UI reason
feedback/generic icon variant payloads, and canonical unit mute readers. The
present bool location checker wrongly calls tag11 invalid and accepts wall/roof
without their algorithms. Merely flipping terrain/stair booleans, accepting an
Unsupported case, drawing a bbox as native outline or returning an error as a
replacement for the original algorithm is not recovery.

The complete address+ID locator/three real history producer source edits were
preserved only in next-validation-candidate and reverted from production at
central's commit freeze. These are uncompiled candidate material for the next
coherent dependency chunk. No new partial validation production API remains.

## Reusable reverse-engineering lessons

1. Check PE identity, physical function boundaries and owner aliases before using
   old decompiler entry addresses, shorter function sizes or stale TODO comments.
2. Separate a plain handle, a per-domain lookup map, and the actual global owner;
   dropping one does not prove the others were retired.
3. Recover original destructor/default/clear order before writing unload code;
   allocation capacity, Arc lifetime, deferred commands and partial failure order
   are observable parts of that algorithm.
4. Treat local aggregation maps and persistent resource maps separately. A
   matching bucket stride or generic method name does not prove field identity.
5. Test the actual production consumer and canonical allocation reclamation path;
   an empty-world assertion, reset-by-default stand-in, or fabricated global
   clear cannot establish a cross-session lifecycle.
