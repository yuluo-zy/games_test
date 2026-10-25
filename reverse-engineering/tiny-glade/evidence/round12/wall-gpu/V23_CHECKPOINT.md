# V23 source candidate checkpoint

`candidates-v23/manifest.json` and `candidate.patch` supersede the source
proposals in `candidates/`. Both are review-only; production has not been edited
during the central build freeze. Central Cargo/LLVM runs remain exclusive.

## Added correct ownership and ready state

- PublicWallState now has an explicit `Option<f32>` gable maximum readiness,
  separate from optional gable profile geometry. `prepare_paths` invalidates it;
  `update_gable_heights` writes the actual base first, then applies the original
  rectangle/gable update. Missing initialization still panics in the getter.
  Initialized no-roof walls return the actual base; projection flags match this
  same owner. The candidate includes a real collect/prepare/init/reprepare test.
- The19 init wrapper uses rectangle→PublicWalls::roof_entity→same-generation
  component lookup; a present stale/missing component panics. It no longer finds
  another roof by scanning matching WallId.
- MaterialStats is attached to this canonical row and publication advances the
  previous half only when replacing the public row. Raw-mask calculation resets
  current stats and writes the actual mask count. The special source is296, not294.

## Original finite pool source policy

Both96B arches/gates use one native per-wall allocation at Public+120; both64B
lintels/fences use Public+130. The candidate owns these as safe typed allocations
in one resource, preserving the actual shared physical SSBO libraries.

-19 `gpu_output::clear_constructor_planks` clears this wall's64B owner.
-22 `construct_arches` clears the shared96B arch/gate owner, then attempts each
  generated brick in order. Special296 nonzero follows the original empty main
  mask branch. Gate events, height updates and material accounting remain alive
  after capacity failures.
-22 lintels and28 gates/fences append at their actual source execution point,
  resuming the existing allocation's final chunk. **44 no longer processes a
  HashMap of pending geometry**, so it cannot reorder allocation quotas or keep
  stale geometry on exhaustion. It only performs real dirty-buffer upload.
- Each failed push is recorded and iteration continues. Successful prefix
  remains. Arch/gate/fence constructor logs use original exact strings and
  **Error** level1 once after each complete reader invocation. Source statistics
  are still written after partial capacity failure. Device upload failures leave
  actual sparse dirty data for retry and remain observable.
- Gate entity cleanup no longer incorrectly clears this shared arch/gate brick
  allocation in28. Deleted wall owners still drop their allocations into the
  actual existing waste queue. They do not alter main-wall InstancedWall owners.

The only underlying library API addition is `SparseInstanceBuffer::append_writer`:
it obtains the current existing writer and sets its cursor to the allocation's
last chunk when present. Generic `writer` still starts at the sentinel. Every
existing user push/clear/batch replacement/resource-owner algorithm is unchanged.
Original call sites explicitly initialize the last-chunk cursor; recovering only
the generic writer's default would waste tail capacity and change finite output.

## Material and roof side effects

Arches and lintels sort+dedup their reader unions, as original wrappers do.
This prevents two simultaneous changed/create notices from duplicating gates or
clearing and appending geometry twice.

The arch pure producers now retain original material statistics observed before
profile changes. For a mask which has a roof and exactly one materialized segment,
the clipped positions' fraction with Y<0 is compared strictly to0.5: larger is
sewer; other/unordered is gate. Other masks add their original mask length to
arches. The native arch count subtracts gates **without subtracting sewers**.
The wrapper calls the original six material setters/transfers in order. It uses
PublicWalls::has_roof (including the real failure-map/generation test), not an
invented geometry or query-presence predicate.

BuildingHadArchesLastFrame is restored from a global bool shell to the original
HashMap<u64,bool>. Every arch invocation scans all actual public walls. For live
associated non-flat roofs it compares previous bool (missing=false) to
`ArchSegments[wall].len()>=2`, emits RecalculateRoofCmd{entity,wall,kind:3} on a
change, and writes that wall's bool. Flat/stale/missing roofs are skipped exactly
at this tail owner. The init system's stale-association failure policy is different
and intentionally remains different.

Lintels now preserve the complementary source296 gate: normal walls skip the
mask loop. They record the actual cumulative materialized segment count and sum
of raw mask lengths. Their full retained LintelSegments/pillar-proposal owner is
still a separate unclosed boundary; the statistics do not imply that owner exists.

## Verification truth

- Existing independent run:530/530 original MaterialStats selector/reset calls
  matched actual Rust.128/128 original publisher move/reset fixtures matched the
  candidate's previous/current advance, including lengths/counts/seed bits and
  real Vec move ownership.
- Newly executed original native only:132 gable base ready/value cases plus
  initialized getter;168 arch classification cases;**24 cases,
  192 total checkpoints** each for96B and64B finite tail-resuming pools. Every
  generated native push is attempted even after exhaustion; clear and reuse are
  included. Native fixture JSONs record all counters, allocation/free ordering,
  active/dirty bits and meaningful payload bytes.
- The finite-pool whole original push/clear executes with only Cargo parking_lot
  deadlock-observer acquire/release bridged in an uncontended reserved-vector
  environment. No private game helper or sparse algorithm is replaced.
- Arch classification executes its original bounded block with both continuation
  edges returning to the fixture; this is not a full original constructor run.
- The arch Rust comparator received a separate bounded central authorization:
  **168/168 passed** bitwise lengths and gate/sewer counts using the actual
  candidate arch_stats.rs, one standalone rustc executable with no Bevy/core
  dependency. `arch-stats-check.log` records the result. Pool Rust drivers are
  **prepared, not compiled or passed** while central memory pressure excludes
  new large LLVM jobs. The first earlier pool-driver
  attempt failed because the test device lacked a required trait method; that
  fixture now explicitly rejects GPU upload. No failed attempt is counted passed.
- Rustfmt parsed all Rust candidates. This is not type checking or GPU acceptance.
  The new shared-owner lifecycle test supersedes the old transactional replacement
  test, whose behavior deliberately differs from the recovered source semantics.

The22-file source was copied and individually hash-checked into immutable
`snapshots/v23-22`. Its manifest SHA256 is
`79fe37ce13325f7bb3a0cb114c584affc716ac4f4ed251f566f245b362ce0bca`.
Central integration can consume this stable snapshot independently of later
candidate work. No source inside that snapshot will be changed.

After integrating candidates and compiling core, central can run:

```powershell
python verify_arch_stats.py
python verify_plank_pool.py --resume-tail
python verify_plank_pool.py --resume-tail --bricks
```

These are standalone rustc comparisons; no Cargo is launched by these scripts.
The pool comparison resolves central's latest existing core rlib and records its
hash. It requires the newly integrated append_writer API. Actual ECS tests and
nonempty GPU/readback remain central's acceptance work.

## Remaining exact boundaries

Main-wall InstancedWall's existing transactional upload is a different owner;
this source candidate does not claim its finite-capacity policy equals original
main-wall construction. LintelSegments/pillar proposals, platform-stair creator
ECS/collision/holes lifecycle, floors/pillars/plaster/half-timber wrappers and any
unproved style-specific outer branch remain open. Existing complete core geometry
algorithms are reused, not replaced by older wrappers. T13 remains excluded.

## Reverse-engineering lessons

1. Recover caller cursor initialization as well as container push/clear. A correct
   library algorithm can still behave differently through a mismatched caller API.
2. Identical generic clear code can be folded across element types. Identify the
   SSBO at the caller and allocation field; do not infer type from clear's address.
3. Inspect full source/consumer paths before inventing a missing output: fence
   posts were already64B outputs, not a separate Nani requirement.
4. Original numeric sets require complete tuple order and chain policy. Adjacent
   strings in rodata must be trimmed with actual per-call lengths, not treated as
   one function name.
5. Separate explicit initialization state from optional feature geometry. Getter
   fallbacks can hide a real missing producer and disagree with projected flags.
6. Version candidate directories, freeze production, record source/candidate
   hashes, and keep original oracle execution distinct from Rust and GPU results.
