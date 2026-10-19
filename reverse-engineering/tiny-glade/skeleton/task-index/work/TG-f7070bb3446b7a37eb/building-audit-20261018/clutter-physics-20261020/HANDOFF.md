# Clutter physics continuation authorization

On 2026-10-20 root explicitly delegated this bounded continuation of historical claim TG-f7070bb3446b7a37eb, owner label building-audit-20261018, to world_graph_team after checking no other active owner thread. The original claim lock, original work artifacts, and earlier deliveries are retained unchanged. All new work belongs only to this `clutter-physics-20261020` subdirectory. Root remains the sole production writer and Cargo/GPU validator.

The authorized scope is canonical cuboid local pose construction and its actual transformed-shape consumers, together with the same clutter upsert's mesh/source receipt and lifecycle dependency. Dimensions are already correctly multiplied by 0.5 inside the constructor; this is not a request to halve them again. No Compound wrapper, second physics registry/world, manual physics algorithm, broad collision rewrite, or protected critter source edit is authorized by this continuation.

Production input is frozen as `before/world.rs`, SHA `8161e74e7b74bc3d128187c872a626505e8e68e32212f026843597004bb86ffe`. The retained B native/source dependency review is in the authorized helper sibling TG-652d8e2c9cc79a01e3/scene-clutter-producer-20261020/physics-review-20261020. Native evidence here extends the actual constructor/adapter contracts; it does not assert C equivalence.

Design must be independently reviewed by validation_integration_team before candidate implementation. No candidate or tests have been run at this design stage.
