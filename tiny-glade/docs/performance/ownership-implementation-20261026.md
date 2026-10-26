# Ownership changes and validation (2026-10-26)

User directed source work to continue while the production-window compatibility blocker is deferred.

- VAO publication moves the previous table out of the assets mutex before releasing its owners.
- Dynamic constant allocation ends its allocator guard before owned chunk writes and Arc construction; allocation failures retain the original error path.
- Explicit texture views are grouped by the full texture identity. Release removes the group and drops native owners outside all host-table locks; other textures and external Arc owners survive.

Read-only follow-up review by the previously requested ownership audit agent found no new lifetime blocker. Grouping adds a per-texture table and another lookup; overall performance is not yet established. Concurrent view creation versus release remains a pre-existing open race.

Validation: strict Vulkan texture-release unit tests 2/2; VAO scope 1/1; collected-vs-inline constants/depth pixel comparison 1/1, without validation errors in those targeted runs. An earlier core_raster_draw run returned 12 successes but emitted VUID-vkDestroyImage-image-01000 during synthetic DeviceLost injection; that run is invalid as a validation-layer gate. The fixture now waits for its real setup upload before simulating DeviceLost. A subsequent strict run passed core_raster_draw 12/12, original_raster_gpu 2/2 and vao_scope 1/1 without validation errors; assertions were preserved. Latest rerun evidence is in .perf/candidate-raster-gpu-tests.log.

No Windows four-scene gain, <=16ms acceptance or macOS regression pass is claimed. Remaining ownership audit candidates require targeted evidence and separate changes.

Surface worker-owned trace migration is documented in collect-migration-20261026.md; reserved native retirement and lock-free-of-destruction ownership transfer are documented in retirement-reservations-20261026.md. These are correctness-validated source candidates, with overall performance pending.
