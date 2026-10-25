# R5A bounded unknowns

- Full `record_batch` command creation, pass command emission, profiling/timing callbacks, and cleanup were not executed.
- The export gate was entered with an injected valid frame; the preceding command loop is not an original-entry oracle.
- Population original entry and its partial-error paths run end-to-end against stubbed device creation/pool/hash-map/drop boundaries; these boundary implementations and unwinding remain unvalidated.
- The exact producer of marker `+2 = 1` is not established. The negative search is limited to the population body, its direct workgraph callers, and the selected registry/resource references.
- The Bevy 0.16 fork and private Vulkan synchronization implementation remain unknown.
