# Canonical9 key / scheduler performance review

Actor: `/root/performance_team`. Scope: root-delegated read-only investigation of the canonical9 Nani maintenance closure, 2026-10-20. No production source, Cargo feature, frozen candidate, database, benchmark or GPU execution was changed. Only this report directory was written. Every source excerpt and existing build fingerprint is bound in `EVIDENCE.json`.

## Decision

Keep the current plain `u32` dense hint, `&mut InstanceKey` query adaptation and `bypass_change_detection`. Do not introduce `Cell`, `UnsafeCell`, an unsafe `Sync` implementation, atomics or `Without` filters to address the unproven claim that the nine mutable-key queries must serialize. The previous necessary-serialization wording was incorrect.

Two separate facts govern scheduling:

1. Current manifests request `bevy_ecs = 0.16.0` with default features disabled and `std`; no manifest in the production workspace requests `bevy_ecs/multi_threaded` and no source call to `set_executor_kind` was found. The directly enabled `bevy_tasks/multi_threaded` does not reverse-enable the ECS feature. Bevy `schedule/executor/mod.rs:52` selects the default `SingleThreaded` executor without the ECS feature; lines 59–60 select default `MultiThreaded` only with it. The report snapshots 42 existing `lib-bevy_ecs.json` cache fingerprints, all without `multi_threaded`. Those are historical cache evidence, not a newly built executable or proof of the executable root will next run. A root build occurring during review can create a further fingerprint, so the count is a bounded observation.
2. If the multi-threaded executor is selected, `multi_threaded.rs:509–517` compares `system.archetype_component_access()` against active accesses. `query/state.rs:633–659` registers only matching archetypes; `681–711` maps component access to each matching `ArchetypeComponentId`. Distinct actual single-marker archetypes therefore have distinct mutable-key access domains. The static `ComponentId` conflict is used by schedule ambiguity reporting (`schedule.rs:1353–1397`) and does not itself prove run-time serialization.

No wall time, speedup, parallel execution or CPU utilization was measured. The canonical9 source correctness status is unaffected by this performance correction.

## Original key and condenser: observed vs unknown

Observed retained B evidence:

- Original backend44 FunctionSystem generic spellings specify shared `&NaniActiveInstance` for the update queries (`../nani-maintain-20261019/backend44-registration-pdb.json`, also `payload-mapping-nine.json`). The tuples are unchained updates between maintain/deferred application and flush.
- The native carrier passed to the update path is a 12-byte record: stable lookup word at `+0`, dense-index hint at `+4`, subset discriminator read at `+8`. These are memory observations; they do not prove the native source field declarations or that the native type named `InstanceKey` alone occupies all 12 bytes. PDB also spells `Condex<country_core::render::nani::gpu_world::InstanceKey>`.
- In the complete original SolidVertexColor updater, `1420710b0–1420710ba` computes the row address using a 12-byte stride into the fetched component storage; `14207124e` calls `14156de90` with that address, rather than a new local key copy. The retained callee reads hint `param_2[1]`, checks the stable word against dense keys, hashes only on a stale/missing hint, then stores the repaired hint back at `+4` (`decomp-0x14156de90.txt:151–155,495–503`). Equivalent mesh-update evidence is retained at `decomp-0x140991d60.txt:99–103,289–297`. The report copies bounded caller/callee excerpts.
- Current core `utils/index_condenser.rs:49–57` faithfully passes the plain hint into a temporary shared-utils `Condex { key, cached_index: Cell::new(hint) }`, calls `get`, and copies the repaired hint back. Shared utils `index_condenser.rs:79–85` performs the dense fast path before hash fallback and repair. This temporary local `Cell` is an implementation adapter; it does not prove native component interior mutability.

Unknown: the original `NaniActiveInstance` / `Condex` field type, original synchronization primitive, original unsafe implementation and borrow-context proof. Retained TPI contains no `NaniActiveInstance`, `Condex`, `cached_index` or qualified native `InstanceKey` declaration; the PDB was built with Rust line-tables-only. Generic symbol spelling is useful evidence for shared Query ownership but is not a field-layout definition. An ordinary x86 store alone also cannot distinguish a relaxed atomic store from a non-atomic cell. `Component: Send + Sync` is required by current Bevy, so current shared-utils `Cell` cannot simply be embedded into this ECS key. No unsafe `Sync` conclusion is justified.

The required behavior is proven independently of that unknown type choice: preserve the stable-key fast path, repair a stale hint after swap removal, keep the repaired value for later frames, and avoid manufacturing `Changed<Key>` from a cache repair. Changing Query to shared while copying and dropping the repaired key would repeatedly hash stale handles and lose the cache behavior. Keeping the current mutable key is the narrow safe adaptation.

## Without-filter design assessment

For a valid entity with exactly one of the nine mapped markers, adding every other marker as `Without` is semantically redundant: the same rows are updated, the same subset is locked once, the same dirty ranges are written, and key repair remains intact. Full symmetric filters would contain 72 exclusions; a triangular priority set can prove pairwise disjointness with 36 exclusions, but neither is proposed for implementation here.

Bevy `Without<T>` adds no component read/write, is archetypal (`IS_ARCHETYPAL = true`) and has a per-row `filter_fetch` that returns true. Its cost is component registration, query/filter metadata and membership matching when archetypes are initialized or added, rather than eight marker fetches for every row. It would prove static pairwise disjointness and could avoid conservative access overlap from an existing multi-marker archetype, including an empty historical archetype retained by the World. It does not create parallel execution under the current default single-threaded executor, and valid single-marker archetypes already separate the actual run-time access domains when multi-threaded execution is selected.

The equivalence domain must not be overstated. Current preflight (`systems.rs.candidate:691–724`) scans actual descriptor-bearing entities, including those with existing keys, rejects unsupported kinds, multiple mapped markers, wrong key kind and missing required inputs before any allocation. The update queries themselves require no `NaniInstanceDesc`. Thus a manually constructed or otherwise malformed entity with a key and multiple mapped markers but no descriptor is outside the preflight guarantee and remains visible to multiple current update queries. Full `Without` filters would silently remove it from all update paths; triangular filters could silently give it a priority. Neither preserves the current failure/side-effect boundary.

Any future filter patch must first establish the invariant for the entire actual key-plus-marker update population, including descriptorless entities, preserve errors before subset mutation, and keep maintenance/deferred/flush ordering. Such an added host guard is a declared domain change, not evidence that the original used those filters. Other eleven unsupported payload contracts remain explicitly unsupported; no guard or filter may be used to hide them.

## Bounded next steps for the central owner

No change is needed in the canonical9 package for this investigation. Before any separate global ECS threading change, record the actual binary feature set and Schedule executor, and review the entire App's parallel resource/interior-mutability contracts. Enabling the ECS feature affects more systems than these nine and is outside this maintenance source owner.

If root later authorizes focused verification, a small structural check is enough before any benchmark: initialize the real nine systems with two valid single-marker archetypes and inspect their actual archetype access compatibility; separately show the current manifest default executor. For a future filter experiment, include descriptorless multiple-marker and empty historical multiple-marker archetypes, and prove errors are not silently converted into missing updates. The existing swap-remove stale-hint and unchanged-row rerun regressions remain the useful semantic checks. Do not run nine parallel Cargo or GPU processes, discard hints, remove guards, change counts or claim a speed multiplier.

## Coordination and validation boundary

04 (`scene_startup_team`) confirmed the preflight domain and original evidence paths; 07 (`validation_integration_team`) independently read the cached Bevy source and accepted the static-versus-runtime access correction. Root adopted the no-filter/no-key-change decision. This is a source/evidence review only: no new native oracle, compiled-candidate result, runtime scheduling result, benchmark or GPU acceptance is claimed.
