# Prefab adapter independent review — B

PASS for the bounded placement adapter. The 13 frozen input hashes match. The original hot-reload range at `0x140843740` (3948 bytes) and GPU-prefab callee at `0x140841b00` (602 bytes) match the identity-bound B image. An independent rerun of 64 resolved-row forwards and 64 callee executions matches every recorded output bit.

The hot-reload caller resolves a query row with a 48-byte stride and passes its placement pointer unchanged as the fifth argument. The callee reads position and scale from that record for culling and forwards the full placement record to the existing GPU API. The candidate therefore corrects a real information loss: it accepts the existing `PlacementDataSsbo`, derives the existing `PrefabTransform` locally for the retained RT/audio code, and forwards the complete placement payload. It does not replace the current prefab architecture.

The candidate regression checks all 48 GPU payload bytes, actual cull position/radius, and the retained RT translation path. No blocking finding was identified. Rust compilation remains the integrator's responsibility.

This review does not accept the full native query/filter, atlas/name lookup, half conversion, GPU execution, or production host lifecycle. The instruction probe begins after row resolution and observes the half-converter input. The current workspace has no production host caller of this hot-reload helper. These boundaries are preserved in `prefab-independent-review.json`.
