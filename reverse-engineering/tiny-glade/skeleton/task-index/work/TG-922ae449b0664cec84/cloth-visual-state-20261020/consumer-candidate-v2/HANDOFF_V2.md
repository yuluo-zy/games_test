# Test-only v2 delta

V1 remains immutable. Independent source review requested this stronger assertion in the existing same-World actual-producer/Named-atlas test: read the actual maintained cull row mesh_index from the same GpuWorld owned subset and compare it to the resolved nonzero expected atlas part. This fixture performs one fresh maintain pass and no deletion/swap, so key.index is a valid dense hint in this bounded test. No claim is made that this hint stays valid after compaction. No new production API/count, extra synthetic owner or business change is introduced.

Production bytes before #[cfg(test)] are mechanically identical to V1. The other flag_connections source remains the exact V1 selection. Twelve new and ten retained tests remain unrun; rustfmt parses this new candidate, not typecheck. Root alone applies/compiles. Upstream actual window inputs, real loader/host order and GPU/C remain independent boundaries from V1.
