# Conditional RoofFloor adapter — design only

This proposal targets the already implemented **Roof-bearing producer subdomain** of native kind9. It is not a full recovery of the original Roof/Floor optional union and does not change any registration or source. Review may reject this restricted domain or require canonical Floor first. The default App is not proven to call `add_roof_systems` in the currently visible checkout; pending user source may add that edge.

## Exact original and current inputs

Original set44 update index5 is `Query<(&NaniActiveInstance, &Transform, Option<&Roof>, Option<&RoofAnimation>, Option<&Floor>), With<InstanceDataMarker<StoneFloorInstanceData>>>`, followed only by `Res<GpuWorld>`. This updater has no PublicWalls/resource lookup. Roof wins when both optional shape sources exist. With neither it panics. Missing animation yields the original scalar1.0.

Fresh identity-bound B native evidence:

* `14207909b..1420790b7`: Roof row stride0x58, optional presence gate.
* `1420790bb..1420790ce`: Floor row stride0x28, optional presence gate. This corrects the old constructor comment's approximate larger payload estimate; no complete Rust Floor declaration is inferred from stride alone.
* `1420790df..1420790f0`: animation row stride8 and read at **offset0**, not the second animation channel. Default branch begins `142078dc0` with xmm15 copied from the original1.0 constant.
* `142078dd0..142078e6d`, `142079145..142079178`: real Roof fields drive the existing payload builder.
* `142079191..1420791bf`: shape circle starts at component+0xc, native `Circle2d::get_tangent_at_u` is called with u=0. Its complete139B body reads rotation at circle+0xc and computes the signed tangent.

Current `Roof::observed() -> &[u8;88]` provides the same actual owned game-field bytes; `RoofAnimation::values() -> [f32;2]` exposes offset0. Current shared `::utils::geometry::circle::Circle2d` is real and implements `new_rotated` and `get_tangent_at_u`. **The old Nani comments saying this dependency is a stub are obsolete.** `instance_updates::circle_tangent_u0` itself still returns Vec2::ZERO; a new adapter must not use that placeholder.

## Proposed narrow interface, requiring 07 review

1. Reuse the actual existing `Roof`, `RoofAnimation`, `Transform`, canonical `InstanceKey`, `InstanceDataMarker<StoneFloorInstanceData>` and unique GpuWorld. Add no proxy component/resource, fake Floor, payload default, separate key, or new atlas lookup.
2. Add a specifically named Roof-bearing ECS updater, e.g. `nani_update_roof_floor_for_roofs_ecs`, with `Query<(&mut InstanceKey, &Transform, &Roof, Option<&RoofAnimation>), With<InstanceDataMarker<StoneFloorInstanceData>>>` and `Res<GpuWorld>`. This is intentionally narrower than the original optional-union Query. Do not call this a fully restored stone-floor ECS contract.
3. Extend maintain preflight **only for actual kind9 descriptors with the StoneFloor marker and a real Roof**; preserve the existing one-marker and Transform requirements for this specific original Query domain. A kind9 descriptor without Roof still fails before key/count publication. Do not open kind9 generally and then silently filter Floor-only rows in the updater. No registration-wide rejection of empty kind9 libraries is introduced.
4. Project the existing88B Roof to22 little-endian u32 words with safe byte reads; pass `Some(actual_roof_words)`, `animation.map(|a| a.values()[0])`, and `None` for Floor into the already recovered `build_stone_floor`. None here describes the restricted actual Roof-present branch; it is not fabricated Floor contents.
5. The tangent closure decodes the four actual circle words `(center.x, center.y, radius, rotation)` from the supplied shape slice into `::utils::geometry::circle::Circle2d::new_rotated`, then calls `.get_tangent_at_u(0.0)`. Avoid `circle_tangent_u0`, zero directions, temporary raw component mirrors or unsafe ABI casts. The original tangent only depends on rotation; passing actual center/radius retains the canonical geometry interface.
6. Write the returned real StoneFloor payload, position and sort key through the existing kind9 owned subset, dense-key hint and dirty-range API. Put this updater at the original index5 position relative to the mapped unordered updates. Retain maintain→explicit ApplyDeferred→updates→one flush. Do not install the roof business plugin or touch VAO prefix scheduling as part of this adapter.

This would support **nine complete adapter contracts plus a restricted Roof-bearing branch of kind9**. It must not be reported as ten fully recovered original kinds; original Floor-only entities remain an explicit unresolved domain. Generic hasTransform validation must not be copied to CurvedHalfTimber, Plaster or StairsFloor, whose original queries do not read Transform.

## Actual writer edges and conditional first break

`system_roof::create_roof::create_roof` constructs Roof, RoofAnimation, identity Transform and a true NaniBundle<StoneFloorInstanceData> with AtlasPart0 / RoofFloor. `switch_roof_type` inserts the same canonical bundle on both type branches. `generate_roof_stone_floor_and_roof_bottom` mutates the real descriptor's atlas part. Their current owner registers them within `add_roof_systems`, but the visible App has no caller of that installer.

If a legitimate installed roof creation/switch chain publishes such an entity, the currently integrated canonical9 preflight is the first Nani-chain rejection, before allocation. Removing the check would leave that real entity with an allocated key and no current ECS payload updater; it is not a valid repair. The adapter described above addresses that one proven producer's input shape without pretending the host has already called it.

## Validation needed before acceptance

Root must own typecheck/test execution and production integration after current rebase and design review. Bounded CPU tests should use actual nonzero Roof inputs and the same World's actual creation/Commands bundle, key allocation, explicit barrier, payload update and existing flush; cover animated channel0, absent-animation1.0, straight and rotated-circle roofs, nontrivial translation/style data, no-key publication on a Floor-only/unsupported descriptor, and real zero count for an empty registered library. No zero or maximal-size surrogate buffers count as payload/count proof.

A true host registration change is separate. It needs the pending user source reconciled and the original roof resource/event/install order audited. No whole-roof plugin registration is authorized or proposed in this document.
