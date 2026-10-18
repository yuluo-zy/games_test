# Borrow the authoritative RaycastWorld for the recovered terrain sphere sweep

This is a necessary current-architecture API adapter, targeting B (not the task index's default C). Existing file-family TG-f7070bb3446b7a37eb remains exclusively claimed by building-audit-20261018. No production source/Cargo/registry or native database is changed by this candidate.

The camera/walk caller already receives the actual ECS RaycastWorld and TerrainHeightsData references. It must not obtain a second Raycasting Resource containing another owned world. The new native-physics method is:

```rust
RaycastWorld::spherecast_w_terrain(
    &self,
    terrain: &TerrainHeightsData<TerrainHeightSsbo>,
    origin: glam::Vec3,
    dir: glam::Vec3,
    radius: f32,
    options: rapier3d::parry::query::ShapeCastOptions,
    mask: u32,
) -> RaycastWorldResult
```

The sole recovered terrain-sweep algorithm moves from the existing Raycasting::spherecast_with_options body into this borrowing method. Existing Raycasting::spherecast_with_options retains its public signature and forwards every parameter to self.world.spherecast_w_terrain. Both existing no-options spherecast APIs keep using the dependency's default options. RaycastWorld::spherecast_with_options remains the single collider-only core delegate to Rapier::spherecast. No query world, terrain data, mesh state, Event or Resource is cloned or mirrored. There is one terrain merge body in the final file. The no-native-physics API is not expanded with a fabricated stub.

B original proof is reused from the previously recovered and integrated v2 sphere-options chain; paths/hashes are in original-proof-reuse.json. No new mining/JIT is needed for a receiver-only extraction. Native Raycasting::spherecast 0x1408cc390 forwards the full Options pointer/radius/mask, queries terrain with max_time_of_impact+radius, checks sphere TOI against the original maximum, and retains a collider only when its TOI is strictly nearer (terrain wins ties). Existing original-order distance/SSE fallback helper stays unchanged. The borrowed method is a current Rust API; it is not claimed to be an original RaycastWorld method or ABI.

The reviewed author patch e32f85cdab51317195a3d0df51858c0d87dfaf7af38bfaa287d5ba66939ea333 applies exactly to current before-source 1f99d3f24f738a55fa41c0048a7be1228ef626060cf41cf245dc80c99fe1ec89 and produces the author's declared candidate69ad767d35506f7de68aef39ca9d4940e369941cf1438b36331b1f219c1b4c15. Our candidate has only a doc clarification distinguishing the adapter from native API; its game code is identical. The old algorithm body and the extracted body compare exactly after whitespace and self.world receiver projection. All existing tests remain byte-for-byte unchanged. Isolated patch application and rustfmt pass; no Cargo or native/JIT execution was performed. A first verification attempt used an absent cfg(test) spelling and failed before writing acceptance; the corrected check uses this file's actual cfg(all(test, feature="native-physics")) boundary.

The existing native-physics sphere tests cover limits/target distance/subworld selection/terrain and original scalar fixtures. Root will compile those and the actual camera consumer together. This adapter review does not accept camera movement/sliding, global scheduling or the full collision module. The actual camera-owned candidate should call this method on its existing borrowed world; no additional upper-level edit belongs in this file-family patch.
