# Session-owned typed named resources

Public GPU resource ownership now stays in `VulkanGraphSession`. `named_texture`, `named_buffer` and `temporal_texture` return graph-scoped typed IDs and explicit validity; `_with_sharing` variants preserve the actual Vulkan creation policy. `reset_temporal(name)` invalidates history and parity without destroying its allocation. `PassContext::history_valid` and `buffer_history_valid` validate graph/type before reading per-frame metadata; that metadata is excluded from cached plans.

The session automatically validates initialization, publishes only complete named outputs, and commits parity only when the selected current member is exported by successful submission. Abandoned and partially submitted frames retain accurate allocation/layout ownership while invalidating contents as appropriate. Descriptor/policy replacement preserves old owners on allocation failure, including failure of the second member of a pair. Retired named allocations leave backend tables only once no pending session frame refers to them.

The public raw token/owner/allocation-callback temporal API and separate temporal begin/execute entry points were removed. The unused `rg::state` alternate compile/owner path was deleted. S04, tiny-glade's real GraphRgResources, and integration fixtures use the typed session path.

## Validation

- `cargo check -p rhapsody --all-features --tests -j 2`: PASS (`/private/tmp/rhapsody-v9-named-temporal-check3.log`).
- CPU temporal/error protocol: 13/13 PASS (`/private/tmp/rhapsody-v9-named-temporal-cpu.log`).
- Strict actual Vulkan/MoltenVK: 20/20 PASS across import_contract (2), multi_queue_graph (4), rg_fixes_vulkan (10), token_space (2), triangle_animation_graph (2).
- Validation layer and synchronization validation enabled; zero Validation Error/VUID/SYNC-HAZARD; no SKIP.
- Rotating triangle: 48 frames, zero pixel mismatch, 47 successful history checks, zero pending command/timeline state at shutdown.

`binary-results.json` binds each actual executable by SHA-256. Binaries were copied before later managed-store work. `owned-source-sha256.json` identifies this closure's source subset; it is not a claim that unrelated work in the shared repository was frozen. tiny-glade all-target consumer validation and final Window Lab/whole-module acceptance are performed by the integrating parent agent and are not counted here.

Follow-up validated: failed named requests restore their specification-table entry, so repeated failed new names do not consume the fixed capacity. The regression issues 1025 distinct invalid requests. The newer `rg_fixes_vulkan` binary passed all 10 tests with validation and synchronization validation enabled, zero errors and no skips; its SHA-256 and log are recorded in `../2026-10-17-v9-metadata-budget/result.json`. The original 20-test manifest above remains the identity of its original run.
