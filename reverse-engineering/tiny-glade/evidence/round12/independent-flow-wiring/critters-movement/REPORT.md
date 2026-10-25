# Shared critter movement and steering

Source candidate `candidate.patch` has been centrally integrated; this agent only wrote evidence. `avoid-candidate.patch` adds the later68-case native-output Rust test without changing any integrated source. Original function ranges, byte hashes and full disassembly are in `original.json` and the matching `.asm` files. Original B whole executable SHA256 is `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.

The original typed system wrappers are independently recorded in `typed-system-contracts.json`: smooth141daaa70 has exactly Target and mutable Velocity query columns plus GameTime; movement141daabf0 requires the full seven-column query and GameTime, WallSpatialHash, PublicWalls, AssetSsboLibrary, WaterRaster, GardenRaster, RaycastWorld; avoid141daa140 requires its six-column query, PublicWalls, WallSpatialHash, mapped library, four PathRasters, WaterRaster, RaycastWorld, DespawnDuck writer and UiSignifierStream. No private default samplers or physics worlds are installed to satisfy those contracts.

Recovered policy differences:

- ECS changed-tick writes are not multiplier writes. The old extra required multiplier query column filtered legitimate smoothing entities.
- Movement uses `(velocity*dt)*multiplier`, builds the exact displacement segment, and queries only the current native wall cell. The old four-unit AABB and whole-wall expansion changed collision selection. Point query preserves repeated rows of the same wall and returns empty outside its bounds; it does not clamp.
- Native utility constructor140c97560 is an ICF shared four-float constructor. Its Circle2d symbol name did not prove any circle business here: actual call operands form the two line endpoints, followed by LineInterval::line_segment and relate.
- Move loc3 skips PublicWalls lookup entirely. Avoid still performs that mandatory lookup and skips only max-height filtering for loc3. Missing public authority retains its native failure.
- Duck water checks preserve unordered handling. Sheep boundary checks after the spherecast sample **GardenRaster twice**, including the prediction; the old source mislabeled them water. Water and Garden typed owners and current selected buffers remain distinct.
- Spherecast direction is normalized full displacement for movement and normalized horizontal goal direction for avoidance. Native legacy output+0x30 tag2 means **miss**, not a rock collider category. Core legacy spherecast owns its full slot policy; the old private slot_query always returned None and its hand-written nearest-hit loop was wrong. The compatibility name now reexports the same core Resource TypeId.
- Movement orientation uses from_rotation_arc(-Z, normalized horizontal displacement), preserves the native sign-bit choice on the old quaternion, weighted nlerp and division by quaternion length. There is no invented small-velocity threshold. A zero/invalid horizontal direction skips only rotation, not BirdSpots.
- BirdSpots use scale and local offset `(0,1.567,-0.495)`; the zero-x scale term is retained for IEEE nonfinite behavior. All present spots update and their occupancy data survives.
- Avoid processes only goal tag1, samples the water probe before its locomotion branch, preserves all four path samples at both positions and uses the original absolute goal length branch. Its wall probe ends at goal+unit-direction, rather than position+unit-direction. Native close Duck sphere hits still request DespawnDuck and clear the goal.

Actual native execution:

| Harness | Executed original game range | Results | Boundary limitation |
|---|---|---|---|
| native_move.py | Complete one-entity business CFG142319b8b onward, including original line geometry and original glam dependency calls |71cases:70return,1mandatory-public panic |Bevy query iteration outside harness; water/garden/mask/public/physics results are injected explicit inputs, with exact call arguments recorded; standard powf delegates to ucrtbase |
| native_avoid.py | Complete one-entity business CFG1423182cd onward, including original line geometry |68cases:66return,2mandatory-public panic;2Duck despawn paths |Query/event storage traversal outside harness; world results injected and recorded; no game predicate hooked |
| native_point_query.py | Complete original273B query140aa5730 with real native bucket headers |289boundary cases including ±65, ±Inf, NaN |Only standard floorf delegates to ucrtbase |

These counts are **native fixture production, not successful Rust differences**. The first source candidate adds a71-case test calling the same actual `advance_critter` used by production, actual ECS smoothing without multiplier, actual nonempty point-cell geometry/get failure contracts and the canonical RaycastWorld TypeId. The core test compares289original point-query results and preserves duplicate rows. The follow-up avoidance test calls actual `avoid_critter_collision` for68cases and compares goal state, despawn decision and every sampled coordinate/sphere direction/radius. Test environments provide recorded external world inputs; actual geometry/owner checks have their separate tests, and no full ECS/GPU completion is inferred from a pure policy fixture.

All four first-candidate complete Rust files and the follow-up test parse with rustfmt, and both patches pass git apply --check against their delivery source windows. This agent ran no Cargo, LLVM, restored Rust tests or game process. Other species still have separate unverified leaf/resource gaps; the old entire CritterPlugin/configure must not be mounted merely because these shared algorithms are recovered. No T13 or user rhapsody code was changed.
