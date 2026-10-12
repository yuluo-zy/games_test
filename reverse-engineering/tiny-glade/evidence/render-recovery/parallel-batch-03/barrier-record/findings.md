# R5A barrier/record findings

Target is the original `tiny-glade.exe` (`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`). The oracle is scoped to original instructions, with only public/device and synchronization boundaries replaced by deterministic fixtures.

## Observed

- `CompiledRenderGraph::add_pre_pass_barriers` (`0x1402c3d00`) skips a no-predecessor resource when `reused=false`; when `reused=true` it requires `initial==0` and emits `0 -> current`.
- A predecessor uses the original `can_be_concurrent_with` (`0x140bece70`); a concurrent access emits no barrier. Otherwise the predecessor code is the source state.
- `record_batch` (`0x1402c1d40`) copies `pending.len` to a local and stores zero at `compiled+0x210` before the first pending item is resolved. It skips `reused=true`, then invokes the original resource lookup and emits the initial transition. A failure after one successful item leaves the pending length zero and preserves the first transition.
- The final record/export branch requires the batch final flag. Export uses the last access when present, otherwise the resource initial state; it then emits through the same synchronization scope and resets it.
- Population writes the physical slot index at `0x140be8789` and explicitly writes reuse marker byte `+2 = 0` at `0x140be878e` on the ordinary append path. The origin is tagged as `3` before acquisition (`0x140be7f21`); release events are skipped at `0x140be7eb7`.
- The complete original population entry runs to its return with preallocated vectors and fixture hash-map/pool/device/drop boundaries: 97 cases cover owned/unowned imports, new image/buffer, zero usage, pool hit/miss, release skipping, and error tags 0..10. Tags 0..8 return failure after the current origin is consumed, retain the original graph origin length, and preserve earlier physical slots. Tags 9..10 reach the result-tag panic. Pool hits explicitly produce `reused=false`.
- Two of these are explicitly synthetic local-map `Some(physical=0)` fixtures: original instructions at `0x140be8001/0x140be80e6` mark the logical resource reused, set its initial access to zero, and append no backing slot. They prove consumers only, not an upstream producer or natural reachability.
- Image export aspect lookup uses export declaration format (`+0x10`, fixture 91), whereas initial/pre barriers use the managed image record format (`+0x18`, fixture 37). Aspect mapping itself is a stub.

## Inferred / not claimed

The injected post-record frame validates only the export basic block; pass recording and command construction are not executed. Vulkan handles, real device generation, tracker internals, allocator behavior, and actual hash-map/pool/device/drop implementations remain outside this oracle.
