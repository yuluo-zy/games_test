# Shared Sheep authority and movement consumer correction — B

## Scope and ownership

Incremental changes inside current modules. Full current critter Sheep's behavior/animation fields now live in the existing core Sheep owner. Critters components.rs reexports that exact component and SheepMoveState; sheep_behavior::SheepGoal reexports the existing core CritterGoal. Transform already reexports core. No parallel state, synchronization system, new crate, native layout, default spawn initializer or replacement critter module is introduced.

Owned claims: behavior TG-2021adb457a86d57d2 and unstuck TG-cca82620362943b1e4, owner building-audit-20261018. Their queue metadata defaults to C; this delivery's actual target/evidence is explicitly B. components.rs has no corresponding original PDB file group and is an explicitly authorized root integration adapter. Core Sheep/control/debug and tests are delivered in integration-owner.patch for root-held owners, not applied by this worker.

## Concrete change

- Single Sheep retains behavior_state / goal_timer / pet_phase / pet_progress / pet_target / amplitude / control_flag. The already recovered typed move_state is the sole movement representation. Old flag_30/shared_timers storage disappears; force_reach_goal is retained from actual current movement/snapshot, never synthesized as an old-state default.
- being_petted uses the existing full component's amplitude name and original strict >0.01 comparison. Core fixture accesses are adjusted; snapshot wire remains position / goal / move_state and existing control helpers remain the capture/apply authority.
- The actual critter goal/velocity queries now access core goal_tag / position. Reexports yield identical Bevy TypeIds; they are not newtype wrappers.
- Current update_sheep_goals is repaired in place: preserve typed old movement while following proposes a transition; decrement active timers; expired non-forced Walking becomes Idle; force keeps an active sufficiently distant XZ goal even after either timer expires; missing/near target or expired goal-change timer without force requests replacement; every replacement clears force. Original transitions create Walking with force=false only where the original explicitly writes byte+31=0. Idle's unused second timer/force padding is not semantic state.
- The native keep-distance threshold is f32 bits0x3e23d70b and uses XZ distance only. Ordered-comparison behavior is retained for the bounded NaN timer cases.
- Goal direction is (disk.x,0,disk.y) + scale*x0z(quat*(-Z)*0.5), with the original active-goal coefficient applied to projected forward. The old algorithm rotated the random sample and read the zero Y component as disk's second horizontal axis. Existing glam, RNG sequence and disk helper remain. Ordinary zero-vector fallback uses current glam normalization.
- Water sampling is short-circuited as in the original: no water reads unless grounded and inside the radius gate; subsequent reads follow the same conditional zero comparisons.
- Existing sheep_unstuck now changes only Idle.timeout_s on its final gate. Original writes one f32 at+34; the old shared_timers=[0,0] changed an extra inactive field.

## Original evidence

B binary SHA256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03; matching PDB SHA256 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. B PDB procedures identify update_sheep_goals0x142315050(3558B), set_sheep_velocities_towards_goals0x142315e40(1609B), sheep_unstuck0x1423246b0(2179B). Independent original-RNE machine-byte extraction and patched-B comparison match all three ranges; evidence/static-identity.json includes original PDB records and hashes.

- Movement layout/force is independently established by the existing B snapshot serializer pack: state tag+30, force+31, timeout+34, goal_change_timeout+38. Serializer0x1413ff520, capture0x1412f03f0 and apply0x1416f24f0 were previously independently reviewed.
- update0x1423157fe/82c reads force+31. 0x142315849..0x142315889 checks goal bit0 and squared XZ displacement against the observed threshold. 0x1423158a3 selects the expired non-forced Walking -> Idle transition. 0x142315247 selects Idle -> Walking. 0x142315290..29f writes new state/timers and explicitly clears force; replacement paths0x142315ccd/0x142315d8c also clear force.
- Direction0x14231596a..0x142315acd executes the original quaternion rotation, horizontal projection, sample combination and normalization. No Ghidra queries against C addresses were made.
- unstuck0x142324d21..d2c checks movement tag0 and writes only timeout+34.

## Validation and acceptance limits

evidence/force_phase_probe.py executes original instructions only. 16 movement-phase cases cover timer positive/expired combinations, force false/true, inactive/near goal, vertical-only distant goal, and NaN timers. It stops before RNG transitions/target generation. Eight direction cases supply already resolved disk samples and execute original quaternion/XZ projection/normalization, including yaw/pitch/roll and exact cancellation to zero. The first direction attempt omitted its scratch-frame pointer and failed unmapped read; that setup was corrected and all24 cases completed. This is not full query discovery, entire goals system, RNG verification, spawn, native process or whole scene acceptance.

Candidate includes:
- Generated regression against16 exact original phase outcomes/timer bits/force.
- Eight direction expectations from original output, with2e-6 tolerance because the existing current glam dependency is reused.
- A real Bevy critter-Sheep/SheepGoal fixture passed to the actual core sheep_save_section_system, asserting unchanged snapshot wire with force=true and identical ECS types.
- Existing core capture/apply fixtures adapted to complete explicit component fields; one restoration fixture uses nonzero animation/behavior fields and verifies that only movement changes.
The explicit zero fixture values are test inputs, not claimed original birth defaults.

rustfmt parses all8 candidate files and all3 patches pass git apply --check. No Cargo invocation or Rust candidate execution by this worker. Root owns independent review, compilation and tests. Delivery remains implemented_unverified.

## Remaining boundaries

- Original production spawn/default initializer was not recovered here; no synthetic Sheep::default is added. Known workspace production sheep consumers query components; this review found no concrete production Sheep struct-literal birth initializer and does not claim a runnable population.
- Current resource wrappers, UI pet drag adapter, accessory/ground records, audio, input replay session gating and critter global scheduling retain their existing designs and documented unresolved behavior. Components identity fixes do not by themselves recover these.
- Render callback SheepView is still a separate view/projection requiring its own real render extraction owner; no duplicate live Sheep component is introduced to satisfy it.
- Invalid/non-native behavior discriminants beyond the observed ordinary0/1 modes, extreme/non-unit quaternions and extreme normalized-vector arithmetic are not new acceptance claims. The candidate preserves current dependency implementations rather than rebuilding them.

