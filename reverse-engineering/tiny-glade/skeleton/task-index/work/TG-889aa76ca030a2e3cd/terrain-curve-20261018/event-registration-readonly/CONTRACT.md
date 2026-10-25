# Actual decorator event registration and maintenance (B static evidence)

No host source was edited. run-game-owner.patch is a minimal necessary candidate for its main-chain owner; apply event-v2 first. app.add_event supplies the dependency EventRegistry/Events maintenance and canonical event resource; merely init_resource Events would omit maintenance.

## Exact original chain

- DecoratorPlugin::build original0x140342c10/PDB17521B, not a guessed source name. At0x140342cc8 it calls EventRegistry::register_event<DecoratorOpEvent>@0x140584b50 after resource-presence checks. registration-typed-procedures.json retains the exact event type.
- @1403451af calls IntoScheduleConfigs::chain@1400d52f0. Its exact typed tuple has19 top-level members; member15(zero-based) is ResMut<DeferredDecoratorOpEvents>, EventWriter<DecoratorOpEvent>. chain-members.json is parsed with both angle and parenthesis nesting, retaining exact types.
- chain@1400d57e0 calls the exact into_configs@1402de9d0 for that two-parameter function. Its System vtable1429b3170 points to native run_unsafe1402dd980/run_without_applying_deferred14042e220, both directly call original unleash1408337c0 (already frozen69 proof). No same-address unrelated grow_one symbol was used for event identity.
- Immediately after chain returns, @1403451b4 writes byte0x19 (25), loads vtable14292af98, and @1403451d0 interns it. Raw vtable is size1/align1 with country_core::systems methods and typed SystemSet::system_type<GameSystemSet>. @1403451e2 applies in_set_inner; @140345253 adds systems. Thus current owner hunk attaches actual unleash to existing GameSystemSet25. It does not invent a new stage or reorder after replay40. Original replay40 queue production is naturally emitted on the next25 frame.

## Current owner reuse and boundary

run_game already initializes actual DecoratorIdGen, DeferredDecoratorOpEvents and DecoratorStorage together. external-wrapper alias was already integrated by root. The hunk adds the canonical event through app.add_event and one actual system in native set25; the single queue/drain implementation remains current owner's. Existing native19-member group is not reproduced wholesale, and the hunk does not claim the other decorator visual/input/collision consumers restored. Current cull_oob_decorators temporary local queue remains a separate known missing producer connection. Host integration/runtime acceptance belongs to root/main-chain owner and tests.

Evidence: original17521+4650+437B decoded packs verified against B PE file, exact PDB procedures, raw vtable/method identity and real direct-call references. Static only. No target execution/JIT, no Cargo, no production writes. Isolated owner hunk applies exactly to recorded current host base.
