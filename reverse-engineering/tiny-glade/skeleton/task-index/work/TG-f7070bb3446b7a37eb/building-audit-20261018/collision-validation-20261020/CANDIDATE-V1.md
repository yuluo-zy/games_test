# Candidate v1 — source review pending

Design passed scoped independent review `COLLISION_QUERY_DESIGN_REVIEW_V1.json`, SHA prefix6758627d, as reported by07/root. This candidate changes only world.rs decorator-overlap method and its focused real Rapier tests. It is bound to before e06d44296c6b764221ce81bbedf5ef77fa66e25323a1d9e60a3b914657252a1c. Root alone may apply/build/run. No source/Git/Cargo/GPU mutation was performed in production by the author.

The five tests cover actual collider insertion and pipeline flush, filtering/self/wall/destination/subworld policy, canonical borrowed source identity, unsigned ordered IDs, transformed query geometry, removal/reuse, and explicit invariant faults through the real predicate. They do not inject query results. Valid live user-data aliasing to another live receipt is not claimed to be distinguished beyond the original receipt contract. There are no runtime results yet.

Suggested root invocation after source review: cargo test -p country-core --lib --features native-physics decorator_overlap_regressions --locked --offline. Root should also retain the existing physics/query regressions; unrelated App compile gaps remain outside this package.01's incoming validation/cull consumer uses this Option/source API in a separate package. All current actual workspace source/test calls to the old method are absent (frozen caller audit), so no legacy mask compatibility shim is introduced.

A separately authorized bounded intersects_any/max_toi investigation will use another subdirectory/package. It must not replace or modify this frozen overlap candidate or manifest.
