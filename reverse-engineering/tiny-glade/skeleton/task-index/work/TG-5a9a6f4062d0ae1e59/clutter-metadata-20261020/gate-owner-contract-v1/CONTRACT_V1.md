# WoodenGate actual owner and region/rejection dependency V1

B RNE f507a6ce/PDB2d9686b8 only. This is a bounded design and original evidence contract. No source candidate, native execution, Cargo, CPU/GPU test or whole-App claim is included. It continues the actual Gate -> region selector3/6 -> process -> manager chain; it does not replace populate with a pending/no-op and does not make a synthetic carrier producer.

## Legal ownership and present break

After root authorization both TG-9acc491e8aa123012c (actual construct_gates systems) and TG-3b74381dd8e4071b86 (around_gates) were individually re-shown NULL and normally claimed to clutter-metadata-20261020. Receipts are retained here. Root delegated only the necessary drawable follow-up under existing TG70b/boot-leaf, baseline324944a4, and the minimal env14/read-view companion under existing TG44dd/main-chain-boot. Neither historical lock is changed. Actual source before hashes and every current WoodenGateState/WoodenGateView/GateHost reference are frozen in current-source-and-reference-inventory.json. Other callbacks/registration are outside scope.

Current actual construct_gates already constructs real leaf geometry and owns top_positions, WallId, normalized endpoint directions, seed and clutter_version. It inserts a Vec-owning WoodenGateState and a separate two-scalar Copy WoodenGateView. The renderer reads the latter. Clutter instead queries an independently declared GateHost that has no current writer. Neither GateHost nor WoodenGateView is the original full component. The missing identity is real even with nonempty actual gate entities; there is no need for a new gate registry or position Vec.

## One canonical owned component

Proposed sole ECS type is country_core::components::drawable::WoodenGate, current semantic placement next to the existing drawable/AutoClutter/BalconyDoor values. Fields preserve the already actual constructor payload:

- top_positions:Vec<[f32;3]>, the owned world-space top polyline;
- wall:resources::walls::WallId, preserving full u64 (not the distinct publication_fields wrapper);
- endpoint_directions:[[f32;2];2], the existing prepared-curve endpoint directions;
- seed:u32, exact low32 native wrapping WallId*command.ordinal+leaf.ordinal;
- clutter_version:u8, the actual public wall+29a writer, later the region weight-group byte.

Use Component/Clone/Debug. No Copy, Default, PartialEq/Eq, native layout/size promise or numeric original TypeId is declared. The existing Vec storage is moved once from the real leaf into the actual entity; the type does not reconstruct an owner from a pointer. Native drop14014b850/32B independently proves the owned Vec allocation, twelve bytes per capacity element and alignment4. Rust Vec supplies destruction. A legacy public WoodenGateState name may re-export this exact type if compatibility requires it; it must not remain another struct/Vec owner.

Full original constructor140756370/3856B calls the full leaf constructor141210110/19893B. At141212b15..c9e the real DrawableMeshBundle+TransientMesh+WoodenGate spawn payload contains the owned top Vec at bundle80..103, full WallId at104, the two directions at112..127, seed128 and version132. Identity transform/world-space geometry and existing actual mesh/shader/TransientMesh path stay intact. The current constructor must stop inserting a second view component. Leaf geometry or its allocation algorithm is not rewritten for this identity fix.

## Actual render consumer, without cloning Vec

Full original env14 1409b57c0/1191B resolves the full component, then1409b58a2..a6 reads native wall+18 and seed+30. Existing bind sequence, GateColorIdSsbo, disk dependencies and signed32 uniform transports are unchanged.

The current two-field WoodenGateView declaration is referenced only by the actual constructor, env14_body/env14, and one scoped callback test. Replace it with a public re-export of canonical drawable::WoodenGate under the old view name, or remove the old name if root prefers; either option has exactly one ECS TypeId and no second active projection owner. env14 obtains a reference from comp, never `*comp`/Clone. env14_body only reads gate.wall.0 as i32 and gate.seed as i32. Existing pure render helper signature may retain the alias to avoid unrelated callback changes. Its focused test must instantiate the real Vec-owning component, borrow it, and verify the same two uniform values; it must not manufacture a Copy view entity. The historical test's full-reference comparison must be adapted without inventing a new Eq contract.

## Original gate region writer: exact data, still required by the next source batch

Full original1422cc390/2910B takes Query<&WoodenGate>. Gate top_positions length must exceed2. It iterates right then left endpoint, not every curve segment: names gate_right/gate_left, side -1/+1, first/last top point with explicit y+2 and original z+0. The current zero-direction skip has no original counterpart; native normalization propagates the original floating-point results.

Original key helper141cf7440/434B hashes the actual tuple(name,seed:u32,WallId:u64), string delimiter0xff included, with the existing zero-seeded DefaultHasher/Sip state. Reuse public Rust Hash/DefaultHasher; no private hash container/algorithm or generated ID is introduced.

The emitted region payload has source Id{Native3,native_u64_08:gate.wall.0,native_u32_10:gate.seed}; it is not the region's hash key. It stores endpoint axis Vec2 at+18, size(2,2) at+20, position Vec3 at+28, priority0 at+34, **mode0** at+38, weight-group gate.clutter_version at+39, candidate sentinelffff at+3a and initial epoch0 at+3c (state add_region supplies its actual epoch). This is weighted mode0, not the fixed Olden modes2/3/4. Original constants are bound, including size(2,2), offset(0,2),0.5 and1.

Do not call the native +28 position an already centered rectangle. Full remove-grid140ee4a70/378B reads axis,size,position and constructs the rectangle center as position.xz + rotate(axis)*(size*0.5), preserving observed expression order. The old current rectangle2d instead normalizes a basis, forces size1 and substitutes a zero-axis rectangle, which is contradicted. The original around-gate point computation includes that rotated half-size subtraction. Its148 continuous instructions are retained without claiming a reassociated simplified formula. A typed region migration must preserve this origin/axis/size distinction plus the source payload. Existing seven-float equality does not compare source ownership; that native selected-field behavior must remain separate from schema ownership.

The present AutoClutterRegion has only source tag and erased payload, approximate basis/center, and no actual source value. Merely replacing GateHost's Query or attaching mesh metadata to it would not complete this writer. The next region/state source batch must also carry the actual source value and preserve actual region insertion, dirty subscriptions, stale removal, candidate receipt and Arc lifetime. Other current region constructors require their own exact adaptation to that shared type, not default fields. This contract does not authorize invented zero metadata or changing the current populate entry into a pending surrogate.

## Actual rejection reader, same TypeId and existing geometry

Populate's derived DecoratorRejectionRes includes Query<&WoodenGate>; its full762B initializer and nested four-raster107B initializer were retained in rejection-param-v1 (ecf0880b). The same canonical type inserted by the actual constructor must be queried here and by around_gates. No GateHost or borrowed numeric pointer supplies the query.

The original common filter gate reader1422da945..dab1f takes the owned vector's first/last points; an empty vector follows native panic paths rather than a zero-sized accepted shape. It forms center as the ordered half-sum of endpoints plus(0,1,0), dimensions(length3D(first-last)-0.5,2,3), and horizontal axis(last.x-first.x,last.z-first.z)/sqrt(dx*dx+dz*dz). It calls the existing Rectangle3d constructor140c98380 then overlaps140c98580 against the candidate rectangle. No normalize_or_zero, epsilon clamp, endpoint direction replacement or default shape is added. Gate dimensions/constants are independently bound to direct PE bytes in gate-consumer-constants-B.json. The actual full44KB producer remains the parent body for that reader; it is not replaced by a standalone fake predicate.

This closes the schema/query ownership prerequisite only when the producer and actual readers use the same type. It does not by itself restore the complete common filter, weighted placement branches, dirty lifecycle, actual nonempty candidate output or App registration. Those remaining steps stay in the main chain and cannot be skipped because a synthetic test can build a carrier.

## Diagnostics and actual next source boundaries

The same rejection path submits DebugRectangle3d for gate rectangles with native color2 and width0.5; process submits every examined candidate with accepted0/rejected1 when DebugGui show_trigger_proxies and ClutterCollisions are selected. The current Rectangle type has no show. Its complete original36/38/39/38/2942B helpers are frozen in this package and have been transferred to01, root's unique TG67 debug owner. Existing UiSignifierStream TG-e514d1dbdfe157bc3e is unclaimed and its writes require &mut, unlike original shared Res with its internal queue. Root must select the minimal real write adaptation; no unrelated caller is converted to a successful no-op. Rectangle display is a real dependency, not suppressed or replaced with a different debug algorithm.

The immediate review request is one canonical Gate schema/actual constructor/render borrowing admission, plus the original region/rejection facts that constrain the following real producer implementation. Candidate changes to actual around_gates/state/populate must be judged as one original producer chain once their remaining shared type/dirty/weight/filter contracts are ready; arbitrary injected Carrier -> manager tests do not count as that chain. No four-source process migration is submitted here.

Focused source tests after admission: actual canonical component inserted from nonempty leaf metadata; actual World query sees the full owned data and full-width WallId/seed/version; legacy names are aliases of one TypeId; render uses the actual component with no Vec clone and unchanged uniform low32 behavior; removal releases the same Vec through normal ECS destruction. Original region computation/filter tests must additionally bind source3/mode0 and the original non-default geometry, but cannot substitute for the nonempty real writer. All tests in this design are not written or run.
