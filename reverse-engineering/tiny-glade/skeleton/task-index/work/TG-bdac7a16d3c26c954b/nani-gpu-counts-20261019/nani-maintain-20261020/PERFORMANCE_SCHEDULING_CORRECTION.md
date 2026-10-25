# Scheduling provenance correction

Historical frozen design/hand-off prose said mutable InstanceKey queries conservatively serialize the nine updates. That statement is incorrect for the locked Bevy0.16 executor and is superseded here. No code SHA or already reviewed design SHA was changed.

Primary local source: bevy_ecs0.16.0 schedule/executor/multi_threaded.rs509..517 updates each system archetype_component_access and checks compatibility with active_access. query/state.rs618..639 registers access only for matching archetypes. Thus With-only queries on distinct single-marker archetypes can run concurrently despite sharing the InstanceKey ComponentId. Static ambiguity checks are different from this runtime execution gate. A malformed multi-marker archetype can still conflict; no speed/timing claim is inferred.

The current candidate's descriptor preflight validates actual NaniInstanceDesc entities, including existing keys, and requires exactly one implemented marker with necessary input components. Update queries do not require Desc, so that invariant must not be generalized to every key-bearing entity in the World. Adding Without filters merely to silence static conflicts could silently omit such entities and is not justified. No Without filters, mirror type, atomic-key rewrite or key ABI change was made. Plain cache-hint writes still bypass change detection; one lock per actual owned subset and actual dirty-range flush remain.

This correction is source-mechanism evidence only, not a measured performance result. The separate performance owner retains its bounded audit in performance-review/.
